# Erro: `RateLimited` (429)

## Como ocorre

O erro `RateLimited` é disparado quando o cliente excede os limites de taxa do GitLab:

1. **Muitas requisições em curto intervalo**: Mais requisições que o limite por segundo.
2. **Paginação rápida**: Múltiplas páginas sem pausa.
3. **Múltiplos clientes**: Várias instâncias compartilhando o mesmo token/IP.

## Exemplo

```rust
match gl.projects.list(None) {
    Err(GitLabError::RateLimited { retry_after, context }) => {
        let wait = retry_after.unwrap_or(60);
        eprintln!("Rate limit excedido. Aguardando {}s...", wait);
        std::thread::sleep(std::time::Duration::from_secs(wait));
        // Retentar
    }
    other => { /* tratar */ }
}
```

## O que fazer

1. **Ajuste `max_rps`**: Configure conforme os limites da sua instância.
2. **Implemente backoff exponencial**: Espere tempo crescente entre retentativas.
3. **Distribua requisições**: Evite rajadas — espaçe chamadas uniformemente.
4. **Use autenticação**: Requisições autenticadas têm limites mais altos.

## 🧠 Nota

O wrapper aplica rate limiting no **lado cliente** via sliding window de 1 segundo.
Quando o limite do servidor é excedido, o erro `RateLimited` é propagado para que
a aplicação decida como retentar. Reintentar sem backoff agrava o problema.

## 🔗 Veja também

- [Guia de Erros](./errors.md)
- [Getting Started: Configuração](../getting-started.md#configuração)

[↑ Voltar ao índice](./errors.md)
