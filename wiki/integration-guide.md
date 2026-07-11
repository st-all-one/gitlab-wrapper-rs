# Guia de Integração

Padrões, práticas recomendadas e patterns para integrar o `gitlab-wrapper-rs` em aplicações Rust reais.

---

## Índice

- [Factory Pattern e DI](#factory-pattern-e-di)
- [Retry com Exponential Backoff](#retry-com-exponential-backoff)
- [Estratégias de Cache](#estratégias-de-cache)
- [Dicas de Performance](#dicas-de-performance)
- [Integração OAuth](#integração-oauth)
- [Monitoramento de Erros com UUIDs](#monitoramento-de-erros-com-uuids)
- [Uso com Tokio/Async](#uso-com-tokioasync)

---

## Factory Pattern e DI

### Instância Única do Cliente

Crie **uma única instância** do cliente e reutilize em toda a aplicação.
O `HttpClient` gerencia rate limiting e conexões internamente.

```rust
// lib.rs ou mod.rs — módulo dedicado
use gitlab_wrapper::{GitLabClient, GitLabConfig};
use std::sync::OnceLock;

static GL: OnceLock<GitLabClient> = OnceLock::new();

pub fn get_client() -> &'static GitLabClient {
    GL.get_or_init(|| {
        GitLabClient::new(GitLabConfig {
            base_url: std::env::var("GITLAB_URL")
                .unwrap_or_else(|_| "https://gitlab.com".into()),
            token: Some(std::env::var("GITLAB_TOKEN")
                .expect("GITLAB_TOKEN required")),
            max_rps: Some(10),
            ..Default::default()
        })
        .expect("Failed to create GitLab client")
    })
}
```

### Injeção de Dependência (DI)

Passe `&GitLabClient` como parâmetro para funções e structs:

```rust
use gitlab_wrapper::{GitLabClient, GitLabError, Project, Issue};

struct SyncResult {
    project: Project,
    issues: Vec<Issue>,
}

fn sync_project(gl: &GitLabClient, project_id: u64) -> Result<SyncResult, GitLabError> {
    let project = gl.projects.get(project_id)?;
    let issues = gl.issues.list_for_project(project_id, None)?;
    Ok(SyncResult { project, issues })
}

// Uso
let result = sync_project(&get_client(), 42)?;
```

### Arc<GitLabClient> para Múltiplas Threads

```rust
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gl = Arc::new(GitLabClient::new(GitLabConfig {
        base_url: "https://gitlab.com".into(),
        token: Some(std::env::var("GITLAB_TOKEN")?),
        ..Default::default()
    })?);

    let gl_clone = gl.clone();
    std::thread::spawn(move || {
        let projects = gl_clone.projects.list(None).unwrap();
        println!("Thread: {} projetos", projects.len());
    });

    let projects = gl.projects.list(None)?;
    println!("Main: {} projetos", projects.len());
    Ok(())
}
```

---

## Retry com Exponential Backoff

O cliente **não implementa retry automático**. Implemente seu próprio wrapper:

```rust
use gitlab_wrapper::{ErrorCategory, GitLabError};
use std::time::Duration;

fn with_retry<T>(
    f: impl Fn() -> Result<T, GitLabError>,
    max_retries: u32,
) -> Result<T, GitLabError> {
    for attempt in 1..=max_retries {
        match f() {
            Ok(val) => return Ok(val),
            Err(ref e) => {
                let retryable = matches!(e.category(), Some(
                    ErrorCategory::RateLimited
                    | ErrorCategory::Timeout
                    | ErrorCategory::NetworkError
                ));

                if !retryable || attempt == max_retries {
                    return Err(e); // propaga mantendo o erro original
                }

                let wait_ms = 2u64.pow(attempt) * 1000;
                log::warn!(
                    "Tentativa {}/{} falhou. Aguardando {}ms...",
                    attempt, max_retries, wait_ms,
                );
                std::thread::sleep(Duration::from_millis(wait_ms));
            }
        }
    }
    unreachable!()
}

// Uso
let project = with_retry(|| gl.projects.get(42), 3)?;
```

### Tabela de Decisão de Retry

| Categoria | Retentar? | Estratégia |
|-----------|-----------|------------|
| `RateLimited` | ✅ Sim | Backoff exponencial (2^tentativa * 1s) |
| `Timeout` | ✅ Sim | Tentar imediatamente |
| `NetworkError` | ✅ Sim | Backoff exponencial (2^tentativa * 500ms) |
| `AuthenticationFailed` | ❌ Não | Token inválido — ação manual |
| `AuthorizationDenied` | ❌ Não | Permissão insuficiente |
| `ResourceNotFound` | ❌ Não | Recurso não existe |
| `ValidationError` | ❌ Não | Payload incorreto |
| `Conflict` | ⚠️ Talvez | Depende do caso de uso |
| `InternalError` | ⚠️ Talvez | Máx 1 retry |

---

## Estratégias de Cache

### Cache Simples com HashMap

```rust
use std::collections::HashMap;
use gitlab_wrapper::{GitLabClient, Project};

struct ProjectCache {
    cache: HashMap<u64, Project>,
}

impl ProjectCache {
    fn new() -> Self { Self { cache: HashMap::new() } }

    fn get(&mut self, gl: &GitLabClient, id: u64) -> Result<&Project, gitlab_wrapper::GitLabError> {
        if !self.cache.contains_key(&id) {
            let project = gl.projects.get(id)?;
            self.cache.insert(id, project);
        }
        Ok(self.cache.get(&id).unwrap()) // seguro: acabamos de inserir
    }
}
```

### Cache com TTL (usando std::time)

```rust
use std::collections::HashMap;
use std::time::{Duration, Instant};

struct TimedCache<T> {
    data: HashMap<u64, (T, Instant)>,
    ttl: Duration,
}

impl<T> TimedCache<T> {
    fn new(ttl: Duration) -> Self { Self { data: HashMap::new(), ttl } }

    fn get(&mut self, id: u64) -> Option<&T> {
        let (val, inserted_at) = self.data.get(&id)?;
        if inserted_at.elapsed() > self.ttl {
            self.data.remove(&id);
            return None;
        }
        Some(val)
    }

    fn set(&mut self, id: u64, val: T) {
        self.data.insert(id, (val, Instant::now()));
    }
}
```

### Cache de Listas

Evite cachear listas paginadas inteiras. Cacheie itens individuais:

```rust
// ✅ Bom: cacheia projetos individuais
let project = gl.projects.get(42)?;

// ❌ Ruim: listas inteiras ficam obsoletas rapidamente
let all = gl.projects.list_all(None)?;
```

---

## Dicas de Performance

### 1. Use Filtros Específicos

Sempre filtre no servidor, nunca em memória:

```rust
// ✅ Bom: filtra no servidor
let issues = gl.issues.list_for_project(1, Some(&IssueFilter {
    state: Some("opened".into()),
    labels: Some("bug".into()),
    ..Default::default()
}))?;

// ❌ Ruim: baixa tudo e filtra em Rust
let all = gl.issues.list_for_project(1, None)?;
let bugs: Vec<_> = all.into_iter()
    .filter(|i| i.state.as_deref() == Some("opened"))
    .collect();
```

### 2. Configure o Rate Limit Adequadamente

| Ambiente | Limite Recomendado (`max_rps`) |
|----------|-------------------------------|
| GitLab.com (autenticado) | 10 req/s |
| GitLab.com (anônimo) | 5 req/s |
| Self-managed (padrão) | 10 req/s |
| Self-managed (configurado) | Consultar admin |

```rust
let gl = GitLabClient::new(GitLabConfig {
    base_url: "https://gitlab.com".into(),
    token: Some(token),
    max_rps: Some(10),
    ..Default::default()
})?;
```

### 3. Ajuste o Timeout para Operações Lentas

```rust
use std::time::Duration;

let gl = GitLabClient::new(GitLabConfig {
    base_url: "https://gitlab.internal/api/v4".into(),
    token: Some(token),
    timeout: Some(Duration::from_secs(120)),
    ..Default::default()
})?;
```

### 4. Use `per_page` Eficiente nos Filtros

O valor máximo aceito pela API é 100. Ajuste conforme necessidade:

```rust
// Coletar tudo rapidamente
.filter(Some(&ProjectFilter { per_page: Some(100), ..Default::default() }))?;
```

### 5. Prefira `filter_to_query` a Queries Manuais

As structs de filtro (`ProjectFilter`, `IssueFilter`, etc.) são serializadas
automaticamente para query params via `filter_to_query`. Prefira usá-las:

```rust
// ✅ Correto
gl.projects.list(Some(&ProjectFilter { search: Some("api".into()), ..Default::default() }))?;
```

---

## Integração OAuth

O módulo OAuth suporta os fluxos **Authorization Code** (com PKCE) e **Device Grant**.

### Fluxo Authorization Code (PKCE)

```rust
use gitlab_wrapper::oauth::{
    self, AuthCodeUrlOptions, ExchangeCodeOptions,
};

// 1. Gerar verifier e challenge (PKCE)
let verifier = oauth::generate_code_verifier();
let challenge = oauth::generate_code_challenge(&verifier);

// 2. Montar URL de autorização
let auth_url = oauth::authorization_code_url(&AuthCodeUrlOptions {
    base_url: "https://gitlab.com".into(),
    client_id: "seu-client-id".into(),
    redirect_uri: "https://sua-app.com/callback".into(),
    scope: "api read_user".into(),
    state: "random-state".into(),
    code_challenge: Some(challenge),
});
println!("Acesse: {}", auth_url);

// 3. No callback, trocar código pelo token
let token = oauth::exchange_authorization_code(&ExchangeCodeOptions {
    base_url: "https://gitlab.com".into(),
    client_id: "seu-client-id".into(),
    client_secret: Some("seu-secret".into()),
    code: "code-recebido".into(),
    redirect_uri: "https://sua-app.com/callback".into(),
    code_verifier: Some(verifier),
})?;

println!("Access Token: {}", token.access_token);
println!("Expira em: {}s", token.expires_in);
```

### Fluxo Device Grant (CLI)

```rust
use gitlab_wrapper::oauth::{self, DeviceAuthOptions, PollTokenOptions};

// 1. Solicitar autorização
let device = oauth::request_device_authorization(&DeviceAuthOptions {
    base_url: "https://gitlab.com".into(),
    client_id: "seu-client-id".into(),
    scope: Some("api".into()),
})?;

println!("Acesse {} e insira o código: {}",
    device.verification_uri,
    device.user_code,
);

// 2. Polling automático
let token = oauth::get_token(&GetTokenOptions {
    base_url: "https://gitlab.com".into(),
    client_id: "seu-client-id".into(),
    scope: Some("api".into()),
})?;

println!("Token: {}", token.access_token);
```

### Refresh e Revogação

```rust
use gitlab_wrapper::oauth::{self, RefreshTokenOptions, RevokeTokenOptions};

// Refresh
let new_token = oauth::refresh_token(&RefreshTokenOptions {
    base_url: "https://gitlab.com".into(),
    client_id: "seu-client-id".into(),
    client_secret: Some("seu-secret".into()),
    refresh_token: token.refresh_token.unwrap(),
    scope: Some("api".into()),
})?;

// Revogar
oauth::revoke_token(&RevokeTokenOptions {
    base_url: "https://gitlab.com".into(),
    client_id: "seu-client-id".into(),
    client_secret: Some("seu-secret".into()),
    token: token.access_token,
})?;
```

### Usando Token OAuth no Cliente

```rust
let gl = GitLabClient::new(GitLabConfig {
    base_url: "https://gitlab.com".into(),
    token: Some(token.access_token),
    auth_method: Some(AuthMethod::Bearer),
    ..Default::default()
})?;
```

---

## Monitoramento de Erros com UUIDs

Cada `GitLabError::Api` possui um UUID v7 único no campo `instance`.

### Rastreamento em Logs

```rust
use gitlab_wrapper::GitLabError;

fn log_error(err: &GitLabError) {
    match err {
        GitLabError::Api { category, status, detail, instance, context } => {
            log::error!(
                target: "app::gitlab",
                "{} [{}] {} (UUID: {}) — operation: {}",
                status, category, detail, instance,
                context.operation.as_deref().unwrap_or("N/A"),
            );
        }
        GitLabError::RateLimited { retry_after, .. } => {
            log::warn!("Rate limited, retry after: {:?}", retry_after);
        }
        _ => log::error!("GitLab error: {}", err),
    }
}
```

### Integração com OpenTelemetry/Sentry

```rust
use gitlab_wrapper::{GitLabError, ErrorCategory};

fn report_to_monitoring(err: &GitLabError) {
    if let GitLabError::Api { category, detail, instance, .. } = err {
        // Enviar para seu sistema de monitoramento
        let event = serde_json::json!({
            "uuid": instance,
            "category": format!("{}", category),
            "detail": detail,
            "service": "gitlab-wrapper",
        });
        log::error!("[MONITORING] {}", event);
    }
}
```

---

## Uso com Tokio/Async

O wrapper é **síncrono (blocking)**. Para uso em runtime async como Tokio:

### Opção 1: `spawn_blocking`

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gl = GitLabClient::new(GitLabConfig {
        base_url: "https://gitlab.com".into(),
        token: Some(std::env::var("GITLAB_TOKEN")?),
        ..Default::default()
    })?;

    // Executa chamada blocking em thread separada
    let projects = tokio::task::spawn_blocking(move || {
        gl.projects.list(None)
    })
    .await??;

    println!("{} projetos", projects.len());
    Ok(())
}
```

### Opção 2: Pool de Threads Dedicado

```rust
use std::thread;

struct GitLabPool {
    client: GitLabClient,
}

impl GitLabPool {
    fn new(config: GitLabConfig) -> Result<Self, GitLabError> {
        Ok(Self { client: GitLabClient::new(config)? })
    }

    fn spawn<F, T>(&self, f: F) -> thread::JoinHandle<Result<T, GitLabError>>
    where
        F: FnOnce(&GitLabClient) -> Result<T, GitLabError> + Send + 'static,
        T: Send + 'static,
    {
        let gl = self.client.clone(); // Arc<HttpClient> é clonável
        thread::spawn(move || f(&gl))
    }
}
```

> **💡 Nota:** O `GitLabClient` usa `Arc<HttpClient>` internamente, portanto é barato de clonar.
> Cada chamada HTTP bloqueia a thread atual, mas o rate limiting é compartilhado via `Mutex`.
