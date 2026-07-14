use gitlab_wrapper::{AuthMethod, ErrorCategory, GitLabClient, GitLabConfig, GitLabError};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

struct TestContext {
    client: GitLabClient,
    #[allow(dead_code)]
    server: MockServer,
}

async fn setup() -> TestContext {
    let server = MockServer::start().await;
    let base_url = server.uri();

    let client = GitLabClient::new(GitLabConfig {
        base_url,
        token: Some("test-token".into()),
        ..Default::default()
    })
    .unwrap();

    TestContext { client, server }
}

#[test]
fn test_client_creation() {
    let client = GitLabClient::new(GitLabConfig {
        base_url: "https://gitlab.com".into(),
        token: Some("glpat-xxx".into()),
        ..Default::default()
    });
    assert!(client.is_ok());
}

#[test]
fn test_client_creation_fails_without_url() {
    let result = GitLabClient::new(GitLabConfig {
        base_url: String::new(),
        token: Some("glpat-xxx".into()),
        ..Default::default()
    });
    assert!(result.is_err());
}

#[test]
fn test_client_creation_succeeds_without_token() {
    let result = GitLabClient::new(GitLabConfig {
        base_url: "https://gitlab.com".into(),
        token: None,
        ..Default::default()
    });
    assert!(result.is_ok());
}

#[test]
fn test_auth_method_default_is_bearer() {
    assert_eq!(AuthMethod::default(), AuthMethod::Bearer);
}

#[tokio::test]
async fn test_projects_list() {
    let ctx = setup().await;
    Mock::given(method("GET"))
        .and(path("/api/v4/projects"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "id": 1,
                "name": "Test Project",
                "path": "test-project"
            }
        ])))
        .mount(&ctx.server)
        .await;

    let projects = ctx.client.projects.list(None).await.unwrap();
    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].name, "Test Project");
    assert_eq!(projects[0].id, 1u64);
}

#[tokio::test]
async fn test_projects_get() {
    let ctx = setup().await;
    Mock::given(method("GET"))
        .and(path("/api/v4/projects/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": 1,
            "name": "Test Project",
            "path": "test-project"
        })))
        .mount(&ctx.server)
        .await;

    let project = ctx.client.projects.get(1).await.unwrap();
    assert_eq!(project.name, "Test Project");
}

#[tokio::test]
async fn test_404_error() {
    let ctx = setup().await;
    Mock::given(method("GET"))
        .and(path("/api/v4/projects/999"))
        .respond_with(ResponseTemplate::new(404).set_body_string("Not Found"))
        .mount(&ctx.server)
        .await;

    let result = ctx.client.projects.get(999).await;
    match result {
        Err(GitLabError::Api { category, status, .. }) => {
            assert_eq!(category, ErrorCategory::ResourceNotFound);
            assert_eq!(status, 404);
        }
        _ => panic!("Expected 404 Api error"),
    }
}

#[tokio::test]
async fn test_401_error() {
    let ctx = setup().await;
    Mock::given(method("GET"))
        .and(path("/api/v4/user"))
        .respond_with(ResponseTemplate::new(401).set_body_string("Unauthorized"))
        .mount(&ctx.server)
        .await;

    let result = ctx.client.users.get_current().await;
    match result {
        Err(GitLabError::Api { category, .. }) => {
            assert_eq!(category, ErrorCategory::AuthenticationFailed);
        }
        _ => panic!("Expected 401 Api error"),
    }
}

#[tokio::test]
async fn test_auth_header_sent() {
    let ctx = setup().await;
    Mock::given(method("GET"))
        .and(path("/api/v4/user"))
        .and(|req: &wiremock::Request| {
            req.headers.get("Authorization").map(|v| v == "Bearer test-token").unwrap_or(false)
        })
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": 1,
            "username": "testuser",
            "name": "Test User"
        })))
        .mount(&ctx.server)
        .await;

    let user = ctx.client.users.get_current().await.unwrap();
    assert_eq!(user.username, "testuser");
}
