# Erro: `AuthorizationDenied` (403)

Ocorre quando a API do GitLab retorna HTTP 403 — permissões insuficientes para o recurso.

## Tratamento Recomendado

```rust
match err.category() {
    Some(ErrorCategory::AUTHORIZATION_DENIED) => {
        // Permissão negada — verificar scopes do token
    }
    _ => {}
}
```

## 🔗 Veja também

- [Guia de Erros](./errors.md)

[↑ Voltar ao índice](./errors.md)
