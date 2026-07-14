//! Renovação e revogação de tokens OAuth para GitLab.
//!
//! Permite renovar um token de acesso expirado usando o `refresh_token`
//! e revogar um token existente, tornando-o inválido.

use std::sync::LazyLock;

use crate::core::errors::{ErrorCategory, ErrorContext, GitLabError};
use crate::types::OAuthTokenResponse;

// Cliente HTTP dedicado — requisições OAuth são esporádicas.
static OAUTH_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(reqwest::Client::new);

/// Opções para renovar um token de acesso OAuth.
pub struct RefreshTokenOptions {
    /// URL base da instância do GitLab (ex.: `https://gitlab.com`).
    pub base_url: String,
    /// ID do cliente OAuth registrado.
    pub client_id: String,
    /// Segredo do cliente OAuth (opcional, usado em fluxos confidenciais).
    pub client_secret: Option<String>,
    /// Token de atualização (`refresh_token`) obtido na autorização original.
    pub refresh_token: String,
    /// Escopos solicitados (opcional, separados por espaço).
    pub scope: Option<String>,
}

/// Opções para revogar um token OAuth.
pub struct RevokeTokenOptions {
    /// URL base da instância do GitLab (ex.: `https://gitlab.com`).
    pub base_url: String,
    /// ID do cliente OAuth registrado.
    pub client_id: String,
    /// Segredo do cliente OAuth (opcional, usado em fluxos confidenciais).
    pub client_secret: Option<String>,
    /// Token de acesso ou de atualização a ser revogado.
    pub token: String,
}

/// Renova um token de acesso OAuth usando o `refresh_token`.
pub async fn refresh_token(
    options: &RefreshTokenOptions,
) -> Result<OAuthTokenResponse, GitLabError> {
    let base = options.base_url.trim_end_matches('/');
    let url = format!("{}/oauth/token", base);

    let mut form = vec![
        ("client_id".to_string(), options.client_id.clone()),
        ("refresh_token".to_string(), options.refresh_token.clone()),
        ("grant_type".to_string(), "refresh_token".to_string()),
    ];

    if let Some(ref secret) = options.client_secret {
        form.push(("client_secret".to_string(), secret.clone()));
    }

    if let Some(ref scope) = options.scope {
        form.push(("scope".to_string(), scope.clone()));
    }

    let resp = OAUTH_CLIENT.post(&url).form(&form).send().await.map_err(|e| {
        GitLabError::api(
            ErrorCategory::NetworkError,
            503,
            format!("Token refresh request failed: {e}"),
            ErrorContext { operation: Some("oauth.refresh_token".into()), ..Default::default() },
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
            ErrorCategory::AuthenticationFailed,
            status.as_u16(),
            body.clone(),
            ErrorContext {
                operation: Some("oauth.refresh_token".into()),
                http_status: Some(status.as_u16()),
                response_body: Some(body),
                ..Default::default()
            },
        ))
    }
}

/// Revoga um token OAuth (acesso ou atualização), tornando-o inválido.
pub async fn revoke_token(options: &RevokeTokenOptions) -> Result<(), GitLabError> {
    let base = options.base_url.trim_end_matches('/');
    let url = format!("{}/oauth/revoke", base);

    let mut form = vec![
        ("client_id".to_string(), options.client_id.clone()),
        ("token".to_string(), options.token.clone()),
    ];

    if let Some(ref secret) = options.client_secret {
        form.push(("client_secret".to_string(), secret.clone()));
    }

    let resp = OAUTH_CLIENT.post(&url).form(&form).send().await.map_err(|e| {
        GitLabError::api(
            ErrorCategory::NetworkError,
            503,
            format!("Token revoke request failed: {e}"),
            ErrorContext { operation: Some("oauth.revoke_token".into()), ..Default::default() },
        )
    })?;

    if resp.status().is_success() {
        Ok(())
    } else {
        let status_code = resp.status().as_u16();
        let body = resp.text().await.unwrap_or_default();
        Err(GitLabError::api(
            ErrorCategory::AuthenticationFailed,
            status_code,
            body.clone(),
            ErrorContext {
                operation: Some("oauth.revoke_token".into()),
                http_status: Some(status_code),
                response_body: Some(body),
                ..Default::default()
            },
        ))
    }
}
