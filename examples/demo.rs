use gitlab_wrapper::{
    AuthMethod, GitLabClient, GitLabConfig,
    oauth::{self, AuthCodeUrlOptions},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // ── Configuração ────────────────────────────────────────────────────
    let client = GitLabClient::new(GitLabConfig {
        base_url: std::env::var("GITLAB_URL").unwrap_or_else(|_| "https://gitlab.com".into()),
        token: Some(
            std::env::var("GITLAB_TOKEN").expect("GITLAB_TOKEN environment variable required"),
        ),
        auth_method: Some(AuthMethod::Bearer),
        ..Default::default()
    })?;

    tracing::info!("GitLab client created for {}", client.config().base_url);

    // ── Projetos ────────────────────────────────────────────────────────
    let projects = client.projects.list(None).await?;
    tracing::info!("Projects: {}", projects.len());

    if let Some(project) = projects.first() {
        tracing::info!("First project: {} (id={})", project.name, project.id);
    }

    // ── Usuário atual ──────────────────────────────────────────────────
    let current_user = client.users.get_current().await?;
    tracing::info!("Authenticated as: {}", current_user.username);

    // ── Grupos ──────────────────────────────────────────────────────────
    let groups = client.groups.list(None).await?;
    tracing::info!("Groups: {}", groups.len());

    // ── Issues ──────────────────────────────────────────────────────────
    let issues = client.issues.list(None).await?;
    tracing::info!("Issues (global): {}", issues.len());

    // ── Merge Requests ──────────────────────────────────────────────────
    let mrs = client.merge_requests.list(None).await?;
    tracing::info!("Merge requests (global): {}", mrs.len());

    // ── oauth helpers ───────────────────────────────────────────────────
    let verifier = oauth::generate_code_verifier();
    let challenge = oauth::generate_code_challenge(&verifier);
    let auth_url = oauth::authorization_code_url(&AuthCodeUrlOptions {
        base_url: "https://gitlab.com".into(),
        client_id: "your-client-id".into(),
        redirect_uri: "https://app.example.com/callback".into(),
        scope: "api read_user".into(),
        state: "random-state".into(),
        code_challenge: Some(challenge),
    });
    tracing::info!("OAuth authorization URL: {}", auth_url);

    Ok(())
}
