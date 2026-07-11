# Particularidades da API do GitLab

Comportamentos, pegadinhas e características específicas da API do GitLab que você precisa conhecer ao usar o `gitlab-wrapper-rs`.

---

## Índice

- [id vs iid](#id-vs-iid)
- [URL Encoding](#url-encoding)
- [Sudo (Impersonação)](#sudo-impersonação)
- [OAuth Scopes](#oauth-scopes)
- [Detecção de Spam](#detecção-de-spam)
- [Rate Limiting](#rate-limiting)
- [Blocking vs Async](#blocking-vs-async)
- [Paginação](#paginação)

---

## id vs iid

Um dos conceitos mais confusos da API do GitLab é a diferença entre `id` e `iid`:

| Identificador | Escopo | Exemplo |
|--------------|--------|---------|
| **`id`** | Global — único em toda instância | `id: 42` |
| **`iid`** | Local — único dentro de um projeto/grupo | `iid: 5` (issue #5 do projeto) |

### Onde isso afeta você

No wrapper Rust, isso é refletido nas assinaturas dos métodos:

```rust
// ✅ Correto: grupos/projetos usam id global
let project = gl.projects.get(42)?;
let group = gl.groups.get(1)?;

// ✅ Correto: issues escopadas a projeto usam iid
let issue = gl.issues.get(1, 5)?;
//   project_id ^  ^ issue_iid (não id global)

// ✅ Correto: MRs também usam iid
let mr = gl.merge_requests.get(1, 7)?;
//                project_id ^  ^ mr_iid

// ✅ Branches/tags usam nome (string)
let branch = gl.branches.get(1, "main")?;
```

### Regra Prática

- **Recursos globais** (projetos, grupos, usuários): use `id` (`u64`)
- **Recursos escopados a projeto** (issues, MRs, milestones): use `iid` (`u32`) + `project_id`
- **Branches, tags, commits**: use o nome (`&str`) ou SHA como identificador

---

## URL Encoding

### Projetos com Path Aninhado

Quando um projeto está em um grupo/subgrupo, o caminho completo deve ser
URL-encoded para usar como identificador. O wrapper oferece `get_by_path`
que codifica automaticamente o `/` do caminho para `%2F`:

```rust
// IDs numéricos sempre funcionam
let project = gl.projects.get(42)?;

// get_by_path codifica '/' → '%2F' automaticamente
let project = gl.projects.get_by_path("group/subgroup/project-name")?;

// O mesmo para grupos
let group = gl.groups.get_by_path("parent/subgroup")?;
```

Para codificação manual de outros parâmetros:

```rust
use gitlab_wrapper::utils::encoding::encode_query_param;
let encoded = encode_query_param("valor com espaços e símbolos");
```

### Arquivos no Repository Files

O caminho do arquivo é codificado automaticamente pelo wrapper:

```rust
// O wrapper codifica o file_path automaticamente com encode_query_param
let file = gl.repository_files.get(1, "docs/README.md", "main")?;
// Internamente: /projects/1/repository/files/docs%2FREADME.md
```

---

## Sudo (Impersonação)

O recurso **Sudo** permite que administradores façam chamadas como se fossem
outro usuário.

```rust
let gl = GitLabClient::new(GitLabConfig {
    base_url: "https://gitlab.com".into(),
    token: Some("token-do-admin".into()),
    sudo: Some("joao.silva".into()),  // nome de usuário alvo
    ..Default::default()
})?;
```

### Requisitos

- ✅ Token de **administrador** da instância GitLab
- ✅ O usuário alvo deve existir
- ❌ Não funciona com tokens de usuário comum

---

## OAuth Scopes

| Scope | Acesso | Uso Típico |
|-------|--------|------------|
| `api` | Completo | Aplicações que precisam de tudo |
| `read_api` | Somente leitura | Dashboards, monitores |
| `read_user` | Ler perfil | Informações do usuário |
| `read_repository` | Ler repositório | Clonagem via API |
| `write_repository` | Escrever repositório | CI/CD |
| `sudo` | Impersonação | Ferramentas admin |

```rust
// Mínimo para operações de leitura
let gl = GitLabClient::new(GitLabConfig {
    base_url: "https://gitlab.com".into(),
    token: Some(token_com_scope_read_api),
    ..Default::default()
})?;
```

> **⚠️ Atenção:** Token com `read_api` não pode criar/atualizar/deletar recursos.
> Você receberá `ErrorCategory::AuthorizationDenied` (403).

---

## Detecção de Spam

O GitLab detecta automaticamente spam em issues, comentários, wikis e descrições.

Quando detectado, a API retorna HTTP 400 com categoria `SpamDetected`:

```rust
match gl.issues.create(1, &CreateIssuePayload { title: "Ganhe dinheiro!!!".into(), ..Default::default() }) {
    Err(GitLabError::Api { category: ErrorCategory::SpamDetected, detail, .. }) => {
        eprintln!("Conteúdo marcado como spam: {}", detail);
        // Se needs_captcha_response=true no response_body,
        // apresente CAPTCHA ao usuário e reenvie
    }
    other => { /* tratar resultado */ }
}
```

---

## Rate Limiting

### Limites Conhecidos

| Ambiente | Limite |
|----------|--------|
| **GitLab.com** (autenticado) | 10 req/s (300 req/min) |
| **GitLab.com** (não autenticado) | 5 req/s (por IP) |
| **Self-managed** | Configurável |

### Como o Cliente Gerencia

O wrapper implementa um **sliding window** local de 1 segundo:

```rust
let gl = GitLabClient::new(GitLabConfig {
    base_url: "https://gitlab.com".into(),
    token: Some(token),
    max_rps: Some(10), // respeita limite do GitLab.com
    ..Default::default()
})?;
```

### Headers de Rate Limit

A API retorna headers `RateLimit-*` nas respostas.
Consulte o `ErrorContext` no erro `RateLimited`:

```rust
Err(GitLabError::RateLimited { retry_after, context }) => {
    // retry_after: segundos sugeridos pela API
    // context.response_body: corpo da resposta com detalhes
}
```

---

## Blocking vs Async

O `gitlab-wrapper-rs` é **síncrono (blocking)**. Todas as chamadas HTTP
bloqueiam a thread atual.

### Para ambientes async (Tokio, async-std):

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gl = GitLabClient::new(/* ... */)?;

    // spawn_blocking libera a thread do runtime
    let projects = tokio::task::spawn_blocking(move || {
        gl.projects.list(None)
    }).await??;
    Ok(())
}
```

### Benefícios do Blocking

- **Zero overhead**: sem runtime async, sem `Future`, sem `Pin`
- **Stack traces limpos**: pilha de chamadas linear e previsível
- **Sem cores mágicas**: `Send + Sync` é trivial com `Arc<Mutex>`
- **Ideal para CLIs, scripts, bg jobs**: processos que não precisam de concorrência massiva

### Quando usar Async

- Servidores HTTP que fazem chamadas ao GitLab durante o request handling
- Aplicações com muitas conexões concorrentes
- Streaming de dados em tempo real (WebSocket, SSE)

Nesses casos, use `spawn_blocking` ou aguarde o lançamento de uma versão async.

---

## Paginação

### Offset Pagination (padrão)

Usa `page` e `per_page`. Disponível em todos os resources via filtros.

```rust
let projects = gl.projects.list(Some(&ProjectFilter {
    per_page: Some(50),
    page: Some(1),
    ..Default::default()
}))?;
```

### Keyset Pagination (cursor)

Usa `id_after` e o header `X-NEXT-CURSOR`. Disponível internamente via
`HttpClient::keyset_paginate_all()`.

```rust
// O método keyset está disponível no HttpClient para uso interno
// Exemplo conceitual:
gl.http.keyset_paginate_all::<Project>("projects", &[], "projects.list_all")?;
```

### Eager vs Lazy

Diferente do wrapper TypeScript que oferece iteradores lazy (`AsyncIterableIterator`),
o Rust implementa paginação **eager**: todos os resultados são coletados em `Vec<T>`.

Para conjuntos muito grandes, use filtros de página manualmente:

```rust
let mut page = 1;
let mut all = Vec::new();
loop {
    let items = gl.projects.list(Some(&ProjectFilter {
        per_page: Some(100),
        page: Some(page),
        ..Default::default()
    }))?;
    if items.is_empty() { break; }
    let count = items.len();
    all.extend(items);
    if count < 100 { break; }
    page += 1;
}
println!("Total: {}", all.len());
```

---

## Diferenças do Wrapper TypeScript

| Aspecto | TS (ref) | Rust |
|---------|----------|------|
| **Runtime** | Deno (async) | Síncrono (blocking) |
| **Paginação** | Lazy (iteradores) | Eager (Vec<T>) |
| **Erros** | Classe `GitLabWrapperError` | Enum `GitLabError` |
| **Null safety** | `undefined` / `null` | `Option<T>` |
| **Serialização** | Tipos `readonly` | `serde` derive |
| **Config** | Objeto posicional | Struct `GitLabConfig` |
| **UUID** | `@std/uuid` v7 | `uuid` crate v7 |
| **Auth** | `header`/`bearer` | `Header`/`Bearer` enum |
| **Pseudo-aleatório** | `crypto.subtle` | `getrandom` (CSPRNG) |
