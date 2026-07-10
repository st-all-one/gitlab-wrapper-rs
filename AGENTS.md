# gitlab-wrapper-rs — Agent Guide

## TL;DR
```bash
cargo build
cargo test
cargo check --example demo
```

## Stack
- **HTTP**: `reqwest` (blocking, rustls-tls)
- **Serialization**: `serde` + `serde_json`
- **Errors**: `thiserror` (enum-based, RFC 7807-aligned)
- **Logs**: `log` crate (no forced logger — user chooses backend)
- **Correlation IDs**: `uuid` v7
- **Rate limiting**: Manual sliding window (~35 lines), no external dep
- **Retry**: Manual loop, no external dep

## Architecture

```
gitlab_wrapper (lib)
├── lib.rs              # Barrel: re-exports everything
├── client.rs           # GitLabClient (factory, 25 resource fields)
│
├── core/               # Config, errors, constants
│   ├── mod.rs
│   ├── config.rs       # GitLabConfig, ResolvedConfig, AuthMethod
│   ├── errors.rs       # GitLabError (enum), ErrorCategory (12), ErrorContext
│   └── constants.rs    # DEFAULT_TIMEOUT, DEFAULT_MAX_RPS, API_VERSION, etc.
│
├── http/               # HTTP client layer
│   ├── mod.rs
│   ├── client.rs       # HttpClient (reqwest wrapper, auth, rate-limit, retry)
│   ├── pagination.rs   # PaginationInfo, paginate_all, error/retry extraction
│   └── rate_limiter.rs # SlidingWindow (Mutex-guarded VecDeque)
│
├── types/              # All serde types — 1 file per domain + barrel
│   ├── mod.rs          # Barrel: re-exports all types
│   ├── base.rs         # GitLabId, AuthorInfo, Links, TimeStats, PaginationParams
│   ├── project.rs      # Project, ProjectFilter, Create/UpdatePayload
│   ├── group.rs        # Group, GroupFilter, Create/UpdatePayload
│   ├── user.rs         # User, UserStatus, UserFilter, Create/UpdatePayload
│   ├── issue.rs        # Issue, IssueFilter, Create/UpdatePayload
│   ├── merge_request.rs # MergeRequest, MRFilter, MergePayload, etc.
│   ├── branch.rs, commit.rs, tag.rs, repository_file.rs, wiki.rs
│   ├── label.rs, milestone.rs, member.rs, note.rs, discussion.rs
│   ├── todo.rs, search.rs, event.rs, pipeline.rs, job.rs
│   ├── pipeline_schedule.rs, runner.rs, release.rs, deploy_key.rs
│   ├── environment.rs
│   ├── oauth.rs        # OAuthTokenResponse, DeviceAuthResponse, OAuthErrorResponse
│   └── access_levels.rs # NO_ACCESS(0) … OWNER(50) constants
│
├── resources/          # All 25 resource structs — 1 file per resource + barrel
│   ├── mod.rs          # Barrel: re-exports all resource structs
│   ├── projects.rs, groups.rs, users.rs, issues.rs, merge_requests.rs
│   ├── branches.rs, commits.rs, tags.rs, repository_files.rs, wikis.rs
│   ├── labels.rs, milestones.rs, members.rs, notes.rs, discussions.rs
│   ├── todos.rs, search.rs, events.rs, pipelines.rs, jobs.rs
│   ├── pipeline_schedules.rs, runners.rs, releases.rs, deploy_keys.rs
│   └── environments.rs
│
├── oauth/              # Standalone OAuth 2.0 helpers
│   ├── mod.rs          # Barrel + re-exports OAuth types from types::oauth
│   ├── pkce.rs         # generate_code_verifier, generate_code_challenge
│   ├── auth_code.rs    # authorization_code_url, exchange_authorization_code
│   ├── device_grant.rs # request_device_authorization, poll_for_token, get_token
│   └── refresh.rs      # refresh_token, revoke_token
│
└── utils/              # Shared utilities
    ├── mod.rs
    └── encoding.rs     # encode_query_param (percent-encoding), filter_to_query
```

## Code conventions
- MPL-2.0 header on all files
- `snake_case` for functions and variables
- `CamelCase` for types and enums
- All types derive `Debug, Clone, Serialize, Deserialize`
- `#[serde(rename_all = "snake_case")]` on all API types
- Optional fields: `Option<T>` with `#[serde(skip_serializing_if = "Option::is_none")]`
- Resource methods synchronously return `Result<T, GitLabError>`
- Operations string identifier (e.g., `"projects.list"`) for logging
- Filter parameters use `Option<&FilterStruct>` (pass `None` for defaults)
- Inline query building with `Vec<(String, String)>`

## Dependencies (prod)
| Crate | Reason |
|-------|--------|
| `reqwest` | HTTP client with TLS |
| `serde` + `serde_json` | JSON serialization |
| `thiserror` | Error derive macro |
| `uuid` (v7) | Correlation IDs |
| `sha2` | PKCE code challenge (OAuth) |
| `log` | Logging facade |

## Testing
```bash
cargo test                    # all 21 tests
cargo test --test errors_test # error type tests
cargo test --test oauth_test  # OAuth tests
cargo test --test client_test # integration tests (wiremock)
```

## Examples
```bash
GITLAB_TOKEN=xxx cargo run --example demo
```
