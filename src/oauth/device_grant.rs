//! Fluxo Device Authorization Grant (OAuth 2.0) para GitLab.
//!
//! Implementa o fluxo de autorização para dispositivos sem navegador,
//! conforme a RFC 8628.

use std::sync::LazyLock;
use std::time::Duration;

use crate::core::errors::{ErrorCategory, ErrorContext, GitLabError};
use crate::types::{DeviceAuthResponse, OAuthTokenResponse};

// Cliente HTTP dedicado — requisições OAuth são esporádicas.
static OAUTH_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(reqwest::Client::new);

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
/// ## Errors
/// Retorna `GitLabError::Api` com `NetworkError` em falha de transporte,
/// `ParseError` na desserialização ou `AuthorizationDenied` se o GitLab recusar a solicitação.
#[tracing::instrument(
    target = "gitlab_wrapper::oauth",
    skip(options),
    fields(operation = "oauth.request_device_authorization")
)]
pub async fn request_device_authorization(
    options: &DeviceAuthOptions,
) -> Result<DeviceAuthResponse, GitLabError> {
    let base = options.base_url.trim_end_matches('/');
    let url = format!("{base}/oauth/authorize_device");

    let mut form = vec![("client_id".to_string(), options.client_id.clone())];
    if let Some(scope) = &options.scope {
        form.push(("scope".to_string(), scope.clone()));
    }

    let resp = OAUTH_CLIENT
        .post(&url)
        .form(&form)
        .send()
        .await
        .map_err(|e| {
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
        resp.json().await.map_err(|e| {
            GitLabError::api(
                ErrorCategory::ParseError,
                500,
                format!("Failed to parse device auth response: {e}"),
                ErrorContext::default(),
            )
        })
    } else {
        let body = resp.text().await.unwrap_or_default();
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
/// ## Errors
/// Retorna `GitLabError::Api` com `NetworkError` em falha de transporte,
/// `ParseError` na desserialização ou `AuthorizationDenied` enquanto a autorização estiver pendente.
#[tracing::instrument(
    target = "gitlab_wrapper::oauth",
    skip(options),
    fields(operation = "oauth.poll_for_token")
)]
pub async fn poll_for_token(options: &PollTokenOptions) -> Result<OAuthTokenResponse, GitLabError> {
    let base = options.base_url.trim_end_matches('/');
    let url = format!("{base}/oauth/token");

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

    let resp = OAUTH_CLIENT
        .post(&url)
        .form(&form)
        .send()
        .await
        .map_err(|e| {
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
        resp.json().await.map_err(|e| {
            GitLabError::api(
                ErrorCategory::ParseError,
                500,
                format!("Failed to parse token response: {e}"),
                ErrorContext::default(),
            )
        })
    } else {
        let body = resp.text().await.unwrap_or_default();
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
/// único loop assíncrono.
///
/// ## Errors
/// Retorna `GitLabError::Api` com `Timeout` se a autorização expirar, propagando
/// os erros de rede, parsing e autorização das funções de autorização e consulta de token.
#[tracing::instrument(
    target = "gitlab_wrapper::oauth",
    skip(options),
    fields(operation = "oauth.get_token")
)]
pub async fn get_token(options: &GetTokenOptions) -> Result<OAuthTokenResponse, GitLabError> {
    let device_response = request_device_authorization(&DeviceAuthOptions {
        base_url: options.base_url.clone(),
        client_id: options.client_id.clone(),
        scope: options.scope.clone(),
    })
    .await?;

    tracing::info!(target: "gitlab_wrapper::oauth", "Open this URL in your browser: {}", device_response.verification_uri_complete.as_deref().unwrap_or(&device_response.verification_uri));
    tracing::info!(target: "gitlab_wrapper::oauth", "Enter the code: {}", device_response.user_code);

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

        tokio::time::sleep(interval).await;

        let result = poll_for_token(&PollTokenOptions {
            base_url: options.base_url.clone(),
            client_id: options.client_id.clone(),
            device_code: device_response.device_code.clone(),
            grant_type: None,
        })
        .await;
        if let Ok(token) = result {
            return Ok(token);
        }
    }
}
