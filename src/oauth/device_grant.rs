use std::time::Duration;

use crate::core::errors::{ErrorCategory, ErrorContext, GitLabError};
use crate::types::{DeviceAuthResponse, OAuthTokenResponse};

pub struct DeviceAuthOptions {
    pub base_url: String,
    pub client_id: String,
    pub scope: Option<String>,
}

pub struct PollTokenOptions {
    pub base_url: String,
    pub client_id: String,
    pub device_code: String,
    pub grant_type: Option<String>,
}

pub struct GetTokenOptions {
    pub base_url: String,
    pub client_id: String,
    pub scope: Option<String>,
}

pub fn request_device_authorization(
    options: &DeviceAuthOptions,
) -> Result<DeviceAuthResponse, GitLabError> {
    let base = options.base_url.trim_end_matches('/');
    let url = format!("{}/oauth/authorize_device", base);

    let mut form = vec![("client_id".to_string(), options.client_id.clone())];
    if let Some(ref scope) = options.scope {
        form.push(("scope".to_string(), scope.clone()));
    }

    let client = reqwest::blocking::Client::new();
    let resp = client.post(&url).form(&form).send().map_err(|e| {
        GitLabError::Api {
            category: ErrorCategory::NetworkError,
            status: 503,
            detail: format!("Device authorization request failed: {e}"),
            instance: String::new(),
            context: Box::new(ErrorContext {
                operation: Some("oauth.request_device_authorization".into()),
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
                detail: format!("Failed to parse device auth response: {e}"),
                instance: String::new(),
                context: Box::new(ErrorContext::default()),
            }
        })?)
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

    let client = reqwest::blocking::Client::new();
    let resp = client.post(&url).form(&form).send().map_err(|e| {
        GitLabError::Api {
            category: ErrorCategory::NetworkError,
            status: 503,
            detail: format!("Token poll request failed: {e}"),
            instance: String::new(),
            context: Box::new(ErrorContext {
                operation: Some("oauth.poll_for_token".into()),
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
            Err(GitLabError::Api { ref detail, .. }) if detail == "authorization_pending" => {
                continue;
            }
            _ => continue,
        }
    }
}
