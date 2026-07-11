# Erro: `SpamDetected` (400)

Ocorre quando a API do GitLab retorna HTTP 400 — conteúdo detectado como spam (requer CAPTCHA).

## Tratamento Recomendado

```rust
match err.category() {
    Some(ErrorCategory::SPAM_DETECTED) => {
        // Apresentar CAPTCHA ao usuário ou revisar conteúdo
    }
    _ => {}
}
```

## 🔗 Veja também

- [Guia de Erros](./errors.md)

[↑ Voltar ao índice](./errors.md)
