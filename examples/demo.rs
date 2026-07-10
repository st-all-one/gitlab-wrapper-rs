use gitlab_wrapper::{
    oauth::{self, AuthCodeUrlOptions},
    AuthMethod, GitLabClient, GitLabConfig,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // ── Configuração ────────────────────────────────────────────────────
    let client = GitLabClient::new(GitLabConfig {
        base_url: std::env::var("GITLAB_URL")
            .unwrap_or_else(|_| "https://gitlab.com".into()),
        token: Some(std::env::var("GITLAB_TOKEN")
            .expect("GITLAB_TOKEN environment variable required")),
        auth_method: Some(AuthMethod::Bearer),
        ..Default::default()
    })?;

    log::info!("GitLab client created for {}", client.config().base_url);

    // ── Projetos ────────────────────────────────────────────────────────
    let projects = client.projects.list(None)?;
    log::info!("Projects: {}", projects.len());

    if let Some(project) = projects.first() {
        log::info!("First project: {} (id={})", project.name, project.id);
    }

    // ── Usuário atual ──────────────────────────────────────────────────
    let current_user = client.users.get_current()?;
    log::info!("Authenticated as: {}", current_user.username);

    // ── Grupos ──────────────────────────────────────────────────────────
    let groups = client.groups.list(None)?;
    log::info!("Groups: {}", groups.len());

    // ── Issues ──────────────────────────────────────────────────────────
    let issues = client.issues.list(None)?;
    log::info!("Issues (global): {}", issues.len());

    // ── Merge Requests ──────────────────────────────────────────────────
    let mrs = client.merge_requests.list(None)?;
    log::info!("Merge requests (global): {}", mrs.len());

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
    log::info!("OAuth authorization URL: {}", auth_url);

    Ok(())
}
