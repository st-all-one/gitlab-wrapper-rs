# Erro: `AuthenticationFailed` (401)

Ocorre quando a API do GitLab retorna HTTP 401 — token inválido, ausente ou expirado.

## Tratamento Recomendado

```rust
match err.category() {
    Some(ErrorCategory::AUTHENTICATION_FAILED) => {
        // Reautenticar — token inválido ou expirado
    }
    _ => {}
}
```

## 🔗 Veja também

- [Guia de Erros](./errors.md)

[↑ Voltar ao índice](./errors.md)
