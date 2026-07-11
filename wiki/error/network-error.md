# Erro: `NetworkError` (503)

Ocorre quando a API do GitLab retorna HTTP 503 — falha de conectividade com a API.

## Tratamento Recomendado

```rust
match err.category() {
    Some(ErrorCategory::NETWORK_ERROR) => {
        // Falha de rede — retentar com backoff exponencial
    }
    _ => {}
}
```

## 🔗 Veja também

- [Guia de Erros](./errors.md)

[↑ Voltar ao índice](./errors.md)
