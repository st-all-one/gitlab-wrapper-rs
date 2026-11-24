# `glw` — CLI do GitLab via stdin/stdout

`glw` é um cliente de linha de comando para a API REST v4 do GitLab. Segue o
modelo Hubble: um verbo, argumentos posicionais e flags; dados vão para o
stdout, diagnósticos para o stderr. Não há estado persistente além do arquivo
de configuração opcional.

## Comece aqui

```bash
glw prime              # protocolo estático: fluxo, credenciais, saída e códigos
glw prime --long       # protocolo completo com gramática e verbos
glw help               # ajuda completa
glw help <verbo>       # ajuda de um verbo específico
glw config check       # valida arquivo/perfil e credencial sem chamar a API
glw whoami             # confirma que a credencial funciona
```

`prime` é o "help para agentes": texto estático por versão, sem ler
configuração, credenciais ou rede. Use-o no início de uma sessão de agente.

## Sintaxe

```text
glw <verbo> [args] [flags]
```

Flags globais podem aparecer antes ou depois do verbo:

```bash
glw --format json --base-url https://gitlab.example.com whoami
glw whoami --format json --base-url https://gitlab.example.com
```

Sem verbo ou com argumentos faltando, a CLI mostra ajuda e termina com código 2
(ou 0 quando o próprio comando `help` é usado). Mensagens de erro incluem o
comando correto e lembram `glw help <verbo>`.

## Comandos

| Verbo | Uso |
|-------|-----|
| `projects` | `list`, `get <id>`, `get-by-path <path>` |
| `issues` | `list`, `get <project_id> <iid>`, `for-project <id>` |
| `merge-requests` | `list`, `get <project_id> <iid>`, `for-project <id>` |
| `users` | `list`, `get <id>` |
| `groups` | `list`, `get <id>`, `get-by-path <path>` |
| `pipelines` | `list <project_id>`, `get <project_id> <pipeline_id>`, `latest <project_id>` |
| `whoami` |Usuário autenticado (`GET /user`) |
| `raw` | Chamada arbitrária: `glw raw <method> <path> [--params ...]` |
| `batch` | Executa operações NDJSON lidas do stdin |
| `config` | `path`, `check` — utilitários de configuração (saída legível, não afetada por `--format`) |
| `prime` | protocolo estático para agentes; `--long` inclui gramática completa |
| `help` | ajuda geral e `glw help <verbo>` |

Exemplos:

```bash
glw whoami --format json
glw projects list --params 'search=api,per_page=5,page=2'
glw projects get 123
glw issues get 123 45
glw merge-requests list --params 'state=opened'
glw pipelines list 123 --params 'status=success'
glw raw GET /api/v4/user
```

## Flags globais

| Flag | Finalidade |
|------|------------|
| `--token <TOKEN>` | Token explícito (maior precedência) |
| `--token-file <PATH>` | Lê o token de um arquivo; use `@-` para ler do stdin |
| `--base-url <URL>` | URL base da instância GitLab |
| `--config <PATH>` | Arquivo TOML de configuração |
| `--auth-method header\|bearer` | Cabeçalho `Private-Token` ou `Authorization: Bearer` |
| `--sudo <USER>` | Cabeçalho `Sudo` |
| `--params <SPEC>` | Filtros/query: `k=v,k=v`, objeto JSON ou `@-` (stdin) |
| `--format json\|jsonl\|table` | Formato de saída (padrão: `json`) |
| `--timeout <SECS>` | Timeout por requisição |
| `--max-rps <N>` | Limite de requisições por segundo |
| `--quiet` | Suprime diagnósticos |
| `--verbose` | Aumenta o detalhamento dos logs no stderr |
| `--dry-run` | Resolve a requisição e imprime no stdout sem abrir conexão |

Ordem completa de resolução (da maior para a menor precedência):

1. flags de linha de comando (`--token`, `--token-file`, `--base-url`, ...);
2. variáveis de ambiente (`GLW_TOKEN`, `GLW_TOKEN_FILE`, `GLW_BASE_URL`, ...);
3. perfil selecionado por `--profile` ou `GLW_PROFILE` (quando presente);
4. seção base do arquivo TOML;
5. padrões (`https://gitlab.com`, `bearer`, timeout 30s, 10 rps).

O perfil **não sobrescreve** variáveis de ambiente: para usar um perfil sem que
um `GLW_TOKEN` exportado no shell interfira, remova essa variável do ambiente
do processo. `--base-url` segue a mesma regra.

Para token especificamente, a ordem é:
`--token` > `--token-file` (incluindo `@-`) > `GLW_TOKEN` > `GITLAB_TOKEN` >
`GLW_TOKEN_FILE` > `token` do perfil > `token_file` do perfil >
`token` da seção base > `token_file` da seção base.

`--base-url` segue: `--base-url` > `GLW_BASE_URL` > `GITLAB_URL` >
`base_url` do perfil > `base_url` da seção base > `https://gitlab.com`.

Os valores do perfil substituem os da seção base campo a campo. Se quiser que
um perfil use um token diferente, defina `token` ou `token_file` dentro dele.

As flags de credencial são globais: `glw whoami --token ...` e
`glw --token ... whoami` são equivalentes.

## Como inserir as credenciais

Existem cinco formas. Escolha uma; a CLI resolve a credencial **antes** de
qualquer chamada à API, no momento em que o comando é iniciado.

### 1. Arquivo de configuração padrão (recomendado no dia a dia)

Crie `~/.config/glw/config.toml`:

```toml
base_url = "https://gitlab.example.com"
token_file = "/run/secrets/glw-token"
auth_method = "bearer"
```

O arquivo apontado por `token_file` deve conter apenas o token, com ou sem
quebra de linha final. Depois disso, basta:

```bash
glw whoami
```

Se preferir o caminho XDG, `glw config path` mostra onde a CLI procura.

### 2. Arquivo de configuração explícito ou perfil

```bash
glw --config ./glw.toml whoami
glw --profile ci projects list
```

Perfis ficam em `[profiles.<nome>]` no TOML. Um perfil pode ter `base_url`,
`token`, `token_file`, `auth_method`, `sudo`, `timeout_secs` e `max_rps`.

### 3. Variáveis de ambiente (recomendado em CI)

```bash
export GLW_BASE_URL="https://gitlab.example.com"
export GLW_TOKEN="glpat-xxxx"
glw whoami
```

Se o segredo já estiver em arquivo:

```bash
export GLW_TOKEN_FILE="/run/secrets/glw-token"
glw whoami
```

`GITLAB_TOKEN` e `GITLAB_URL` são aceitos como aliases legados, com menor
precedência que `GLW_TOKEN` e `GLW_BASE_URL`.

### 4. Flag `--token-file` (incluindo stdin)

```bash
glw --token-file /run/secrets/glw-token whoami
cat /run/secrets/glw-token | glw --token-file @- whoami
```

Com `@-`, a CLI lê **apenas a primeira linha** do stdin e faz `trim` nela.
Use isso quando o token vier de um gerenciador de segredos, por exemplo:

```bash
vault read -field=token secret/gitlab | glw --token-file @- whoami
```

### 5. Flag `--token` (maior precedência)

```bash
glw --token "$GITLAB_TOKEN" whoami
```

Evite essa forma em shells com histórico ou processos visíveis: o token aparece
na linha de comando. Prefira `--token-file @-`, variáveis de ambiente ou
arquivo de configuração em ambientes compartilhados.

### Quando as credenciais são usadas

1. `Cli::parse()` lê as flags.
2. `Context::new()` chama `config::resolve()`: localiza o TOML, seleciona o
   perfil e lê `token_file`/`@-` se necessário.
3. O token fica em `ResolvedOptions.token`, em memória.
4. Somente handlers que precisam de rede chamam `Context::client()`, que
   converte as opções em `GitLabConfig` e cria o cliente HTTP.
5. O cliente HTTP anexa o cabeçalho em cada requisição:
   - `auth_method = bearer` (padrão): `Authorization: Bearer <token>`;
   - `auth_method = header`: `Private-Token: <token>`;
   - `--sudo usuario` adiciona `Sudo: usuario`.
6. `--dry-run` para antes de criar o cliente e não faz chamada de rede; ainda
   assim o token é resolvido para manter uma única precedência em todos os
   comandos. `config path` e `config check` são utilitários locais e não
   criam cliente HTTP nem chamam a API.

O token nunca é escrito em stdout/stderr, logs, mensagens de erro ou no JSON de
`--dry-run`. `glw config check` imprime apenas `token: set`/`unset`.

### Validação rápida

```bash
glw config check
glw whoami --format json
```

`config check` valida o arquivo e mostra a origem resolvida de forma segura;
`whoami` confirma que o token funciona contra a API.

### Parâmetros

`--params` aceita dois formatos equivalentes:

```bash
glw projects list --params 'per_page=5,page=2'
glw projects list --params '{"per_page":5,"page":2}'
```

`@-` lê a especificação do stdin (heredoc ou pipe):

```bash
glw projects list --params @- <<'EOF'
search=api
per_page=20
EOF

Separadores `,` e quebra de linha são equivalentes em `--params`.
```

### Arquivos e stdin

Um `@-` em valores de arquivo/token lê o conteúdo do stdin. Isso permite pipes
sem arquivos temporários:

```bash
printf 'glpat-xxxx\n' | glw --token-file @- whoami
cat operations.ndjson | glw batch
```

## Batch (NDJSON)

`glw batch` lê uma operação por linha do stdin e executa em ordem, de forma
sequencial e determinística:

```bash
glw batch <<'EOF'
{"op":"whoami","args":{},"params":{}}
{"op":"projects.list","args":{},"params":{"per_page":5,"page":2}}
EOF
```

Cada linha de entrada tem `op`, `args` e `params`:

- `op`: nome canônico com ponto (`projects.list`, `issues.get`, `raw`, ...).
  Espaços no lugar do ponto e `merge-requests.*` são aceitos como aliases.
- `args`: argumentos posicionais como array de strings (`["123"]`), ou objeto
  no caso de `raw`: `{"method":"GET","path":"/api/v4/user"}`.
- `params`: objeto JSON ou string `k=v,k=v` (o mesmo formato de `--params`).

Saída: uma linha NDJSON por operação, na mesma ordem:

```json
{"ok":true,"op":"whoami","result":{"id":7,"username":"cli-tester"}}
{"ok":false,"op":"projects.get","error":"404 resource-not-found"}
```

`ok=false` em qualquer operação resulta em código de saída 3.

### Operações disponíveis no batch

`whoami`, `projects.list`, `projects.get`, `projects.get_by_path`,
`issues.list`, `issues.get`, `issues.list_for_project`, `merge_requests.list`,
`merge_requests.get`, `merge_requests.list_for_project`, `users.list`,
`users.get`, `groups.list`, `groups.get`, `groups.get_by_path`,
`pipelines.list`, `pipelines.get`, `pipelines.latest`, `raw`.

## Formatos de saída

- `json`: JSON do recurso (array para listas). Útil para `jq` e scripts.
- `jsonl`: um objeto JSON por linha, um por item de lista.
- `table`: colunas determinísticas, legíveis em terminal/CI.

`config path` e `config check` imprimem texto legível e ignoram `--format`.
Com `--dry-run`, a saída é um objeto JSON descrevendo a requisição resolvida:

```json
{
  "dry_run": true,
  "op": "projects.list",
  "method": "GET",
  "path": "projects",
  "query": {"per_page": "5", "page": "2"},
  "base_url": "https://gitlab.example.com"
}
```

Tokens nunca aparecem na saída, inclusive no `dry-run`.

## Configuração

Local padrão: `~/.config/glw/config.toml` (sobrescrito por `--config`).
TOML com valores padrão e perfis opcionais:

```toml
base_url = "https://gitlab.example.com"
token_file = "/run/secrets/glw-token"
auth_method = "bearer"
sudo = "deploy-bot"
timeout_secs = 30
max_rps = 10

[profiles.ci]
base_url = "https://gitlab.ci.example.com"
token_file = "/run/secrets/glw-ci-token"
```

Seleção de perfil:

```bash
glw --profile ci projects list --format json
```

`glw config path` mostra o caminho resolvido; `glw config check` valida o
arquivo, as credenciais e a URL sem fazer chamadas à API. Caminhos relativos em
`token_file` (ou `GLW_TOKEN_FILE`) são resolvidos a partir do diretório atual.

## Variáveis de ambiente

| Variável | Equivalente |
|----------|-------------|
| `GLW_TOKEN` | `--token` |
| `GLW_TOKEN_FILE` | `--token-file` |
| `GLW_BASE_URL` | `--base-url` |
| `GLW_CONFIG` | `--config` |
| `GLW_AUTH_METHOD` | `--auth-method` |
| `GLW_SUDO` | `--sudo` |
| `GLW_TIMEOUT` | `--timeout` |
| `GLW_MAX_RPS` | `--max-rps` |
| `GLW_PROFILE` | perfil em `[profiles.*]` |
| `GITLAB_TOKEN` | token legado (menor precedência) |
| `GITLAB_URL` | URL legada (menor precedência que `GLW_BASE_URL`) |
| `GLW_LOG` | filtro de log em stderr (ou `RUST_LOG`) |

## Logs

Diagnósticos (`tracing`) vão para o stderr, nunca para o stdout. `--verbose`
aumenta o detalhamento; `--quiet` suprime. Redirecionar o stderr não altera os
dados produzidos no stdout, o que mantém pipes e `jq` determinísticos.

## Códigos de saída

| Código | Significado |
|--------|-------------|
| `0` | Sucesso |
| `2` | Uso/configuração inválida (args, TOML, token ausente) |
| `3` | Falha da API (por exemplo, 404) ou falha em operação do batch |
| `4` | Erro de I/O (leitura de arquivo/stdin) |

Mensagens de erro incluem status HTTP e descrição legível quando disponíveis:

```text
glw: 404 resource-not-found (instance=018f...)
```

## GitOps / CI

Listar projetos e alimentar outro processo sem arquivos temporários:

```bash
glw projects list --format jsonl \
  --params 'membership=true,per_page=100' \
  | jq -r '.path_with_namespace'
```

Validar credenciais no início de um job:

```bash
glw --token-file /run/secrets/glw-token whoami --format json >/dev/null
glw config check
```

Aplicar mudanças em lote com ordem garantida (dry-run primeiro):

```bash
cat operations.ndjson | glw --dry-run batch > /tmp/glw-plan.ndjson
cat operations.ndjson | glw batch > /tmp/glw-result.ndjson
```

Exemplo em pipeline GitLab CI:

```yaml
stages: [verify]

glw-check:
  stage: verify
  image: rust:1.100
  script:
    - export GLW_TOKEN="$CI_JOB_TOKEN"
    - export GLW_BASE_URL="https://gitlab.example.com"
    - glw config check
    - glw projects list --format jsonl --params 'search=infra' | jq -c .
```
