# Erro: `NotModified` (304)

Ocorre quando a API do GitLab retorna HTTP 304 — recurso não modificado (requisição condicional).

## Tratamento Recomendado

```rust
match err.category() {
    Some(ErrorCategory::NOT_MODIFIED) => {
        // Usar recurso em cache (se aplicável)
    }
    _ => {}
}
```

## 🔗 Veja também

- [Guia de Erros](./errors.md)

[↑ Voltar ao índice](./errors.md)
