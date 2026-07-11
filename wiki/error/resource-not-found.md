# Erro: `ResourceNotFound` (404)

## Como ocorre

O erro `ResourceNotFound` é lançado quando o recurso solicitado não existe ou não está acessível:

1. **ID incorreto**: O identificador fornecido não corresponde a nenhum recurso.
2. **Confusão `id` vs `iid`**: Uso do ID global quando o escopo do projeto exige `iid`.
3. **Problemas de encoding**: Caminhos URL-encoded incorretos (ex.: grupos aninhados).
4. **Recurso foi excluído**: Existia na listagem mas foi removido antes do acesso.

## Exemplo

```rust
match gl.projects.get(99999) {
    Err(GitLabError::Api { category: ErrorCategory::ResourceNotFound, detail, instance, .. }) => {
        eprintln!("Projeto não encontrado: {} (UUID: {})", detail, instance);
        // Retornar null ou 404 para o cliente
    }
    other => { /* tratar */ }
}
```

## O que fazer

1. **Verifique o ID**: Confirme se o identificador está correto.
2. **Diferencie `id` de `iid`**: Issues e MRs usam `iid` (`u32`) dentro do projeto.
3. **Verifique encoding**: Caminhos com grupos aninhados precisam de URL-encoding.
4. **Considere ambiguidade**: 404 pode ser "não encontrado" ou "sem acesso" (por segurança).

## 🔗 Veja também

- [Guia de Erros](./errors.md)
- [Particularidades: id vs iid](../particularities.md#id-vs-iid)

[↑ Voltar ao índice](./errors.md)
