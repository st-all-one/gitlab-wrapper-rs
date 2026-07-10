use crate::core::errors::{ErrorCategory, ErrorContext, GitLabError};
use crate::types::OAuthTokenResponse;
use crate::utils::encoding::encode_query_param;

pub struct AuthCodeUrlOptions {
    pub base_url: String,
    pub client_id: String,
    pub redirect_uri: String,
    pub scope: String,
    pub state: String,
    pub code_challenge: Option<String>,
}

pub fn authorization_code_url(options: &AuthCodeUrlOptions) -> String {
    let base = options.base_url.trim_end_matches('/');
    let mut url = format!(
        "{}/oauth/authorize?client_id={}&redirect_uri={}&response_type=code&scope={}&state={}",
        base,
        encode_query_param(&options.client_id),
        encode_query_param(&options.redirect_uri),
        encode_query_param(&options.scope),
        encode_query_param(&options.state),
    );

    if let Some(ref challenge) = options.code_challenge {
        url.push_str(&format!("&code_challenge={}", encode_query_param(challenge)));
        url.push_str("&code_challenge_method=S256");
    }

    url
}

pub struct ExchangeCodeOptions {
    pub base_url: String,
    pub client_id: String,
    pub client_secret: Option<String>,
    pub code: String,
    pub redirect_uri: String,
    pub code_verifier: Option<String>,
}

pub fn exchange_authorization_code(
    options: &ExchangeCodeOptions,
) -> Result<OAuthTokenResponse, GitLabError> {
    let base = options.base_url.trim_end_matches('/');
    let url = format!("{}/oauth/token", base);

    let mut form = vec![
        ("client_id".to_string(), options.client_id.clone()),
        ("code".to_string(), options.code.clone()),
        ("redirect_uri".to_string(), options.redirect_uri.clone()),
        ("grant_type".to_string(), "authorization_code".to_string()),
    ];

    if let Some(ref secret) = options.client_secret {
        form.push(("client_secret".to_string(), secret.clone()));
    }

    if let Some(ref verifier) = options.code_verifier {
        form.push(("code_verifier".to_string(), verifier.clone()));
    }

    let client = reqwest::blocking::Client::new();
    let resp = client.post(&url).form(&form).send().map_err(|e| {
        GitLabError::Api {
            category: ErrorCategory::NetworkError,
            status: 503,
            detail: format!("OAuth request failed: {e}"),
            instance: String::new(),
            context: Box::new(ErrorContext {
                operation: Some("oauth.exchange_authorization_code".into()),
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
                detail: format!("Failed to parse OAuth response: {e}"),
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
                operation: Some("oauth.exchange_authorization_code".into()),
                http_status: Some(status.as_u16()),
                response_body: Some(body),
                ..Default::default()
            },
        ))
    }
}
