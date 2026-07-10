use crate::core::errors::{ErrorCategory, ErrorContext, GitLabError};
use crate::types::OAuthTokenResponse;

pub struct RefreshTokenOptions {
    pub base_url: String,
    pub client_id: String,
    pub client_secret: Option<String>,
    pub refresh_token: String,
    pub scope: Option<String>,
}

pub struct RevokeTokenOptions {
    pub base_url: String,
    pub client_id: String,
    pub client_secret: Option<String>,
    pub token: String,
}

pub fn refresh_token(options: &RefreshTokenOptions) -> Result<OAuthTokenResponse, GitLabError> {
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

    let client = reqwest::blocking::Client::new();
    let resp = client.post(&url).form(&form).send().map_err(|e| {
        GitLabError::Api {
            category: ErrorCategory::NetworkError,
            status: 503,
            detail: format!("Token refresh request failed: {e}"),
            instance: String::new(),
            context: Box::new(ErrorContext {
                operation: Some("oauth.refresh_token".into()),
                ..Default::default()
            }),
        }
    })?;

    let status = resp.status();
    if status.is_success() {
        Ok(resp.json().map_err(|e| {
            GitLabError::Api {
                category: ErrorCategory::ParseError,
                status: 500,
                detail: format!("Failed to parse token response: {e}"),
                instance: String::new(),
                context: Box::new(ErrorContext::default()),
            }
        })?)
    } else {
        let body = resp.text().unwrap_or_default();
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

pub fn revoke_token(options: &RevokeTokenOptions) -> Result<(), GitLabError> {
    let base = options.base_url.trim_end_matches('/');
    let url = format!("{}/oauth/revoke", base);

    let mut form = vec![
        ("client_id".to_string(), options.client_id.clone()),
        ("token".to_string(), options.token.clone()),
    ];

    if let Some(ref secret) = options.client_secret {
        form.push(("client_secret".to_string(), secret.clone()));
    }

    let client = reqwest::blocking::Client::new();
    let resp = client.post(&url).form(&form).send().map_err(|e| {
        GitLabError::Api {
            category: ErrorCategory::NetworkError,
            status: 503,
            detail: format!("Token revoke request failed: {e}"),
            instance: String::new(),
            context: Box::new(ErrorContext {
                operation: Some("oauth.revoke_token".into()),
                ..Default::default()
            }),
        }
    })?;

    if resp.status().is_success() {
        Ok(())
    } else {
        let status_code = resp.status().as_u16();
        let body = resp.text().unwrap_or_default();
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
