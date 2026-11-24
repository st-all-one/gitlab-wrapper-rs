//! Handler de requisições HTTP arbitrárias (`raw`).
//!
//! A biblioteca não expõe um método HTTP genérico público, então este módulo
//! usa `reqwest` diretamente com a configuração já resolvida (token, base URL,
//! auth method, sudo, timeout), sem alterar a biblioteca.

use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};
use serde_json::Value;

use crate::args::RawArgs;
use crate::commands::projects::dry_run_value;
use crate::commands::{Context, OperationParams, dry_run, emit};
use crate::error::CliError;
use crate::input;

/// Executa uma requisição raw.
pub async fn run(context: &Context, args: RawArgs) -> Result<(), CliError> {
    let method = normalize_method(&args.method)?;
    let path = normalize_path(&args.path);
    let query = match args.query.as_deref() {
        Some(raw) => input::object_to_query(&input::parse_object(&input::resolve_value(raw)?)?),
        None => Vec::new(),
    };
    let body = match args.body.as_deref() {
        Some(raw) if raw.trim().is_empty() => {
            return Err(CliError::usage(
                "--body não pode ser vazio; use um JSON como '{\"name\":\"demo\"}' ou @- com JSON no stdin",
            ));
        }
        Some(raw) => Some(input::parse_body(raw)?),
        None => None,
    };

    if context.dry_run {
        return dry_run(
            "raw",
            &method,
            &path,
            &context.options.base_url,
            query,
            body,
        );
    }

    let result = request(context, &method, &path, &query, body.as_ref()).await?;
    emit(context, &result)
}

/// Executa uma requisição raw em batch, retornando o valor.
pub async fn batch(context: &Context, params: &OperationParams) -> Result<Value, CliError> {
    let method = params
        .named
        .get("method")
        .or_else(|| params.named.get("verb"))
        .and_then(Value::as_str)
        .map(str::to_owned)
        .or_else(|| params.args.first().cloned())
        .ok_or_else(|| CliError::usage("raw batch requires argument 'method'"))?;
    let method = normalize_method(&method)?;
    let path = params
        .named
        .get("path")
        .and_then(Value::as_str)
        .map(str::to_owned)
        .or_else(|| params.args.get(1).cloned())
        .ok_or_else(|| CliError::usage("raw batch requires argument 'path'"))?;
    let path = normalize_path(&path);
    let query = input::object_to_query(&params.query);
    let body = params.named.get("body").cloned();

    if context.dry_run {
        return Ok(dry_run_value(
            "raw",
            &method,
            &path,
            &context.options.base_url,
            query,
            body,
        ));
    }
    request(context, &method, &path, &query, body.as_ref()).await
}

/// Executa a requisição HTTP.
async fn request(
    context: &Context,
    method: &str,
    path: &str,
    query: &[(String, String)],
    body: Option<&Value>,
) -> Result<Value, CliError> {
    let url = build_url(&context.options.base_url, path, query);
    let headers = build_headers(context)?;
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(context.options.timeout_secs))
        .build()
        .map_err(|error| CliError::usage(format!("failed to build HTTP client: {error}")))?;

    let mut builder = client
        .request(method_to_verb(method), &url)
        .headers(headers);
    if let Some(body) = body {
        builder = builder.json(body);
    }
    tracing::debug!(target: "glw::raw", method = %method, path = %path, "sending raw request");
    let response = builder
        .send()
        .await
        .map_err(|error| CliError::api(&gitlab_wrapper::GitLabError::from(error)))?;
    let status = response.status();
    let text = response
        .text()
        .await
        .map_err(|error| CliError::api(&gitlab_wrapper::GitLabError::from(error)))?;
    if !status.is_success() && status != reqwest::StatusCode::NOT_MODIFIED {
        return Err(CliError::api(&gitlab_wrapper::GitLabError::api(
            gitlab_wrapper::ErrorCategory::from_status(status.as_u16())
                .unwrap_or(gitlab_wrapper::ErrorCategory::InternalError),
            status.as_u16(),
            summarize_body(&text),
            gitlab_wrapper::ErrorContext::default(),
        )));
    }
    if text.is_empty() {
        return Ok(Value::Null);
    }
    serde_json::from_str(&text)
        .map_err(|error| CliError::usage(format!("invalid JSON response: {error}")))
}

/// Constrói a URL completa `base/api/v4/path?query`.
fn build_url(base_url: &str, path: &str, query: &[(String, String)]) -> String {
    let mut url = format!("{base_url}/api/v4/{path}");
    if !query.is_empty() {
        let encoded: Vec<String> = query
            .iter()
            .filter(|(_, value)| !value.is_empty())
            .map(|(key, value)| {
                format!(
                    "{}={}",
                    url::form_urlencoded::byte_serialize(key.as_bytes()).collect::<String>(),
                    url::form_urlencoded::byte_serialize(value.as_bytes()).collect::<String>()
                )
            })
            .collect();
        if !encoded.is_empty() {
            url.push('?');
            url.push_str(&encoded.join("&"));
        }
    }
    url
}

/// Constrói os cabeçalhos de autenticação (nunca inclui o token em logs).
fn build_headers(context: &Context) -> Result<HeaderMap, CliError> {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    if let Some(token) = &context.options.token {
        let value = match context.options.auth_method {
            gitlab_wrapper::AuthMethod::Bearer => format!("Bearer {token}"),
            gitlab_wrapper::AuthMethod::Header => token.clone(),
        };
        let header = match context.options.auth_method {
            gitlab_wrapper::AuthMethod::Bearer => AUTHORIZATION,
            gitlab_wrapper::AuthMethod::Header => {
                reqwest::header::HeaderName::from_static("private-token")
            }
        };
        let header_value = HeaderValue::from_str(&value)
            .map_err(|error| CliError::usage(format!("invalid token header: {error}")))?;
        headers.insert(header, header_value);
    }
    if let Some(sudo) = &context.options.sudo {
        let value = HeaderValue::from_str(sudo)
            .map_err(|error| CliError::usage(format!("invalid sudo header: {error}")))?;
        headers.insert("SUDO", value);
    }
    Ok(headers)
}

/// Normaliza o método HTTP informado.
fn normalize_method(method: &str) -> Result<String, CliError> {
    let method = method.trim().to_ascii_uppercase();
    let method = match method.as_str() {
        "PATCH" => "PUT".to_owned(),
        "GET" | "POST" | "PUT" | "DELETE" | "HEAD" => method,
        other => {
            return Err(CliError::usage(format!(
                "unsupported HTTP method '{other}' (use GET, POST, PUT or DELETE)"
            )));
        }
    };
    Ok(method)
}

/// Converte o método em `reqwest::Method`.
fn method_to_verb(method: &str) -> reqwest::Method {
    match method {
        "POST" => reqwest::Method::POST,
        "PUT" => reqwest::Method::PUT,
        "DELETE" => reqwest::Method::DELETE,
        "HEAD" => reqwest::Method::HEAD,
        _ => reqwest::Method::GET,
    }
}

/// Remove o prefixo `/api/v4/` se o usuário já o incluiu.
pub fn normalize_path(path: &str) -> String {
    let path = path.trim();
    let path = path.strip_prefix('/').unwrap_or(path);
    let path = path.strip_prefix("api/v4/").unwrap_or(path);
    path.trim_end_matches('/').to_owned()
}

/// Resume o corpo de erro sem registrar dados sensíveis em excesso.
fn summarize_body(body: &str) -> String {
    const LIMIT: usize = 512;
    if body.chars().count() <= LIMIT {
        return body.to_owned();
    }
    body.chars().take(LIMIT).collect::<String>() + "…"
}
