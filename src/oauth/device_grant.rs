//! Fluxo Device Authorization Grant (OAuth 2.0) para GitLab.
//!
//! Implementa o fluxo de autorização para dispositivos sem navegador,
//! conforme a RFC 8628. Permite solicitar autorização, consultar
//! o status e obter o token final.

use std::sync::LazyLock;
use std::time::Duration;

use reqwest::blocking::Client;

use crate::core::errors::{ErrorCategory, ErrorContext, GitLabError};
use crate::types::{DeviceAuthResponse, OAuthTokenResponse};

// Cliente HTTP dedicado — NÃO passa pelo rate limiter do `HttpClient`.
// Isso é intencional: fluxos OAuth são chamadas esporádicas (não em loops).
static OAUTH_CLIENT: LazyLock<Client> = LazyLock::new(|| {
    Client::new()
});

/// Opções para solicitar autorização de dispositivo.
pub struct DeviceAuthOptions {
    /// URL base da instância do GitLab (ex.: `https://gitlab.com`).
    pub base_url: String,
    /// ID do cliente OAuth registrado.
    pub client_id: String,
    /// Escopos solicitados (opcional, separados por espaço).
    pub scope: Option<String>,
}

/// Opções para consultar (poll) o token após a autorização do dispositivo.
pub struct PollTokenOptions {
    /// URL base da instância do GitLab.
    pub base_url: String,
    /// ID do cliente OAuth registrado.
    pub client_id: String,
    /// Código do dispositivo recebido na resposta de autorização.
    pub device_code: String,
    /// Tipo do grant (opcional; padrão: `urn:ietf:params:oauth:grant-type:device_code`).
    pub grant_type: Option<String>,
}

/// Opções para obter um token completo via fluxo de dispositivo.
pub struct GetTokenOptions {
    /// URL base da instância do GitLab.
    pub base_url: String,
    /// ID do cliente OAuth registrado.
    pub client_id: String,
    /// Escopos solicitados (opcional, separados por espaço).
    pub scope: Option<String>,
}

/// Solicita autorização de dispositivo ao GitLab.
///
/// Envia uma requisição POST para `<base>/oauth/authorize_device` com
/// `client_id` e, opcionalmente, `scope`. Retorna os dados necessários
/// para o usuário autorizar o aplicativo em um navegador.
///
/// ## Params
/// - `options`: Opções de configuração para a solicitação.
///
/// ## Returns
/// `Result<DeviceAuthResponse, GitLabError>` — resposta contendo `device_code`,
/// `user_code`, `verification_uri`, `verification_uri_complete`, `interval`
/// e `expires_in`.
///
/// ## Errors
/// Retorna `GitLabError` em caso de falha de rede, erro de parse da resposta
/// ou erro de autorização retornado pelo GitLab.
pub fn request_device_authorization(
    options: &DeviceAuthOptions,
) -> Result<DeviceAuthResponse, GitLabError> {
    let base = options.base_url.trim_end_matches('/');
    let url = format!("{}/oauth/authorize_device", base);

    let mut form = vec![("client_id".to_string(), options.client_id.clone())];
    if let Some(ref scope) = options.scope {
        form.push(("scope".to_string(), scope.clone()));
    }

    let resp = OAUTH_CLIENT.post(&url).form(&form).send().map_err(|e| {
        GitLabError::api(
            ErrorCategory::NetworkError,
            503,
            format!("Device authorization request failed: {e}"),
            ErrorContext {
                operation: Some("oauth.request_device_authorization".into()),
                ..Default::default()
            },
        )
    })?;

    let status = resp.status();
    if status.is_success() {
        resp.json().map_err(|e| {
            GitLabError::api(
                ErrorCategory::ParseError,
                500,
                format!("Failed to parse device auth response: {e}"),
                ErrorContext::default(),
            )
        })
    } else {
        let body = resp.text().unwrap_or_default();
        Err(GitLabError::api(
            ErrorCategory::AuthorizationDenied,
            status.as_u16(),
            body.clone(),
            ErrorContext {
                operation: Some("oauth.request_device_authorization".into()),
                http_status: Some(status.as_u16()),
                response_body: Some(body),
                ..Default::default()
            },
        ))
    }
}

/// Consulta o token de acesso após a autorização do dispositivo.
///
/// Envia uma requisição POST para `<base>/oauth/token` com `client_id`,
/// `device_code` e `grant_type`. Deve ser chamada repetidamente em
/// intervalo definido até que o usuário autorize ou o tempo expire.
///
/// ## Params
/// - `options`: Opções de configuração para a consulta.
///
/// ## Returns
/// `Result<OAuthTokenResponse, GitLabError>` — resposta contendo o token de acesso
/// em caso de sucesso.
///
/// ## Errors
/// Retorna `GitLabError` com `AuthorizationDenied` se a autorização ainda estiver
/// pendente ou foi negada, ou erro de rede/parse.
pub fn poll_for_token(options: &PollTokenOptions) -> Result<OAuthTokenResponse, GitLabError> {
    let base = options.base_url.trim_end_matches('/');
    let url = format!("{}/oauth/token", base);

    let form = vec![
        ("client_id".to_string(), options.client_id.clone()),
        ("device_code".to_string(), options.device_code.clone()),
        (
            "grant_type".to_string(),
            options
                .grant_type
                .clone()
                .unwrap_or_else(|| "urn:ietf:params:oauth:grant-type:device_code".into()),
        ),
    ];

    let resp = OAUTH_CLIENT.post(&url).form(&form).send().map_err(|e| {
        GitLabError::api(
            ErrorCategory::NetworkError,
            503,
            format!("Token poll request failed: {e}"),
            ErrorContext {
                operation: Some("oauth.poll_for_token".into()),
                ..Default::default()
            },
        )
    })?;

    let status = resp.status();
    if status.is_success() {
        resp.json().map_err(|e| {
            GitLabError::api(
                ErrorCategory::ParseError,
                500,
                format!("Failed to parse token response: {e}"),
                ErrorContext::default(),
            )
        })
    } else {
        let body = resp.text().unwrap_or_default();
        Err(GitLabError::api(
            ErrorCategory::AuthorizationDenied,
            status.as_u16(),
            body.clone(),
            ErrorContext {
                operation: Some("oauth.poll_for_token".into()),
                http_status: Some(status.as_u16()),
                response_body: Some(body),
                ..Default::default()
            },
        ))
    }
}

/// Obtém um token de acesso via fluxo Device Grant completo.
///
/// Combina [`request_device_authorization`] e [`poll_for_token`] em um
/// único loop: solicita a autorização, exibe instruções no log e fica
/// consultando até obter o token ou atingir o tempo máximo (`expires_in`).
///
/// ## Params
/// - `options`: Opções de configuração para obter o token.
///
/// ## Returns
/// `Result<OAuthTokenResponse, GitLabError>` — token de acesso obtido com sucesso.
///
/// ## Errors
/// Retorna `GitLabError` com `Timeout` se o tempo máximo de espera for excedido,
/// ou demais erros de rede e autorização.
pub fn get_token(options: &GetTokenOptions) -> Result<OAuthTokenResponse, GitLabError> {
    let device_response = request_device_authorization(&DeviceAuthOptions {
        base_url: options.base_url.clone(),
        client_id: options.client_id.clone(),
        scope: options.scope.clone(),
    })?;

    log::info!(target: "gitlab_wrapper::oauth", "Open this URL in your browser: {}", device_response.verification_uri_complete.as_deref().unwrap_or(&device_response.verification_uri));
    log::info!(target: "gitlab_wrapper::oauth", "Enter the code: {}", device_response.user_code);

    let interval = Duration::from_secs(device_response.interval.max(5));
    let max_duration = Duration::from_secs(device_response.expires_in);

    let start = std::time::Instant::now();

    loop {
        if start.elapsed() >= max_duration {
            return Err(GitLabError::api(
                ErrorCategory::Timeout,
                504,
                "Device authorization timed out",
                ErrorContext {
                    operation: Some("oauth.get_token".into()),
                    ..Default::default()
                },
            ));
        }

        std::thread::sleep(interval);

        match poll_for_token(&PollTokenOptions {
            base_url: options.base_url.clone(),
            client_id: options.client_id.clone(),
            device_code: device_response.device_code.clone(),
            grant_type: None,
        }) {
            Ok(token) => return Ok(token),
            Err(ref err) if is_authorization_pending(err) => continue,
            Err(_) => continue,
        }
    }
}

/// Verifica se o erro retornado indica que a autorização do dispositivo
/// ainda está pendente.
///
/// Analisa o corpo da resposta de erro em busca do campo `error` com valor
/// `"authorization_pending"`.
///
/// ## Params
/// - `err`: Erro retornado pela API do GitLab.
///
/// ## Returns
/// `bool` — `true` se o erro for `authorization_pending`, `false` caso contrário.
fn is_authorization_pending(err: &GitLabError) -> bool {
    if let GitLabError::Api { context, .. } = err {
        if let Some(ref body) = context.response_body {
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(body) {
                return val.get("error").and_then(|v| v.as_str()) == Some("authorization_pending");
            }
        }
    }
    false
}
