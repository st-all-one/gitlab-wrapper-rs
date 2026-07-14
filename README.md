<div align="center">

# gitlab-wrapper-rs

**Wrapper Rust tipado para a API REST do GitLab v4 — assíncrono, seguro, zero custo**

[![License: MPL 2.0](https://img.shields.io/badge/License-MPL_2.0-2b3a42?style=for-the-badge)](https://opensource.org/licenses/MPL-2.0)
[![Rust](https://img.shields.io/badge/Rust-1.85%2B-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org)
[![Made in Brazil](https://img.shields.io/badge/Made_in-Brazil-009739?style=for-the-badge)](https://github.com/topics/brazil)

</div>

**`gitlab-wrapper-rs`** é um wrapper puramente back-end para a API REST do GitLab v4, construído em **Rust assíncrono (tokio + reqwest)** com foco em segurança de tipos, rastreabilidade via UUID v7 e cobertura completa de todos os 25 recursos da API. Cada instância é isolada, imutável e `Send + Sync` — pode ser compartilhada entre tasks tokio sem risco de contaminação de estado.

---

## 📖 Documentação

- [Guia de Uso](./wiki/usage-guide.md) — Exemplos completos para todos os 25 recursos
- [Getting Started](./wiki/getting-started.md) — Instalação, configuração, primeira chamada
- [Guia de Integração](./wiki/integration-guide.md) — DI, retry, cache, OAuth, axum
- [Particularidades da API](./wiki/particularities.md) — `id` vs `iid`, encoding, sudo, rate limiting
- [Referência da API](./wiki/api-reference.md) — Lista completa de structs e métodos
- [Catálogo de Erros](./wiki/error/errors.md) — Erros RFC 7807 com UUID v7

---

## 🚀 Quick-start

### Adicione ao `Cargo.toml`:

```toml
[dependencies]
gitlab-wrapper-rs = "0.2"
tokio = { version = "1", features = ["macros", "rt"] }
```

### Configuração:

```rust
use gitlab_wrapper::{AuthMethod, GitLabClient, GitLabConfig};

let gl = GitLabClient::new(GitLabConfig {
    base_url: std::env::var("GITLAB_URL")
        .unwrap_or_else(|_| "https://gitlab.com".into()),
    token: Some(std::env::var("GITLAB_TOKEN")
        .expect("GITLAB_TOKEN é obrigatório")),
    auth_method: Some(AuthMethod::Bearer),
    ..Default::default()
})?;
```

### Uso básico:

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gl = /* ... */;

    // Projetos onde sou membro
    let projects = gl.projects.list(None).await?;
    for p in &projects {
        println!("{}: {}", p.id, p.name);
    }

    // Issues abertas de um projeto
    let issues = gl.issues
        .list_for_project(42, Some(&IssueFilter {
            state: Some("opened".into()),
            ..Default::default()
        }))
        .await?;

    // Criar issue
    let issue = gl.issues
        .create(42, &CreateIssuePayload {
            title: "Bug encontrado".into(),
            description: Some("Passos para reproduzir...".into()),
            labels: Some("bug,prioridade-alta".into()),
            ..Default::default()
        })
        .await?;

    Ok(())
}
```

### Tratamento de erros (RFC 7807):

```rust
use gitlab_wrapper::{ErrorCategory, GitLabError};

async fn handle(gl: &GitLabClient) -> Result<(), GitLabError> {
    match gl.projects.get(99999).await {
        Err(GitLabError::Api { category, status, detail, instance, .. }) => {
            eprintln!("[{}] {} (UUID: {})", status, detail, instance);
            match category {
                ErrorCategory::ResourceNotFound => { /* 404 */ }
                ErrorCategory::RateLimited => { /* 429 — aguardar */ }
                ErrorCategory::AuthenticationFailed => { /* 401 — reautenticar */ }
                _ => { /* outros */ }
            }
        }
        Err(GitLabError::Config(msg)) => {
            eprintln!("Erro de configuração: {}", msg);
        }
        Ok(project) => println!("{}", project.name),
        Err(e) => return Err(e),
    }
    Ok(())
}
```

---

## 🌐 Recursos Cobertos

Todos os **25 recursos** com **~195 métodos** públicos.

| Resource | list | get | create | update | delete | Métodos Extras |
|---|---|---|---|---|---|---|
| **Projects** | ✅ | ✅ | ✅ | ✅ | ✅ | `archive`, `unarchive`, `fork`, `transfer`, `list_all` |
| **Groups** | ✅ | ✅ | ✅ | ✅ | ✅ | `subgroups`, `descendant_groups`, `projects` |
| **Users** | ✅ | ✅ | ✅ | ✅ | ✅ | `get_current`, `status`, `set_status`, `preferences`, `deactivate`, `activate`, `ban`, `unban` |
| **Issues** | ✅ | ✅ | ✅ | ✅ | ✅ | `subscribe`, `unsubscribe`, `move`, time tracking, `get_by_group` |
| **Merge Requests** | ✅ | ✅ | ✅ | ✅ | ✅ | `merge`, `approve`, `unapprove`, `rebase`, `commits`, `changes`, `list_by_group` |
| **Branches** | ✅ | ✅ | ✅ | — | ✅ | `delete_merged` |
| **Commits** | ✅ | ✅ | ✅ | — | — | `cherry_pick`, `revert`, `diff`, `refs`, `comments` |
| **Tags** | ✅ | ✅ | ✅ | — | ✅ | — |
| **Repository Files** | — | ✅ | ✅ | ✅ | ✅ | `raw`, `blame` |
| **Wikis** | ✅ | ✅ | ✅ | ✅ | ✅ | — |
| **Labels** | ✅ | ✅ | ✅ | ✅ | ✅ | CRUD projeto + grupo, `promote_project_label` |
| **Milestones** | ✅ | ✅ | ✅ | ✅ | ✅ | CRUD projeto + grupo, list issues/MRs |
| **Members** | ✅ | ✅ | ✅ | ✅ | ✅ | CRUD projeto + grupo, inherited members |
| **Notes** | ✅ | ✅ | ✅ | ✅ | ✅ | CRUD completo (issue, MR, commit, snippet, wiki) |
| **Discussions** | ✅ | ✅ | ✅ | ✅ | ✅ | `add_note`, `update_note`, `delete_note`, `resolve` |
| **Todos** | ✅ | — | — | — | ✅ | `mark_all_done` |
| **Search** | ✅ | — | — | — | — | `global`, `in_group`, `in_project` |
| **Events** | ✅ | — | — | — | — | `list_user_events`, `list_project_events` |
| **Pipelines** | ✅ | ✅ | ✅ | — | ✅ | `retry`, `cancel`, `variables`, `test_report` |
| **Jobs** | ✅ | ✅ | — | — | — | `trace`, `artifacts`, `cancel`, `retry`, `play`, `erase` |
| **Pipeline Schedules** | ✅ | ✅ | ✅ | ✅ | ✅ | `take_ownership`, `create/update/delete_variable` |
| **Runners** | ✅ | ✅ | ✅ | ✅ | ✅ | `list_jobs` |
| **Releases** | ✅ | ✅ | ✅ | ✅ | ✅ | `create_link`, `delete_link` |
| **Deploy Keys** | ✅ | ✅ | ✅ | ✅ | ✅ | `enable` |
| **Environments** | ✅ | ✅ | ✅ | ✅ | ✅ | `stop` |

---

## 🎯 Principais Características

### Imutabilidade e Isolamento

Cada instância é criada com `GitLabClient::new()` e sua `ResolvedConfig` é **imutável** — não há `set_token()` ou `set_sudo()`. Para usar configurações diferentes, crie uma nova instância. Isso permite que múltiplas partes do sistema operem com credenciais e servidores diferentes **simultaneamente** sem risco de contaminação de estado global.

```rust
let gl_admin = GitLabClient::new(GitLabConfig {
    base_url: url.clone(), token: Some(admin_token), sudo: Some("joao".into()),
    ..Default::default()
})?;
let gl_user  = GitLabClient::new(GitLabConfig {
    base_url: url, token: Some(user_token),
    ..Default::default()
})?;
```

### Segurança de Tipos com `serde`

Todas as ~200 structs de domínio derivam `Serialize + Deserialize` com `#[serde(rename_all = "snake_case")]` e `skip_serializing_if = "Option::is_none"`. Isso garante que apenas campos preenchidos sejam serializados e que o mapeamento JSON-Rust seja exato.

### Sistema de Erros RFC 7807 com UUID v7

Todos os erros de API seguem o padrão **Problem Details (RFC 7807)**. Cada erro recebe um **UUID v7** único para correlação distribuída:

```json
{
    "category": "resource-not-found",
    "status": 404,
    "detail": "Project with id=99999 not found",
    "instance": "0194b3e0-7f1a-7d80-8000-123456789abc",
    "context": {
        "operation": "projects.get",
        "http_status": 404
    }
}
```

12 categorias mapeadas diretamente para status HTTP via `ErrorCategory::from_status()`.

### Async nativo (tokio + reqwest)

Toda a biblioteca é **assíncrona** — integra-se nativamente com axum, actix e outros frameworks async sem `spawn_blocking`:

```rust
use axum::{extract::State, routing::get, Json, Router};
use std::sync::Arc;

async fn listar_projetos(State(client): State<Arc<GitLabClient>>) -> Json<serde_json::Value> {
    match client.projects.list(None).await {
        Ok(projects) => Json(serde_json::json!({ "projects": projects })),
        Err(e) => Json(serde_json::json!({ "error": e.to_string() })),
    }
}

#[tokio::main]
async fn main() {
    let client = Arc::new(GitLabClient::new(/* ... */).unwrap());
    let app = Router::new()
        .route("/projects", get(listar_projetos))
        .with_state(client);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

### Rate Limiting (Semáforo assíncrono)

Proteção client-side contra sobrecarga do servidor usando `tokio::sync::Semaphore`. Padrão: **10 requisições/s**, configurável via `max_rps`.

```rust
let gl = GitLabClient::new(GitLabConfig {
    base_url: url,
    token: Some(token),
    max_rps: Some(5),  // máximo 5 req/s
    ..Default::default()
})?;
```

### Paginação Automática

Métodos de listagem com `list_all()` fazem **auto-paginação eager** e retornam `Vec<T>` com todos os registros. Paginação offset e keyset suportadas.

### Thread Safety nativo

Toda a biblioteca é `Send + Sync`. O `HttpClient` interno é `Clone` (usa `Arc<Semaphore>` internamente), permitindo compartilhamento seguro entre tasks tokio:

```rust
let gl = Arc::new(GitLabClient::new(config)?);

let c = Arc::clone(&gl);
tokio::spawn(async move {
    let projects = c.projects.list(None).await.unwrap();
    // ...
});

let users = gl.users.get_current().await?; // seguro: Arc compartilha o rate limiter
```

### OAuth 2.0 Completo

Suporte aos fluxos OAuth com **CSPRNG verdadeiro** via `getrandom`:

- **Authorization Code com PKCE** — `generate_code_verifier()`, `generate_code_challenge()`, `authorization_code_url()`, `exchange_authorization_code()`
- **Device Grant** — `request_device_authorization()`, `poll_for_token()`, `get_token()`
- **Token Management** — `refresh_token()`, `revoke_token()`

```rust
use gitlab_wrapper::oauth;

let verifier = oauth::generate_code_verifier();  // 32 bytes CSPRNG → base64url
let challenge = oauth::generate_code_challenge(&verifier); // SHA-256 → base64url
```

### Logs Estruturados via `tracing`

O wrapper usa `tracing` — logs estruturados com spans e integração com OpenTelemetry:

```bash
RUST_LOG=gitlab_wrapper=debug cargo run
```

---

## 🔍 Testes

```bash
cargo test                       # 36 testes (unitários + integração com wiremock)
cargo test --test client_test    # testes do cliente HTTP
cargo test --test errors_test    # testes do sistema de erros
cargo test --test oauth_test     # testes OAuth (PKCE, auth URL)
cargo clippy                     # lints (0 warnings)
```

```
❯ cargo test
running 36 tests
test result: ok. 36 passed; 0 failed
```

---

## 🛠️ Stack

| Componente | Tecnologia |
|---|---|
| Runtime | Rust 1.85+ (edition 2024) |
| Async | `tokio` (sync, time) |
| HTTP | `reqwest` 0.12 (async, rustls-tls) |
| Serialização | `serde` + `serde_json` |
| Erros | `thiserror` — enum com 12 categorias + UUID v7 |
| Logs | `tracing` — spans, structured |
| UUID | `uuid` v7 (correlação distribuída) |
| Rate limiting | Semáforo tokio (`tokio::sync::Semaphore`) |
| OAuth PKCE | `sha2` + `getrandom` + `base64` (URL_SAFE_NO_PAD) |
| Retry | Manual (você decide como e quando) |
| Licença | MPL-2.0 |

---

## 📊 Comparação com o Wrapper TypeScript

| Aspecto | TypeScript (`@st-all-one/gitlab-wrapper-ts`) | Rust (`gitlab-wrapper-rs`) |
|---|---|---|
| **Runtime** | Deno 2.0 (async) | Tokio (async nativo) |
| **HTTP** | `fetch` nativo | `reqwest` com rustls |
| **Paginação** | Lazy (`AsyncIterableIterator`) | Eager (`Vec<T>`) |
| **Erros** | Classe `GitLabWrapperError` | Enum `GitLabError` com `match` |
| **Null safety** | `undefined` / `null` | `Option<T>` com `unwrap`/`?` |
| **Config** | Objeto `create({...})` | Struct `GitLabConfig + Default` |
| **Resources** | 25 classes lazy via getter | 25 structs via `Deref<ResourceGroup>` |
| **OAuth PKCE** | `crypto.subtle` | `getrandom` CSPRNG |
| **Thread safety** | `EventLoop` single-thread | `Send + Sync` nativo |
| **Testes** | 28 testes (Deno) | 36 testes (wiremock) |
| **Maturidade** | ~195 métodos públicos | ~195 métodos públicos |

---

## ⚠️ Limitações Conhecidas

### Sem suporte a multipart (upload de arquivos)

O `reqwest` async não suporta multipart de forma ergonômica através deste wrapper.
Métodos como `upload_avatar()` retornam `Err(GitLabError::Config(...))`.
Use o cliente HTTP diretamente com `reqwest::Client` para uploads.

### Paginação eager (não lazy)

Diferente do wrapper TypeScript que oferece iteradores lazy (`AsyncIterableIterator`),
o Rust implementa paginação **eager**: `list_all()` carrega tudo em `Vec<T>`.
Para conjuntos muito grandes, use `page`/`per_page` manualmente nos filtros.

### OAuth bypassa o rate limiter

As funções auxiliares OAuth criam seu próprio cliente HTTP via `LazyLock<Client>` e
**não passam pelo rate limiter** do `HttpClient`. Fluxos OAuth são chamados
esporadicamente, então isso raramente é um problema.

---

## 📋 Pré-requisitos

- **Rust 1.85+** (edição 2024) — verifique com `rustc --version`
- **Token de acesso GitLab** — [Personal Access Token](https://docs.gitlab.com/ee/user/profile/personal_access_tokens.html) com escopos `api` ou `read_api`

---

<div align="center">

---

**Licença:** Mozilla Public License v2.0 (MPL-2.0) — veja o arquivo [`LICENSE`](./LICENSE)

</div>
