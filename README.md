<div align="center">

# gitlab-wrapper-rs

**Wrapper Rust tipado para a API REST do GitLab v4 — síncrono, seguro, zero custo**

[![License: MPL 2.0](https://img.shields.io/badge/License-MPL_2.0-2b3a42?style=for-the-badge)](https://opensource.org/licenses/MPL-2.0)
[![Rust](https://img.shields.io/badge/Rust-1.85%2B-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org)
[![Made in Brazil](https://img.shields.io/badge/Made_in-Brazil-009739?style=for-the-badge)](https://github.com/topics/brazil)

</div>

**`gitlab-wrapper-rs`** é um wrapper puramente back-end para a API REST do GitLab v4, construído em **Rust síncrono (blocking)** com foco em segurança de tipos, rastreabilidade e mínimo de dependências. Cada instância é isolada, imutável e `Send + Sync` — pode ser compartilhada entre threads sem risco de contaminação de estado.

---

## 📖 Documentação

- [Guia de Uso](./wiki/usage-guide.md) — Exemplos completos para todos os 25 recursos
- [Getting Started](./wiki/getting-started.md) — Instalação, configuração, primeira chamada
- [Guia de Integração](./wiki/integration-guide.md) — DI, retry, cache, OAuth, async wrappers
- [Particularidades da API](./wiki/particularities.md) — `id` vs `iid`, encoding, sudo, blocking vs async
- [Referência da API](./wiki/api-reference.md) — Lista completa de structs e métodos
- [Catálogo de Erros](./wiki/error/errors.md) — Erros RFC 7807 com UUID v7

---

## 🚀 Quick-start

### Adicione ao `Cargo.toml`:

```bash
cargo add gitlab-wrapper-rs
```

Ou manualmente:

```toml
[dependencies]
gitlab-wrapper-rs = "0.1"
log = "0.4"              # opcional — logs internos
env_logger = "0.11"      # opcional — exibir logs no terminal
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
// Projetos onde sou membro
let projects = gl.projects.list(None)?;
for p in &projects {
    println!("{}: {}", p.id, p.name);
}

// Issues abertas de um projeto
let issues = gl.issues.list_for_project(42, Some(&IssueFilter {
    state: Some("opened".into()),
    ..Default::default()
}))?;

// Criar issue
let issue = gl.issues.create(42, &CreateIssuePayload {
    title: "Bug encontrado".into(),
    description: Some("Passos para reproduzir...".into()),
    labels: Some("bug,prioridade-alta".into()),
    ..Default::default()
})?;
```

### Tratamento de erros (RFC 7807):

```rust
use gitlab_wrapper::{ErrorCategory, GitLabError};

fn handle(gl: &GitLabClient) -> Result<(), GitLabError> {
    match gl.projects.get(99999) {
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

Todos os **25 recursos** com **~195 métodos** públicos, paridade total com a contraparte TypeScript.

| Resource | Métodos | Endpoints |
|---|---|---|
| **Projects** | `list`, `list_all`, `get`, `create`, `update`, `delete` | `archive`, `unarchive`, `fork`, `transfer` |
| **Groups** | `list`, `get`, `create`, `update`, `delete` | `subgroups`, `descendant_groups`, `projects` |
| **Users** | `list`, `get`, `get_current`, `create`, `update`, `delete` | `status`, `set_status`, `preferences`, `set_preferences`, `deactivate`, `activate`, `ban`, `unban` |
| **Issues** | `list`, `list_for_project`, `get`, `create`, `update`, `delete` | `subscribe`, `unsubscribe`, `move`, `set_time_estimate`, `add_spent_time`, `reset_time_estimate`, `reset_spent_time`, `get_by_group` |
| **Merge Requests** | `list`, `list_for_project`, `get`, `create`, `update`, `delete` | `merge`, `approve`, `unapprove`, `rebase`, `cancel_merge_when_pipeline_succeeds`, `commits`, `changes`, `list_by_group` |
| **Branches** | `list`, `get`, `create`, `delete` | `delete_merged` |
| **Commits** | `list`, `get`, `create` | `cherry_pick`, `revert`, `diff`, `refs`, `comments`, `add_comment` |
| **Tags** | `list`, `get`, `create`, `delete` | — |
| **Repository Files** | `get`, `raw`, `blame`, `create`, `update`, `delete` | — |
| **Wikis** | `list`, `get`, `create`, `update`, `delete` | — |
| **Labels** | CRUD projeto + grupo | `promote_project_label` |
| **Milestones** | CRUD projeto + grupo | `list_*_milestone_issues`, `list_*_milestone_merge_requests` |
| **Members** | CRUD projeto + grupo | `list_*_inherited_members` |
| **Notes** | CRUD *completo* (issue, MR, commit, snippet, wiki) | `get`, `list`, `create`, `update`, `delete` |
| **Discussions** | CRUD *completo* (issue, MR, commit) | `add_note`, `update_note`, `delete_note`, `resolve` |
| **Todos** | `list`, `mark_done` | `mark_all_done` |
| **Search** | `global`, `in_group`, `in_project` | — |
| **Events** | `list`, `list_user_events` | `list_project_events` |
| **Pipelines** | `list`, `get`, `get_latest`, `create`, `delete` | `retry`, `cancel`, `variables`, `test_report`, `test_report_summary` |
| **Jobs** | `list`, `list_by_pipeline`, `get` | `trace`, `artifacts`, `cancel`, `retry`, `play`, `erase` |
| **Pipeline Schedules** | `list`, `get`, `create`, `update`, `delete` | `take_ownership`, `create_variable`, `update_variable`, `delete_variable` |
| **Runners** | `list`, `get`, `create`, `update`, `delete` | `list_jobs` |
| **Releases** | `list`, `get`, `create`, `update`, `delete` | `create_link`, `delete_link` |
| **Deploy Keys** | `list`, `get`, `create`, `update`, `delete` | `enable` |
| **Environments** | `list`, `get`, `create`, `update`, `delete` | `stop` |

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

Todas as ~200 structs de domínio derivam `Serialize + Deserialize` com `#[serde(rename_all = "snake_case")]` e `skip_serializing_if = "Option::is_none"`. Isso garante que apenas campos preenchidos sejam serializados e que o mapeamento JSON-Rust seja exato:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateIssuePayload {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<String>,
    // ...
}
```

### Sistema de Erros RFC 7807 com UUID v7

Todos os erros de API seguem o padrão **Problem Details (RFC 7807)**. Cada erro recebe um **UUID v7** único para correlação distribuída:

```rust
// Exemplo de resposta RFC 7807 serializada:
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

### Paginação Dupla (Offset + Keyset)

Suporte a **offset pagination** (página/por página) e **keyset pagination** (cursor-based via `X-NEXT-CURSOR`):

```rust
// Offset: filtra por página
let projects = gl.projects.list(Some(&ProjectFilter {
    page: Some(1),
    per_page: Some(50),
    ..Default::default()
}))?;

// Auto-paginate eager (ProjectsResource apenas)
let all = gl.projects.list_all(None)?;
```

> **💡 Nota:** Diferente do wrapper TypeScript que oferece iteradores lazy (`AsyncIterableIterator`),
> o Rust implementa paginação **eager** (coleta tudo em `Vec<T>`). Para conjuntos muito grandes,
> use `page`/`per_page` manualmente nos filtros.

### Thread Safety nativo

Toda a biblioteca é `Send + Sync`. O `HttpClient` interno usa `Arc<HttpClient>` e `Mutex<SlidingWindow>`, permitindo compartilhamento seguro entre threads:

```rust
let gl = Arc::new(GitLabClient::new(config)?);

let gl2 = gl.clone();
std::thread::spawn(move || {
    let projects = gl2.projects.list(None)?;
    // ...
});

let users = gl.users.get_current()?; // seguro: Arc compartilha o rate limiter
```

### OAuth 2.0 Completo

Suporte aos fluxos OAuth com **CSPRNG verdadeiro** via `getrandom`:

- **Authorization Code com PKCE** — `generate_code_verifier()`, `generate_code_challenge()`, `authorization_code_url()`, `exchange_authorization_code()`
- **Device Grant** — `request_device_authorization()`, `poll_for_token()`, `get_token()`
- **Token Management** — `refresh_token()`, `revoke_token()`
- Cliente HTTP **compartilhado** via `LazyLock` (reusa conexões TCP)

```rust
use gitlab_wrapper::oauth;

let verifier = oauth::generate_code_verifier();  // 32 bytes CSPRNG → base64url
let challenge = oauth::generate_code_challenge(&verifier); // SHA-256 → base64url
```

### Logs Estruturados via `log` Crate

O wrapper usa a facade `log` — você escolhe o backend (env_logger, tracing, slog, etc.):

```bash
RUST_LOG=gitlab_wrapper=debug cargo run
```

Cada erro de API é registrado automaticamente com `category`, `status`, `operation` e `response_body`.

---

## 🔍 Testes

```bash
cargo test                    # 21 testes unitários + integração com wiremock
cargo test --test client_test # testes do cliente HTTP
cargo test --test oauth_test  # testes OAuth (PKCE, auth URL)
cargo test --test errors_test # testes do sistema de erros
```

> ```
> ❯ cargo test
> running 21 tests
> test result: ok. 21 passed; 0 failed
> ```

---

## 🛠️ Stack

| Componente | Tecnologia |
|---|---|
| Runtime | Rust 1.85+ (edition 2021) |
| HTTP | `reqwest` 0.12 (blocking, rustls-tls) |
| Serialização | `serde` + `serde_json` |
| Erros | `thiserror` — enum com 12 categorias + UUID v7 |
| Logs | `log` crate (facade — escolha o backend) |
| OAuth PKCE | `sha2` + `getrandom` (CSPRNG) |
| UUID | `uuid` v7 (correlação distribuída) |
| Rate limiting | Sliding window manual (~35 linhas, zero dependências) |
| Retry | Manual (você decide como e quando) |
| Licença | MPL-2.0 |

---

## 📊 Comparação com o Wrapper TypeScript

| Aspecto | TypeScript (`@st-all-one/gitlab-wrapper-ts`) | Rust (`gitlab-wrapper-rs`) |
|---|---|---|
| **Runtime** | Deno 2.0 (async) | Rust síncrono (blocking) |
| **HTTP** | `fetch` nativo | `reqwest::blocking` com rustls |
| **Paginação** | Lazy (`AsyncIterableIterator`) | Eager (`Vec<T>`) |
| **Erros** | Classe `GitLabWrapperError` | Enum `GitLabError` com `match` |
| **Null safety** | `undefined` / `null` | `Option<T>` com `unwrap`/`?` |
| **Config** | Objeto `create({...})` | Struct `GitLabConfig + Default` |
| **Resources** | 25 classes lazy via getter | 25 structs via `Deref<ResourceGroup>` |
| **OAuth PKCE** | `crypto.subtle` | `getrandom` CSPRNG |
| **Thread safety** | `EventLoop` single-thread | `Send + Sync` nativo |
| **Testes** | 28 testes (Deno) | 21 testes (wiremock) |
| **Maturidade** | ~195 métodos públicos | ~195 métodos públicos |

---

## ⚠️ Limitações Conhecidas

### OAuth bypassa o rate limiter

As funções auxiliares OAuth (`oauth::exchange_authorization_code`, `oauth::poll_for_token`, `oauth::refresh_token`, `oauth::revoke_token`, `oauth::request_device_authorization`) criam seu próprio cliente HTTP via `LazyLock<Client>` e **não passam pelo rate limiter** do `HttpClient`.

Isso significa que chamadas OAuth não contam para o limite de 10 RPS, mas também **não respeitam** o limite caso você faça muitas chamadas OAuth simultâneas. Na prática, isso raramente é um problema porque:
- Fluxos OAuth são chamados esporadicamente (não em loops)
- Cada fluxo faz no máximo 2-3 requisições HTTP

Se você precisar de rate limiting estrito nas chamadas OAuth, use as funções de mais baixo nível (`generate_code_verifier`, `authorization_code_url`) e faça as requisições HTTP manualmente através do `HttpClient` interno — ou abra uma issue solicitando a integração.

### Paginação eager (não lazy)

Diferente do wrapper TypeScript que oferece iteradores lazy (`AsyncIterableIterator`), o Rust implementa paginação **eager**: `list_all()` carrega tudo em `Vec<T>`. Para conjuntos muito grandes, use `page`/`per_page` manualmente nos filtros.

### Sem suporte a multipart (upload de arquivos)

O `reqwest::blocking` não suporta multipart de forma ergonômica. Métodos como `upload_avatar()` retornam `Err(GitLabError::Config(...))`. Use o cliente HTTP diretamente com `reqwest::blocking::Client::builder()` para uploads.

### Sem suporte nativo a async

Toda a biblioteca é **blocking** (síncrona). Para uso em runtimes async (tokio, async-std), envolva as chamadas em `tokio::task::spawn_blocking()`:

```rust
let gl = GitLabClient::new(config)?;
let projects = tokio::task::spawn_blocking(move || gl.projects.list(None)).await??;
```

---

## 📋 Pré-requisitos

- **Rust 1.85+** (edição 2021) — verifique com `rustc --version`
- **Token de acesso GitLab** — [Personal Access Token](https://docs.gitlab.com/ee/user/profile/personal_access_tokens.html) com escopos `api` ou `read_api`

---

<div align="center">

---

**Licença:** Mozilla Public License v2.0 (MPL-2.0) — veja o arquivo [`LICENSE`](./LICENSE)

</div>
