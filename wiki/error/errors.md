# Sistema de Erros

O `gitlab-wrapper-rs` implementa a especificação **RFC 7807 (Problem Details)** para relatório de erros,
usando o enum `GitLabError` com 12 categorias e UUID v7 para rastreamento.

## Estrutura do Erro

Todo erro de API é da variante `GitLabError::Api`:

| Campo | Tipo | Descrição |
|-------|------|-----------|
| `category` | `ErrorCategory` | Categoria do erro (12 variantes) |
| `status` | `u16` | Código HTTP |
| `detail` | `String` | Descrição legível |
| `instance` | `String` | UUID v7 (ex.: `0194b3e0-7f1a-7d80-8000-123456789abc`) |
| `context` | `ErrorContext` | Contexto adicional |

## UUID v7 para Rastreamento

Cada erro recebe um UUID v7 via `uuid::Uuid::new_v7()`:

```
0194b3e0-7f1a-7d80-8000-123456789abc
```

Isso permite correlacionar erros entre logs, tracing e sistemas de monitoramento.

## 12 Categorias de Erro

| Categoria | HTTP | Descrição | Documentação |
|-----------|------|-----------|--------------|
| `AuthenticationFailed` | 401 | Autenticação inválida | [detalhes](authentication-failed.md) |
| `AuthorizationDenied` | 403 | Permissões insuficientes | [detalhes](authorization-denied.md) |
| `ResourceNotFound` | 404 | Recurso não encontrado | [detalhes](resource-not-found.md) |
| `ValidationError` | 422 | Falha na validação | [detalhes](validation-error.md) |
| `Conflict` | 409 | Conflito de recurso | [detalhes](conflict.md) |
| `RateLimited` | 429 | Limite excedido | [detalhes](rate-limited.md) |
| `SpamDetected` | 400 | Spam detectado | [detalhes](spam-detected.md) |
| `NotModified` | 304 | Não modificado | [detalhes](not-modified.md) |
| `Timeout` | 504 | Timeout excedido | [detalhes](timeout.md) |
| `NetworkError` | 503 | Falha de rede | [detalhes](network-error.md) |
| `ParseError` | 500 | Falha ao parsear resposta | [detalhes](parse-error.md) |
| `InternalError` | 500 | Erro interno do GitLab | [detalhes](internal-error.md) |

## ErrorContext

```rust
pub struct ErrorContext {
    pub operation: Option<String>,       // operação que falhou
    pub http_status: Option<u16>,        // status HTTP original
    pub api_errors: Option<Vec<String>>, // mensagens da API
    pub response_body: Option<String>,   // corpo bruto da resposta
}
```

## Padrão de Tratamento

```rust
use gitlab_wrapper::{ErrorCategory, GitLabError};

fn handle_error(err: GitLabError) {
    match err {
        GitLabError::Api { category, status, detail, instance, .. } => {
            eprintln!("[{}] {} (UUID: {})", status, detail, instance);
            match category {
                ErrorCategory::AuthenticationFailed => {
                    // Reautenticar
                }
                ErrorCategory::RateLimited => {
                    // Aguardar e retentar
                }
                ErrorCategory::ResourceNotFound => {
                    // Recurso não existe
                }
                _ => {} // tratamento genérico
            }
        }
        GitLabError::RateLimited { retry_after, .. } => {
            eprintln!("Rate limited, retry after: {:?}", retry_after);
        }
        GitLabError::Config(msg) => {
            eprintln!("Erro de configuração: {}", msg);
        }
        e => eprintln!("Erro: {}", e),
    }
}
```

## Log Estruturado

```rust
use gitlab_wrapper::GitLabError;

fn log_error(err: &GitLabError) {
    if let GitLabError::Api { category, status, detail, instance, context } = err {
        let entry = serde_json::json!({
            "severity": "error",
            "message": detail,
            "category": format!("{}", category),
            "http_status": status,
            "uuid": instance,
            "operation": context.operation,
            "service": "gitlab-wrapper",
        });
        log::error!("{}", entry);
    }
}
```

## Retry com Backoff Exponencial

Para erros transitórios:

```rust
fn is_retryable(err: &GitLabError) -> bool {
    matches!(err.category(), Some(
        ErrorCategory::RateLimited
        | ErrorCategory::Timeout
        | ErrorCategory::NetworkError
    ))
}

fn with_retry<F, T>(f: F, max: u32) -> Result<T, GitLabError>
where F: Fn() -> Result<T, GitLabError> {
    for attempt in 1..=max {
        match f() {
            Ok(v) => return Ok(v),
            Err(ref e) if is_retryable(e) && attempt < max => {
                std::thread::sleep(std::time::Duration::from_millis(
                    2u64.pow(attempt) * 1000
                ));
            }
            Err(e) => return Err(e),
        }
    }
    unreachable!()
}
```

## 🔗 Veja também

- [Getting Started](../getting-started.md)
- [Usage Guide](../usage-guide.md)

[↑ Voltar ao índice](../index.md)
