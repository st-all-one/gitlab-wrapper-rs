# gitlab-wrapper-rs — Documentação

**gitlab-wrapper-rs** é um wrapper fortemente tipado para a API REST do GitLab v4,
construído em Rust assíncrono (tokio) com foco em segurança, rastreabilidade e zero custo de abstração.

## Índice

| Seção | Descrição |
|-------|-----------|
| [**Getting Started**](./getting-started.md) | Instalação, configuração, primeira chamada |
| [**Guia de Uso**](./usage-guide.md) | Exemplos completos para todos os 25 resources |
| [**Guia de Integração**](./integration-guide.md) | Padrões, DI, retry, cache, OAuth |
| [**Particularidades da API GitLab**](./particularities.md) | `id` vs `iid`, encoding, sudo, rate limiting |
| [**Referência da API**](./api-reference.md) | Lista completa de tipos, structs e métodos |
| [**Erros**](./error/errors.md) | Catálogo de erros RFC 7807 |

## Exemplo Rápido

```rust
use gitlab_wrapper::{CreateIssuePayload, GitLabClient, GitLabConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gl = GitLabClient::new(GitLabConfig {
        base_url: "https://gitlab.com".into(),
        token: Some(std::env::var("GITLAB_TOKEN")?),
        ..Default::default()
    })?;

    // Projetos onde sou membro
    let projects = gl.projects.list(None).await?;
    for project in &projects {
        println!("{}: {}", project.id, project.name);
    }

    // Buscar usuário atual
    let me = gl.users.get_current().await?;
    println!("Logado como {} ({})", me.name, me.username);

    // Criar issue
    let issue = gl.issues
        .create(42, &CreateIssuePayload {
            title: "Bug crítico".into(),
            description: Some("Descrição do bug".into()),
            ..Default::default()
        })
        .await?;
    println!("Issue criada: {}", issue.web_url.unwrap_or_default());

    Ok(())
}
```

## Licença

MPL-2.0 — Veja o arquivo `LICENSE` para detalhes.
