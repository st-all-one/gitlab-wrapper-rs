use gitlab_wrapper::{ErrorCategory, ErrorContext, GitLabError};

#[test]
fn test_error_category_from_status() {
    assert_eq!(ErrorCategory::from_status(304), Some(ErrorCategory::NotModified));
    assert_eq!(ErrorCategory::from_status(400), Some(ErrorCategory::SpamDetected));
    assert_eq!(ErrorCategory::from_status(401), Some(ErrorCategory::AuthenticationFailed));
    assert_eq!(ErrorCategory::from_status(403), Some(ErrorCategory::AuthorizationDenied));
    assert_eq!(ErrorCategory::from_status(404), Some(ErrorCategory::ResourceNotFound));
    assert_eq!(ErrorCategory::from_status(409), Some(ErrorCategory::Conflict));
    assert_eq!(ErrorCategory::from_status(422), Some(ErrorCategory::ValidationError));
    assert_eq!(ErrorCategory::from_status(429), Some(ErrorCategory::RateLimited));
    assert_eq!(ErrorCategory::from_status(500), Some(ErrorCategory::InternalError));
    assert_eq!(ErrorCategory::from_status(503), Some(ErrorCategory::NetworkError));
    assert_eq!(ErrorCategory::from_status(504), Some(ErrorCategory::Timeout));
    assert_eq!(ErrorCategory::from_status(200), None);
}

#[test]
fn test_error_category_http_status() {
    assert_eq!(ErrorCategory::NotModified.http_status(), 304);
    assert_eq!(ErrorCategory::AuthenticationFailed.http_status(), 401);
    assert_eq!(ErrorCategory::RateLimited.http_status(), 429);
}

#[test]
fn test_error_category_display() {
    assert_eq!(ErrorCategory::AuthenticationFailed.to_string(), "authentication-failed");
    assert_eq!(ErrorCategory::ResourceNotFound.to_string(), "resource-not-found");
    assert_eq!(ErrorCategory::RateLimited.to_string(), "rate-limited");
}

#[test]
fn test_api_error_creation() {
    let err = GitLabError::api(
        ErrorCategory::ResourceNotFound,
        404,
        "Project not found",
        ErrorContext {
            operation: Some("projects.get".into()),
            http_status: Some(404),
            ..Default::default()
        },
    );

    match err {
        GitLabError::Api { category, status, detail, .. } => {
            assert_eq!(category, ErrorCategory::ResourceNotFound);
            assert_eq!(status, 404);
            assert_eq!(detail, "Project not found");
        }
        _ => panic!("Expected Api error variant"),
    }
}

#[test]
fn test_api_error_has_instance() {
    let err = GitLabError::api(
        ErrorCategory::InternalError,
        500,
        "Internal error",
        ErrorContext::default(),
    );

    match err {
        GitLabError::Api { instance, .. } => {
            assert!(!instance.is_empty(), "Instance should be a non-empty UUID");
        }
        _ => panic!("Expected Api error variant"),
    }
}

#[test]
fn test_error_category() {
    let api_err = GitLabError::api(
        ErrorCategory::RateLimited,
        429,
        "Rate limit exceeded",
        ErrorContext::default(),
    );
    assert_eq!(api_err.category(), Some(ErrorCategory::RateLimited));

    let config_err: GitLabError = GitLabError::Config("test".into());
    assert_eq!(config_err.category(), None);
}

#[test]
fn test_error_display() {
    let err = GitLabError::api(
        ErrorCategory::AuthenticationFailed,
        401,
        "Bad credentials",
        ErrorContext::default(),
    );
    let display = format!("{}", err);
    assert!(display.contains("GitLab API error"));
    assert!(display.contains("authentication-failed"));
}
