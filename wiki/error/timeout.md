# Erro: `Timeout` (504)

Ocorre quando a API do GitLab retorna HTTP 504 — requisição excedeu o tempo limite configurado.

## Tratamento Recomendado

```rust
match err.category() {
    Some(ErrorCategory::TIMEOUT) => {
        // Aumentar timeout ou retentar
    }
    _ => {}
}
```

## 🔗 Veja também

- [Guia de Erros](./errors.md)

[↑ Voltar ao índice](./errors.md)
