# Erro: `ParseError` (500)

Ocorre quando a API do GitLab retorna HTTP 500 — falha ao interpretar a resposta JSON da API.

## Tratamento Recomendado

```rust
match err.category() {
    Some(ErrorCategory::PARSE_ERROR) => {
        // Verificar versão do wrapper ou reportar bug
    }
    _ => {}
}
```

## 🔗 Veja também

- [Guia de Erros](./errors.md)

[↑ Voltar ao índice](./errors.md)
