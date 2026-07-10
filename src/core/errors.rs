use std::fmt;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorCategory {
    AuthenticationFailed,
    AuthorizationDenied,
    ResourceNotFound,
    ValidationError,
    Conflict,
    RateLimited,
    SpamDetected,
    NotModified,
    Timeout,
    NetworkError,
    ParseError,
    InternalError,
}

impl ErrorCategory {
    pub fn from_status(status: u16) -> Option<Self> {
        match status {
            304 => Some(Self::NotModified),
            400 => Some(Self::SpamDetected),
            401 => Some(Self::AuthenticationFailed),
            403 => Some(Self::AuthorizationDenied),
            404 => Some(Self::ResourceNotFound),
            409 => Some(Self::Conflict),
            422 => Some(Self::ValidationError),
            429 => Some(Self::RateLimited),
            500 => Some(Self::InternalError),
            503 => Some(Self::NetworkError),
            504 => Some(Self::Timeout),
            _ => None,
        }
    }

    pub fn http_status(&self) -> u16 {
        match self {
            Self::NotModified => 304,
            Self::SpamDetected => 400,
            Self::AuthenticationFailed => 401,
            Self::AuthorizationDenied => 403,
            Self::ResourceNotFound => 404,
            Self::Conflict => 409,
            Self::ValidationError => 422,
            Self::RateLimited => 429,
            Self::InternalError | Self::ParseError => 500,
            Self::NetworkError => 503,
            Self::Timeout => 504,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::AuthenticationFailed => "authentication-failed",
            Self::AuthorizationDenied => "authorization-denied",
            Self::ResourceNotFound => "resource-not-found",
            Self::ValidationError => "validation-error",
            Self::Conflict => "conflict",
            Self::RateLimited => "rate-limited",
            Self::SpamDetected => "spam-detected",
            Self::NotModified => "not-modified",
            Self::Timeout => "timeout",
            Self::NetworkError => "network-error",
            Self::ParseError => "parse-error",
            Self::InternalError => "internal-error",
        }
    }
}

impl fmt::Display for ErrorCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.description())
    }
}

#[derive(Debug, Clone, Default)]
pub struct ErrorContext {
    pub operation: Option<String>,
    pub http_status: Option<u16>,
    pub api_errors: Option<Vec<String>>,
    pub response_body: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum GitLabError {
    #[error("GitLab API error: {detail} (category: {category})")]
    Api {
        category: ErrorCategory,
        status: u16,
        detail: String,
        instance: String,
        context: Box<ErrorContext>,
    },

    #[error("HTTP error: {0}")]
    Http(reqwest::Error),

    #[error("Rate limit exceeded, retry after {retry_after:?}")]
    RateLimited {
        retry_after: Option<u64>,
        context: Box<ErrorContext>,
    },

    #[error("Timeout after {duration:?}")]
    Timeout {
        duration: std::time::Duration,
        context: Box<ErrorContext>,
    },

    #[error("URL parse error: {0}")]
    Url(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Configuration error: {0}")]
    Config(String),
}

impl GitLabError {
    pub fn api(
        category: ErrorCategory,
        status: u16,
        detail: impl Into<String>,
        context: ErrorContext,
    ) -> Self {
        let instance = Uuid::new_v7(uuid::Timestamp::now(uuid::NoContext)).to_string();
        Self::Api {
            category,
            status,
            detail: detail.into(),
            instance,
            context: Box::new(context),
        }
    }

    pub fn category(&self) -> Option<ErrorCategory> {
        match self {
            Self::Api { category, .. } => Some(*category),
            Self::RateLimited { .. } => Some(ErrorCategory::RateLimited),
            Self::Timeout { .. } => Some(ErrorCategory::Timeout),
            Self::Http(e) if e.is_timeout() => Some(ErrorCategory::Timeout),
            Self::Http(e) if e.is_connect() => Some(ErrorCategory::NetworkError),
            Self::Http(e) if e.is_status() => {
                e.status()
                    .and_then(|s| ErrorCategory::from_status(s.as_u16()))
            }
            _ => None,
        }
    }
}

impl From<reqwest::Error> for GitLabError {
    fn from(e: reqwest::Error) -> Self {
        log::error!(target: "gitlab_wrapper::http", "HTTP error: {}", e);
        if e.is_timeout() {
            Self::Timeout {
                duration: std::time::Duration::from_secs(30),
                context: Box::new(ErrorContext::default()),
            }
        } else {
            Self::Http(e)
        }
    }
}
