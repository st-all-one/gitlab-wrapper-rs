# Erro: `ValidationError` (422)

Ocorre quando a API do GitLab retorna HTTP 422 — payload da requisição não passou na validação.

## Tratamento Recomendado

```rust
match err.category() {
    Some(ErrorCategory::VALIDATION_ERROR) => {
        // Corrigir payload antes de reenviar
    }
    _ => {}
}
```

## 🔗 Veja também

- [Guia de Erros](./errors.md)

[↑ Voltar ao índice](./errors.md)
