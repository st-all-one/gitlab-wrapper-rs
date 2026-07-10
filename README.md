# gitlab-wrapper-rs

Typed Rust wrapper for the GitLab REST API v4.

## Features

- **25 GitLab resources**: projects, groups, users, issues, merge requests, pipelines, jobs, and more
- **Typed API**: all request/response types with `serde` `snake_case` serialization
- **Authentication**: PRIVATE-TOKEN header or Bearer token
- **Sudo support**: impersonate users via `SUDO` header
- **Rate limiting**: built-in sliding window (configurable)
- **Error handling**: typed `GitLabError` enum with categories (RFC 7807-aligned)
- **Correlation IDs**: UUID v7 on every API error
- **Logging**: via `log` crate facade (you choose the backend)
- **OAuth 2.0 helpers**: PKCE, authorization code, device grant, token refresh
- **Pagination**: automatic `list_all()` for collection resources
- **Minimal dependencies**: 6 production crates
- **Zero async required**: blocking HTTP client

## Quick Start

```rust
use gitlab_wrapper::{GitLabClient, GitLabConfig};

let client = GitLabClient::new(GitLabConfig {
    base_url: "https://gitlab.com".into(),
    token: std::env::var("GITLAB_TOKEN").unwrap(),
    ..Default::default()
})?;

let projects = client.projects.list(None)?;
println!("Found {} projects", projects.len());

let user = client.users.get_current()?;
println!("Authenticated as: {}", user.username);
```

## Configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `base_url` | `String` | — | GitLab instance URL (e.g. `https://gitlab.com`) |
| `token` | `String` | — | Personal/Project/Group access token |
| `auth_method` | `AuthMethod` | `Bearer` | `Header` (PRIVATE-TOKEN) or `Bearer` |
| `sudo` | `Option<String>` | `None` | Impersonate user (admin only) |
| `timeout` | `Option<Duration>` | 30s | Request timeout |
| `max_rps` | `Option<u32>` | 10 | Max requests per second |

## Resources

| Resource | Key methods |
|----------|------------|
| `projects` | `list`, `get`, `create`, `update`, `delete`, `archive`, `fork`, `transfer` |
| `groups` | `list`, `get`, `create`, `delete`, `subgroups`, `projects` |
| `users` | `list`, `get`, `get_current`, `create`, `status`, `set_status` |
| `issues` | `list`, `get`, `create`, `update`, `delete`, `subscribe`, `move` |
| `merge_requests` | `list`, `get`, `create`, `update`, `merge`, `approve`, `commits` |
| `pipelines` | `list`, `get`, `create`, `retry`, `cancel`, `variables` |
| `jobs` | `list`, `get`, `trace`, `cancel`, `retry`, `play`, `erase`, `artifacts` |
| `branches` | `list`, `get`, `create`, `delete`, `delete_merged` |
| `commits` | `list`, `get`, `create`, `cherry_pick`, `revert`, `diff` |
| `tags` | `list`, `get`, `create`, `delete` |
| `repository_files` | `get`, `raw`, `blame`, `create`, `update`, `delete` |
| `wikis` | `list`, `get`, `create`, `update`, `delete` |
| `labels` | CRUD for project and group labels |
| `milestones` | CRUD for project and group milestones |
| `members` | CRUD for project and group members |
| `notes` | CRUD for issue, MR, commit, and snippet notes |
| `discussions` | CRUD for issue, MR, and commit discussions |
| `todos` | `list`, `mark_done`, `mark_all_done` |
| `search` | `global`, `in_group`, `in_project` |
| `events` | `list`, `list_user_events`, `list_project_events` |
| `releases` | CRUD + release links |
| `runners` | CRUD + `list_jobs` |
| `pipeline_schedules` | CRUD + variables |
| `deploy_keys` | CRUD + `enable` |
| `environments` | CRUD + `stop` |

## Pagination

```rust
// Manual pagination
let page = client.projects.list(Some(&ProjectFilter {
    page: Some(2),
    per_page: Some(50),
    ..Default::default()
}))?;

// Auto-paginate all pages (available on ProjectsResource)
let all = client.projects.list_all(None)?;
```

## Error Handling

```rust
match client.projects.get(999) {
    Err(GitLabError::Api { category, status, detail, instance, .. }) => {
        eprintln!("API error: {category} (HTTP {status}): {detail}");
        eprintln!("Correlation ID: {instance}");
    }
    Err(GitLabError::RateLimited { retry_after, .. }) => {
        eprintln!("Rate limited, retry after {retry_after:?}s");
    }
    Ok(project) => { /* ... */ }
    Err(e) => eprintln!("Other error: {e}"),
}
```

## OAuth 2.0

```rust
use gitlab_wrapper::oauth;

// PKCE helpers
let verifier = oauth::generate_code_verifier();
let challenge = oauth::generate_code_challenge(&verifier);

// Authorization code URL
let url = oauth::authorization_code_url(&AuthCodeUrlOptions {
    base_url: "https://gitlab.com".into(),
    client_id: "my-app".into(),
    redirect_uri: "https://app.example.com/callback".into(),
    scope: "api read_user".into(),
    state: "random".into(),
    code_challenge: Some(challenge),
});

// Exchange code for token
let token = oauth::exchange_authorization_code(&ExchangeCodeOptions {
    base_url: "https://gitlab.com".into(),
    client_id: "my-app".into(),
    client_secret: None,
    code: "auth-code".into(),
    redirect_uri: "https://app.example.com/callback".into(),
    code_verifier: Some(verifier),
})?;

// Refresh token
let new_token = oauth::refresh_token(&RefreshTokenOptions {
    base_url: "https://gitlab.com".into(),
    client_id: "my-app".into(),
    client_secret: None,
    refresh_token: token.refresh_token.unwrap(),
    scope: None,
})?;
```

## Logging

The library uses `log` crate. Configure any logger backend:

```rust
// In your binary:
env_logger::init();

// Or with filter:
std::env::set_var("RUST_LOG", "gitlab_wrapper=debug");
env_logger::init();
```

## License

MPL-2.0
