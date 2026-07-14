# Primeiros Passos

Guia rápido para começar a usar o `gitlab-wrapper-rs` em seus projetos Rust.

---

## Índice

- [Instalação](#instalação)
- [Configuração](#configuração)
- [Primeira Chamada à API](#primeira-chamada-à-api)
- [Tratamento de Erros](#tratamento-de-erros)
- [Paginação](#paginação)
- [Imutabilidade](#imutabilidade)

---

## Instalação

Adicione ao seu `Cargo.toml`:

```toml
[dependencies]
gitlab-wrapper-rs = { git = "https://github.com/st-all-one/gitlab-wrapper-rs" }
tokio = { version = "1", features = ["macros", "rt"] }  # runtime async
tracing-subscriber = { version = "0.3", features = ["env-filter"] }  # logs
```

Ou, se publicado no crates.io:

```toml
[dependencies]
gitlab-wrapper-rs = "0.1"
```

### Feature flags (TLS backend)

Por padrão o crate usa **rustls** (OpenSSL-free). Para usar native-tls (OpenSSL):

```toml
[dependencies]
gitlab-wrapper-rs = { version = "0.1", default-features = false, features = ["native-tls"] }
```

| Feature | Padrão | Descrição |
|---------|--------|-----------|
| `rustls` | ✅ | TLS via `rustls` (Rust puro, sem dependência C) |
| `native-tls` | — | TLS via OpenSSL/native (`native-tls` crate) |

Use `--features native-tls` se sua aplicação já depende de OpenSSL ou precisa de
algoritmos específicos de cipher suite não suportados pelo rustls.

---

## Configuração

### Variáveis de Ambiente

Sempre carregue valores sensíveis de variáveis de ambiente:

```rust
let base_url = std::env::var("GITLAB_URL")
    .unwrap_or_else(|_| "https://gitlab.com".into());
let token = std::env::var("GITLAB_TOKEN")
    .expect("GITLAB_TOKEN é obrigatório");
```

Exemplo de arquivo `.env`:

```bash
GITLAB_URL=https://gitlab.com
GITLAB_TOKEN=glpat-xxxxxxxxxxxx
```

### Criando o Cliente

```rust
use gitlab_wrapper::{AuthMethod, GitLabClient, GitLabConfig};

let gl = GitLabClient::new(GitLabConfig {
    base_url: "https://gitlab.com".into(),
    token: Some("glpat-xxxxxxxxxxxx".into()),
    auth_method: Some(AuthMethod::Bearer),
    ..Default::default()
})?;
```

### Opções de Configuração

| Campo | Tipo | Padrão | Descrição |
|-------|------|--------|-----------|
| `base_url` | `String` | — | URL base da API (ex.: `https://gitlab.com`). Sem `/api/v4`. **Obrigatório.** |
| `token` | `Option<String>` | `None` | Token de autenticação. Opcional para instâncias públicas. |
| `auth_method` | `Option<AuthMethod>` | `Bearer` | `Header` envia `PRIVATE-TOKEN`; `Bearer` envia `Authorization: Bearer`. |
| `sudo` | `Option<String>` | `None` | Nome de usuário para impersonação (requer token de admin). |
| `timeout` | `Option<Duration>` | `30s` | Timeout por requisição. |
| `max_rps` | `Option<u32>` | `10` | Máximo de requisições por segundo (sliding window local). |

> **💡 Dica:** O `base_url` é a URL raiz da instância (ex.: `https://gitlab.com`).
> O wrapper adiciona `/api/v4` automaticamente.

---

## Primeira Chamada à API

### Listar Projetos

```rust
use gitlab_wrapper::GitLabClient;

let gl = GitLabClient::new(GitLabConfig {
    base_url: "https://gitlab.com".into(),
    token: Some(std::env::var("GITLAB_TOKEN").unwrap()),
    ..Default::default()
})?;

let projects = gl.projects.list(None).await?;
println!("Encontrados {} projetos", projects.len());
if let Some(first) = projects.first() {
    println!("Primeiro: {} (ID: {})", first.name, first.id);
}
```

### Buscar um Único Projeto

```rust
// Por ID numérico
let project = gl.projects.get(42).await?;
println!("Projeto: {} (ID: {})", project.name, project.id);

// Por caminho URL-encoded
let project = gl.projects.get_by_path("group/subgroup/my-project").await?;
// Internamente codifica '/' → '%2F' automaticamente
// Equivalente a: /api/v4/projects/group%2Fsubgroup%2Fmy-project
```

O mesmo vale para grupos:

```rust
let group = gl.groups.get_by_path("parent/subgroup").await?;
```

---

## Tratamento de Erros

Todos os erros são do tipo `GitLabError`, um enum que segue o padrão **RFC 7807**:

```rust
use gitlab_wrapper::{ErrorCategory, GitLabClient, GitLabError};

async fn exemplo(gl: &GitLabClient) -> Result<(), GitLabError> {
    match gl.projects.get(99999).await? {
        Ok(project) => println!("{}", project.name),
        Err(GitLabError::Api { category, status, detail, instance, .. }) => {
            eprintln!("[{}] {} (UUID: {})", status, detail, instance);
            match category {
                ErrorCategory::ResourceNotFound => {
                    // Recurso não existe — tratamento específico
                }
                ErrorCategory::AuthenticationFailed => {
                    // Token inválido — reautenticar
                }
                ErrorCategory::RateLimited => {
                    // Aguardar e tentar novamente
                }
                _ => {} // outros erros
            }
        }
        Err(e) => return Err(e), // Http, Config, etc.
    }
    Ok(())
}
```

### Categorias de Erro

| Categoria | HTTP Status | Descrição |
|-----------|-------------|-----------|
| `AuthenticationFailed` | 401 | Token inválido ou expirado |
| `AuthorizationDenied` | 403 | Permissões insuficientes |
| `ResourceNotFound` | 404 | Recurso inexistente |
| `ValidationError` | 422 | Payload inválido |
| `Conflict` | 409 | Conflito de estado |
| `RateLimited` | 429 | Limite excedido |
| `SpamDetected` | 400 | Conteúdo marcado como spam |
| `NotModified` | 304 | Recurso não modificado |
| `Timeout` | 504 | Requisição excedeu o timeout |
| `NetworkError` | 503 | Falha de rede |
| `ParseError` | 500 | Falha ao interpretar resposta |
| `InternalError` | 500 | Erro interno do GitLab |

---

## Paginação

O wrapper oferece dois modos de paginação, ambos **eager** (carregam tudo na memória).

### Paginação Offset (padrão)

```rust
// Lista simples — primeira página apenas
let projects = gl.projects.list(None).await?;

// Auto-paginar todas as páginas (apenas em ProjectsResource)
let all_projects = gl.projects.list_all(None).await?;
```

Para paginar manualmente, use `ProjectFilter`:

```rust
use gitlab_wrapper::ProjectFilter;

let projects = gl.projects
    .list(Some(&ProjectFilter {
        per_page: Some(100),
        page: Some(2),
        ..Default::default()
    }))
    .await?;
```

### Paginação Keyset (cursor)

Disponível via `HttpClient::keyset_paginate_all()` para uso interno nos resources
(implementado, mas não exposto publicamente nos resources).

> **⚠️ Nota:** Diferente do wrapper TypeScript que oferece iteradores lazy (`AsyncIterableIterator`),
> o Rust implementa paginação **eager** (coleta tudo em `Vec<T>`). Para conjuntos muito grandes,
> considere usar os filtros de página manualmente com `page` e `per_page`.

---

## Imutabilidade

> **⚠️ Importante:** Uma vez criado, `GitLabClient` é **imutável**. A `ResolvedConfig` interna
> é imutável. Para alterar configurações (como `sudo`), crie uma nova instância.

```rust
// ✅ Correto: criar nova instância
let gl_admin = GitLabClient::new(GitLabConfig {
    base_url: "https://gitlab.com".into(),
    token: Some(admin_token.clone()),
    sudo: Some("joao".into()),
    ..Default::default()
})?;

let gl_normal = GitLabClient::new(GitLabConfig {
    base_url: "https://gitlab.com".into(),
    token: Some(user_token),
    max_rps: Some(5),
    ..Default::default()
})?;
```

---

## Próximos Passos

- [Guia de Uso](usage-guide.md) — exemplos práticos para todos os 93 resources
- [Guia de Integração](integration-guide.md) — padrões, DI, retry, cache, OAuth
- [Particularidades da API](particularities.md) — `id` vs `iid`, encoding, sudo, rate limiting
- [Referência da API](api-reference.md) — documentação completa de structs e métodos
