# Erro: `Conflict` (409)

Ocorre quando a API do GitLab retorna HTTP 409 — conflito com o estado atual do recurso (ex.: branch com mesmo nome).

## Tratamento Recomendado

```rust
match err.category() {
    Some(ErrorCategory::CONFLICT) => {
        // Resolver conflito antes de tentar novamente
    }
    _ => {}
}
```

## 🔗 Veja também

- [Guia de Erros](./errors.md)

[↑ Voltar ao índice](./errors.md)
