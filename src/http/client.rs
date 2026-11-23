use reqwest::StatusCode;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};

use crate::core::config::{AuthMethod, ResolvedConfig};
use crate::core::constants::{API_VERSION, DEFAULT_PER_PAGE, USER_AGENT_VALUE};
use crate::core::errors::{ErrorCategory, ErrorContext, GitLabError};
use crate::http::pagination::{
    PaginationInfo, extract_error_messages, extract_retry_after, parse_pagination_headers,
};
use crate::http::rate_limiter::RateLimiter;
use crate::utils::encoding::encode_query_param;

/// Alvo de tracing usado pelos spans e eventos do cliente HTTP.
const HTTP_TARGET: &str = "gitlab_wrapper::http";

/// Cliente HTTP interno que encapsula chamadas assíncronas à API do GitLab.
///
/// Mantém um `reqwest::Client`, a configuração resolvida,
/// e um limitador de taxa assíncrono (`RateLimiter`) baseado em semáforo.
#[derive(Debug, Clone)]
pub(crate) struct HttpClient {
    client: reqwest::Client,
    config: ResolvedConfig,
    rate_limiter: RateLimiter,
}

impl HttpClient {
    /// Cria uma nova instância de `HttpClient`.
    ///
    /// Aplica `timeout` e `user_agent` via builder. Como ambos os valores são
    /// controlados pela biblioteca (constante e duração já validada), uma falha
    /// de build é inesperada; nesse caso usa `reqwest::Client::new` como fallback
    /// determinístico e sem pânico, registrando um aviso. O timeout configurado
    /// continua disponível em `config` para futuras aplicações por requisição.
    ///
    /// ## Params
    /// - `config`: Configuração resolvida do GitLab (URL, token, timeout, etc.).
    ///
    /// ## Returns
    /// `HttpClient` — nova instância pronta para uso.
    pub fn new(config: ResolvedConfig) -> Self {
        let rate_limiter = RateLimiter::new(config.max_rps);
        let client = reqwest::Client::builder()
            .timeout(config.timeout)
            .user_agent(USER_AGENT_VALUE)
            .build()
            .unwrap_or_else(|error| {
                tracing::warn!(target: HTTP_TARGET, "falling back to default reqwest client: {error}");
                reqwest::Client::new()
            });
        Self {
            client,
            config,
            rate_limiter,
        }
    }

    /// Monta a URL completa para uma requisição à API do GitLab.
    ///
    /// Concatena a URL base, o prefixo da API e o *path* informado,
    /// adicionando os parâmetros de consulta com codificação percentual.
    pub(crate) fn build_url(&self, path: &str, query: &[(String, String)]) -> String {
        let base = self.config.base_url.trim_end_matches('/');
        let path = path.trim_start_matches('/');
        let mut url = format!("{base}/api/{API_VERSION}/{path}");

        if !query.is_empty() {
            let mut first = true;
            for (key, val) in query {
                if val.is_empty() {
                    continue;
                }
                if first {
                    url.push('?');
                    first = false;
                } else {
                    url.push('&');
                }
                url.push_str(&encode_query_param(key));
                url.push('=');
                url.push_str(&encode_query_param(val));
            }
        }

        url
    }

    fn build_headers(&self, extra: Option<&HeaderMap>) -> Result<HeaderMap, GitLabError> {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        if let Some(token) = &self.config.token {
            match self.config.auth_method {
                AuthMethod::Header => {
                    headers.insert(
                        "PRIVATE-TOKEN",
                        HeaderValue::from_str(token)
                            .map_err(|e| GitLabError::Config(format!("Invalid token: {e}")))?,
                    );
                }
                AuthMethod::Bearer => {
                    let value = format!("Bearer {token}");
                    headers.insert(
                        AUTHORIZATION,
                        HeaderValue::from_str(&value).map_err(|e| {
                            GitLabError::Config(format!("Invalid bearer token: {e}"))
                        })?,
                    );
                }
            }
        }

        if let Some(sudo) = &self.config.sudo {
            headers.insert(
                "SUDO",
                HeaderValue::from_str(sudo)
                    .map_err(|e| GitLabError::Config(format!("Invalid sudo header: {e}")))?,
            );
        }

        if let Some(extra) = extra {
            headers.extend(extra.iter().map(|(k, v)| (k.clone(), v.clone())));
        }

        Ok(headers)
    }

    #[tracing::instrument(target = "gitlab_wrapper::http", skip(self, response), fields(operation = "http.handle_response", status = tracing::field::Empty))]
    async fn handle_response<T: serde::de::DeserializeOwned>(
        &self,
        response: reqwest::Response,
        operation: &str,
    ) -> Result<T, GitLabError> {
        let status = response.status();
        let http_status = status.as_u16();
        tracing::Span::current().record("status", http_status);

        if status.is_success() || status == StatusCode::NOT_MODIFIED {
            if http_status == 204 {
                return serde_json::from_value(serde_json::Value::Null).map_err(GitLabError::from);
            }
            return response.json::<T>().await.map_err(|e| {
                tracing::error!(target: HTTP_TARGET, "{} failed to parse JSON: {}", operation, e);
                GitLabError::Api {
                    category: ErrorCategory::ParseError,
                    status: http_status,
                    detail: format!("Failed to parse response: {e}"),
                    instance: String::new(),
                    context: Box::new(ErrorContext {
                        operation: Some(operation.to_string()),
                        http_status: Some(http_status),
                        ..Default::default()
                    }),
                }
            });
        }

        let body = response.text().await.unwrap_or_default();
        let api_errors = extract_error_messages(&body);
        let category =
            ErrorCategory::from_status(http_status).unwrap_or(ErrorCategory::InternalError);

        tracing::error!(target: HTTP_TARGET, "{} failed: status={}, category={}", operation, http_status, category);

        if http_status == 429 {
            let retry_after = extract_retry_after(&body);
            return Err(GitLabError::RateLimited {
                retry_after,
                context: Box::new(ErrorContext {
                    operation: Some(operation.to_string()),
                    http_status: Some(http_status),
                    response_body: Some(body),
                    api_errors,
                }),
            });
        }

        Err(GitLabError::api(
            category,
            http_status,
            body.clone(),
            ErrorContext {
                operation: Some(operation.to_string()),
                http_status: Some(http_status),
                response_body: Some(body),
                api_errors,
            },
        ))
    }

    /// Executa uma requisição HTTP GET.
    #[tracing::instrument(target = "gitlab_wrapper::http", skip(self, query), fields(operation = "http.get", method = "GET", path, status = tracing::field::Empty))]
    pub async fn get<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        query: &[(String, String)],
        operation: &str,
    ) -> Result<T, GitLabError> {
        self.rate_limiter.acquire().await;
        let url = self.build_url(path, query);
        let headers = self.build_headers(None)?;

        tracing::debug!(target: HTTP_TARGET, "GET {} - {}", operation, path);

        let resp = self
            .client
            .get(&url)
            .headers(headers)
            .send()
            .await
            .map_err(|e| {
                tracing::error!(target: HTTP_TARGET, "GET {} failed: {}", operation, e);
                GitLabError::from(e)
            })?;
        tracing::Span::current().record("status", resp.status().as_u16());

        self.handle_response(resp, operation).await
    }

    /// Executa uma requisição HTTP GET e retorna os dados junto com informações de paginação.
    #[tracing::instrument(target = "gitlab_wrapper::http", skip(self, query), fields(operation = "http.get_with_headers", method = "GET", path, status = tracing::field::Empty))]
    pub async fn get_with_headers<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        query: &[(String, String)],
        operation: &str,
    ) -> Result<(T, PaginationInfo), GitLabError> {
        self.rate_limiter.acquire().await;
        let url = self.build_url(path, query);
        let headers = self.build_headers(None)?;

        tracing::debug!(target: HTTP_TARGET, "GET {} (with headers) - {}", operation, path);

        let resp = self
            .client
            .get(&url)
            .headers(headers)
            .send()
            .await
            .map_err(|e| {
                tracing::error!(target: HTTP_TARGET, "GET {} failed: {}", operation, e);
                GitLabError::from(e)
            })?;

        let pagination = parse_pagination_headers(resp.headers());
        let status = resp.status();
        let http_status = status.as_u16();
        tracing::Span::current().record("status", http_status);

        if status.is_success() {
            let data: T = resp.json().await.map_err(|e| {
                tracing::error!(target: HTTP_TARGET, "GET {} failed to parse JSON: {}", operation, e);
                GitLabError::Api {
                    category: ErrorCategory::ParseError,
                    status: http_status,
                    detail: format!("Failed to parse response: {e}"),
                    instance: String::new(),
                    context: Box::new(ErrorContext {
                        operation: Some(operation.to_string()),
                        http_status: Some(http_status),
                        ..Default::default()
                    }),
                }
            })?;
            return Ok((data, pagination));
        }

        let body = resp.text().await.unwrap_or_default();
        let api_errors = extract_error_messages(&body);
        let category =
            ErrorCategory::from_status(http_status).unwrap_or(ErrorCategory::InternalError);

        tracing::error!(target: HTTP_TARGET, "{} failed: status={}, category={}", operation, http_status, category);

        Err(GitLabError::api(
            category,
            http_status,
            body.clone(),
            ErrorContext {
                operation: Some(operation.to_string()),
                http_status: Some(http_status),
                response_body: Some(body),
                api_errors,
            },
        ))
    }

    /// Executa uma requisição HTTP POST.
    #[tracing::instrument(target = "gitlab_wrapper::http", skip(self, body), fields(operation = "http.post", method = "POST", path, status = tracing::field::Empty))]
    pub async fn post<T: serde::de::DeserializeOwned, B: serde::Serialize + Sync>(
        &self,
        path: &str,
        body: &B,
        operation: &str,
    ) -> Result<T, GitLabError> {
        self.rate_limiter.acquire().await;
        let url = self.build_url(path, &[]);
        let headers = self.build_headers(None)?;

        tracing::debug!(target: HTTP_TARGET, "POST {} - {}", operation, path);

        let resp = self
            .client
            .post(&url)
            .headers(headers)
            .json(body)
            .send()
            .await
            .map_err(|e| {
                tracing::error!(target: HTTP_TARGET, "POST {} failed: {}", operation, e);
                GitLabError::from(e)
            })?;
        tracing::Span::current().record("status", resp.status().as_u16());

        self.handle_response(resp, operation).await
    }

    /// Executa uma requisição HTTP POST com `multipart/form-data`.
    ///
    /// Útil para upload de arquivos (avatar, anexos de wiki, etc.).
    /// O `Content-Type` é definido automaticamente pelo reqwest com o
    /// boundary apropriado — **não** inclua `Content-Type` nos headers.
    #[tracing::instrument(target = "gitlab_wrapper::http", skip(self, form), fields(operation = "http.post_multipart", method = "POST", path, status = tracing::field::Empty))]
    pub async fn post_multipart<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        form: reqwest::multipart::Form,
        operation: &str,
    ) -> Result<T, GitLabError> {
        self.rate_limiter.acquire().await;
        let url = self.build_url(path, &[]);
        // Apenas headers de autenticação — sem Content-Type (reqwest define multipart boundary)
        let mut headers = self.build_headers(None)?;
        headers.remove(CONTENT_TYPE);

        tracing::debug!(target: HTTP_TARGET, "POST multipart {} - {}", operation, path);

        let resp = self
            .client
            .post(&url)
            .headers(headers)
            .multipart(form)
            .send()
            .await
            .map_err(|e| {
                tracing::error!(target: HTTP_TARGET, "POST multipart {} failed: {}", operation, e);
                GitLabError::from(e)
            })?;
        tracing::Span::current().record("status", resp.status().as_u16());

        self.handle_response(resp, operation).await
    }

    /// Executa uma requisição HTTP PUT com `multipart/form-data`.
    ///
    /// Útil para upload de avatar (`PUT /projects/:id` com campo avatar).
    /// O `Content-Type` é definido automaticamente pelo reqwest com o
    /// boundary apropriado.
    #[tracing::instrument(target = "gitlab_wrapper::http", skip(self, form), fields(operation = "http.put_multipart", method = "PUT", path, status = tracing::field::Empty))]
    pub async fn put_multipart<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        form: reqwest::multipart::Form,
        operation: &str,
    ) -> Result<T, GitLabError> {
        self.rate_limiter.acquire().await;
        let url = self.build_url(path, &[]);
        let mut headers = self.build_headers(None)?;
        headers.remove(CONTENT_TYPE);

        tracing::debug!(target: HTTP_TARGET, "PUT multipart {} - {}", operation, path);

        let resp = self
            .client
            .put(&url)
            .headers(headers)
            .multipart(form)
            .send()
            .await
            .map_err(|e| {
                tracing::error!(target: HTTP_TARGET, "PUT multipart {} failed: {}", operation, e);
                GitLabError::from(e)
            })?;
        tracing::Span::current().record("status", resp.status().as_u16());

        self.handle_response(resp, operation).await
    }

    /// Executa uma requisição HTTP PUT.
    #[tracing::instrument(target = "gitlab_wrapper::http", skip(self, body), fields(operation = "http.put", method = "PUT", path, status = tracing::field::Empty))]
    pub async fn put<T: serde::de::DeserializeOwned, B: serde::Serialize + Sync>(
        &self,
        path: &str,
        body: &B,
        operation: &str,
    ) -> Result<T, GitLabError> {
        self.rate_limiter.acquire().await;
        let url = self.build_url(path, &[]);
        let headers = self.build_headers(None)?;

        tracing::debug!(target: HTTP_TARGET, "PUT {} - {}", operation, path);

        let resp = self
            .client
            .put(&url)
            .headers(headers)
            .json(body)
            .send()
            .await
            .map_err(|e| {
                tracing::error!(target: HTTP_TARGET, "PUT {} failed: {}", operation, e);
                GitLabError::from(e)
            })?;
        tracing::Span::current().record("status", resp.status().as_u16());

        self.handle_response(resp, operation).await
    }

    /// Executa uma requisição HTTP DELETE.
    #[tracing::instrument(target = "gitlab_wrapper::http", skip(self, query), fields(operation = "http.delete", method = "DELETE", path, status = tracing::field::Empty))]
    pub async fn delete(
        &self,
        path: &str,
        query: &[(String, String)],
        operation: &str,
    ) -> Result<(), GitLabError> {
        self.rate_limiter.acquire().await;
        let url = self.build_url(path, query);
        let headers = self.build_headers(None)?;

        tracing::debug!(target: HTTP_TARGET, "DELETE {} - {}", operation, path);

        let resp = self
            .client
            .delete(&url)
            .headers(headers)
            .send()
            .await
            .map_err(|e| {
                tracing::error!(target: HTTP_TARGET, "DELETE {} failed: {}", operation, e);
                GitLabError::from(e)
            })?;

        let status = resp.status();
        let http_status = status.as_u16();
        tracing::Span::current().record("status", http_status);
        if status.is_success() || status == StatusCode::NO_CONTENT {
            return Ok(());
        }

        let body = resp.text().await.unwrap_or_default();
        let category =
            ErrorCategory::from_status(http_status).unwrap_or(ErrorCategory::InternalError);

        Err(GitLabError::api(
            category,
            http_status,
            body.clone(),
            ErrorContext {
                operation: Some(operation.to_string()),
                http_status: Some(http_status),
                response_body: Some(body),
                ..Default::default()
            },
        ))
    }

    /// Executa uma requisição HTTP DELETE com corpo.
    #[tracing::instrument(target = "gitlab_wrapper::http", skip(self, body), fields(operation = "http.delete_with_body", method = "DELETE", path, status = tracing::field::Empty))]
    pub async fn delete_with_body<T: serde::de::DeserializeOwned, B: serde::Serialize + Sync>(
        &self,
        path: &str,
        body: &B,
        operation: &str,
    ) -> Result<T, GitLabError> {
        self.rate_limiter.acquire().await;
        let url = self.build_url(path, &[]);
        let headers = self.build_headers(None)?;

        tracing::debug!(target: HTTP_TARGET, "DELETE {} (with body) - {}", operation, path);

        let resp = self
            .client
            .delete(&url)
            .headers(headers)
            .json(body)
            .send()
            .await
            .map_err(|e| {
                tracing::error!(target: HTTP_TARGET, "DELETE {} failed: {}", operation, e);
                GitLabError::from(e)
            })?;
        tracing::Span::current().record("status", resp.status().as_u16());

        self.handle_response(resp, operation).await
    }

    /// Executa uma requisição HTTP GET e retorna o corpo como bytes brutos.
    #[tracing::instrument(target = "gitlab_wrapper::http", skip(self, query), fields(operation = "http.get_raw", method = "GET", path, status = tracing::field::Empty))]
    pub async fn get_raw(
        &self,
        path: &str,
        query: &[(String, String)],
        operation: &str,
    ) -> Result<Vec<u8>, GitLabError> {
        self.rate_limiter.acquire().await;
        let url = self.build_url(path, query);
        let headers = self.build_headers(None)?;

        tracing::debug!(target: HTTP_TARGET, "GET raw {} - {}", operation, path);

        let resp = self
            .client
            .get(&url)
            .headers(headers)
            .send()
            .await
            .map_err(|e| {
                tracing::error!(target: HTTP_TARGET, "GET raw {} failed: {}", operation, e);
                GitLabError::from(e)
            })?;

        let status = resp.status();
        let http_status = status.as_u16();
        tracing::Span::current().record("status", http_status);

        if status.is_success() {
            return resp
                .bytes()
                .await
                .map(|b| b.to_vec())
                .map_err(|e| GitLabError::Api {
                    category: ErrorCategory::ParseError,
                    status: http_status,
                    detail: format!("Failed to read response body: {e}"),
                    instance: String::new(),
                    context: Box::new(ErrorContext {
                        operation: Some(operation.to_string()),
                        http_status: Some(http_status),
                        ..Default::default()
                    }),
                });
        }

        let body = resp.text().await.unwrap_or_default();
        let category =
            ErrorCategory::from_status(http_status).unwrap_or(ErrorCategory::InternalError);
        Err(GitLabError::api(
            category,
            http_status,
            body.clone(),
            ErrorContext {
                operation: Some(operation.to_string()),
                http_status: Some(http_status),
                response_body: Some(body),
                ..Default::default()
            },
        ))
    }

    /// Executa uma requisição HTTP GET e retorna o corpo como texto bruto.
    #[tracing::instrument(target = "gitlab_wrapper::http", skip(self, query), fields(operation = "http.get_raw_text", method = "GET", path, status = tracing::field::Empty))]
    pub async fn get_raw_text(
        &self,
        path: &str,
        query: &[(String, String)],
        operation: &str,
    ) -> Result<String, GitLabError> {
        self.rate_limiter.acquire().await;
        let url = self.build_url(path, query);
        let headers = self.build_headers(None)?;

        tracing::debug!(target: HTTP_TARGET, "GET raw text {} - {}", operation, path);

        let resp = self
            .client
            .get(&url)
            .headers(headers)
            .send()
            .await
            .map_err(|e| {
                tracing::error!(target: HTTP_TARGET, "GET raw text {} failed: {}", operation, e);
                GitLabError::from(e)
            })?;

        let status = resp.status();
        let http_status = status.as_u16();
        tracing::Span::current().record("status", http_status);

        if status.is_success() {
            return resp.text().await.map_err(|e| GitLabError::Api {
                category: ErrorCategory::ParseError,
                status: http_status,
                detail: format!("Failed to read response body: {e}"),
                instance: String::new(),
                context: Box::new(ErrorContext {
                    operation: Some(operation.to_string()),
                    http_status: Some(http_status),
                    ..Default::default()
                }),
            });
        }

        let body = resp.text().await.unwrap_or_default();
        let category =
            ErrorCategory::from_status(http_status).unwrap_or(ErrorCategory::InternalError);
        Err(GitLabError::api(
            category,
            http_status,
            body.clone(),
            ErrorContext {
                operation: Some(operation.to_string()),
                http_status: Some(http_status),
                response_body: Some(body),
                ..Default::default()
            },
        ))
    }

    /// Auto-pagina todas as páginas de um endpoint paginado (paginação baseada em página).
    #[tracing::instrument(
        target = "gitlab_wrapper::http",
        skip(self, query),
        fields(operation = "http.paginate_all")
    )]
    pub async fn paginate_all<T: serde::de::DeserializeOwned + 'static>(
        &self,
        path: &str,
        query: &[(String, String)],
        operation: &str,
    ) -> Result<Vec<T>, GitLabError> {
        use crate::http::pagination::paginate_all as auto_paginate;
        let this = self.clone();
        let path_owned = path.to_string();
        let query_owned = query.to_vec();
        let op_owned = operation.to_string();
        auto_paginate(
            {
                let this = this.clone();
                let path = path_owned.clone();
                let query = query_owned.clone();
                let op = op_owned.clone();
                move |page: u32| {
                    let this = this.clone();
                    let mut paged_query = query.clone();
                    paged_query.push(("per_page".to_string(), DEFAULT_PER_PAGE.to_string()));
                    paged_query.push(("page".to_string(), page.to_string()));
                    let path = path.clone();
                    let op = op.clone();
                    Box::pin(async move { this.get_with_headers(&path, &paged_query, &op).await })
                }
            },
            &op_owned,
        )
        .await
    }

    #[expect(dead_code, reason = "reserved for future keyset pagination")]
    /// Auto-pagina todas as páginas de um endpoint com paginação por cursor (*keyset*).
    #[tracing::instrument(
        target = "gitlab_wrapper::http",
        skip(self, query),
        fields(operation = "http.keyset_paginate_all")
    )]
    pub async fn keyset_paginate_all<T: serde::de::DeserializeOwned + 'static>(
        &self,
        path: &str,
        query: &[(String, String)],
        operation: &str,
    ) -> Result<Vec<T>, GitLabError> {
        use crate::http::pagination::keyset_paginate_all as auto_keyset;
        let this = self.clone();
        let path_owned = path.to_string();
        let query_owned = query.to_vec();
        let op_owned = operation.to_string();
        auto_keyset(
            {
                let this = this.clone();
                let path = path_owned.clone();
                let query = query_owned.clone();
                let op = op_owned.clone();
                move |cursor: Option<String>| {
                    let this = this.clone();
                    let mut paged_query = query.clone();
                    paged_query.push(("pagination".to_string(), "keyset".to_string()));
                    if let Some(c) = cursor {
                        paged_query.push(("id_after".to_string(), c));
                    }
                    let path = path.clone();
                    let op = op.clone();
                    Box::pin(async move { this.get_with_headers(&path, &paged_query, &op).await })
                }
            },
            &op_owned,
        )
        .await
    }
}
