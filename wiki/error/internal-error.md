# Erro: `InternalError` (500)

Ocorre quando a API do GitLab retorna HTTP 500 — erro interno do servidor GitLab.

## Tratamento Recomendado

```rust
match err.category() {
    Some(ErrorCategory::INTERNAL_ERROR) => {
        // Pode ser transitório — retentar 1x com backoff
    }
    _ => {}
}
```

## 🔗 Veja também

- [Guia de Erros](./errors.md)

[↑ Voltar ao índice](./errors.md)
