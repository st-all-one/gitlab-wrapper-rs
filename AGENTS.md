# gitlab-wrapper-rs — Agent Guide

## TL;DR
```bash
cargo build
cargo test
cargo clippy
cargo fmt -- --check
cargo check --example demo
GITLAB_TOKEN=xxx cargo run --example demo
```

## Stack
- **HTTP**: `reqwest` (async, rustls-tls)
- **Serialization**: `serde` + `serde_json`
- **Errors**: `thiserror` (enum-based, RFC 7807-aligned)
- **Logs**: `tracing` (with `log` compat feature)
- **Async runtime**: `tokio` (time, sync)
- **Correlation IDs**: `uuid` v7
- **Rate limiting**: `tokio::sync::Semaphore` (async, ~25 lines)
- **Streams**: `futures` + `tokio-stream`
- **Crypto**: `base64` (URL_SAFE_NO_PAD) + `sha2`

## Architecture

```
gitlab_wrapper (lib)
├── lib.rs              # Barrel: re-exports everything
├── client.rs           # GitLabClient (factory, Deref to ResourceGroup)
│
├── core/               # Config, errors, constants
│   ├── mod.rs
│   ├── config.rs       # GitLabConfig, ResolvedConfig, AuthMethod
│   ├── errors.rs       # GitLabError (enum), ErrorCategory (12), ErrorContext
│   └── constants.rs    # DEFAULT_TIMEOUT, DEFAULT_MAX_RPS, API_VERSION, etc.
│
├── http/               # HTTP client layer (async)
│   ├── mod.rs
│   ├── client.rs       # HttpClient (reqwest async wrapper, auth, rate-limit)
│   ├── pagination.rs   # PaginationInfo, paginate_all, error/retry extraction
│   └── rate_limiter.rs # RateLimiter (tokio::sync::Semaphore-based)
│
├── types/              # All serde types — 1 file per domain + barrel
│   ├── mod.rs
│   ├── base.rs         # GitLabId, AuthorInfo, Links, TimeStats, PaginationParams
│   ├── project.rs … issue.rs … user.rs … merge_request.rs …
│   ├── oauth.rs        # OAuthTokenResponse, DeviceAuthResponse, OAuthErrorResponse
│   └── access_levels.rs
│
├── resources/          # All 25 resource structs — 1 file per resource + barrel
│   ├── mod.rs
│   ├── projects.rs … issues.rs … users.rs … merge_requests.rs …
│   └── environments.rs
│
├── oauth/              # OAuth 2.0 helpers (async)
│   ├── mod.rs
│   ├── pkce.rs         # generate_code_verifier, generate_code_challenge
│   ├── auth_code.rs    # authorization_code_url, exchange_authorization_code
│   ├── device_grant.rs # request_device_authorization, poll_for_token, get_token
│   └── refresh.rs      # refresh_token, revoke_token
│
└── utils/
    ├── mod.rs
    └── encoding.rs     # encode_query_param, filter_to_query
```

## Code conventions
- MPL-2.0 header on all files
- `snake_case` for functions and variables
- `CamelCase` for types and enums
- All types derive `Debug, Clone, Serialize, Deserialize`
- `#[serde(rename_all = "snake_case")]` on all API types
- Optional fields: `Option<T>` with `#[serde(skip_serializing_if = "Option::is_none")]`
- Resource methods return `async fn … -> Result<T, GitLabError>`
- Operations string identifier (e.g., `"projects.list"`) for tracing
- Filter parameters use `Option<&FilterStruct>` (pass `None` for defaults)
- Inline query building with `Vec<(String, String)>`

## Dependencies (prod)
| Crate | Reason |
|-------|--------|
| `reqwest` | HTTP client with TLS |
| `serde` + `serde_json` | JSON serialization |
| `thiserror` | Error derive macro |
| `tracing` | Structured logging |
| `uuid` (v7) | Correlation IDs |
| `tokio` (time, sync) | Async runtime + semaphore |
| `sha2` + `base64` | PKCE code challenge (OAuth) |
| `futures` + `tokio-stream` | Async streams |

## Testing
```bash
cargo test                       # all 36 tests
cargo test --test errors_test    # error type tests
cargo test --test oauth_test     # OAuth tests
cargo test --test client_test    # integration tests (wiremock)
cargo bench                      # criterion benchmarks
```

## Security / linting
```bash
cargo clippy                     # 0 warnings expected
cargo fmt -- --check             # formatting check
cargo audit                      # vulnerability audit
cargo deny check                 # license audit
```
