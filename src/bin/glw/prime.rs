//! Protocolo estático de uso do `glw` ("help para agentes").
//!
//! Texto byte-idêntico por versão, sem ler configuração, credenciais ou rede.

/// Versão do protocolo (incrementar ao alterar o texto).
pub const PROTOCOL_VERSION: &str = "1";

pub const fn version() -> &'static str {
    PROTOCOL_VERSION
}

/// Resumo do protocolo (padrão).
pub const PRIME_COMPACT: &str = "GLW PRIME v1 — static usage protocol.
CYCLE: glw prime → glw config check → glw whoami → <verbo> [--params] → glw batch.
WHEN (use):
- consultas rápidas à API GitLab v4 com credenciais já configuradas;
- lote determinístico de operações via stdin (NDJSON).
WHEN (don't):
- não use para fluxos OAuth complexos nem para mutações sem --dry-run primeiro.
AUTH: ~/.config/glw/config.toml (token_file), GLW_TOKEN/GLW_BASE_URL, --token-file @- (stdin) ou --token. Nunca exponha o token.
ESSENTIAL:
  glw config check                       # valida TOML/URL/credencial, sem API
  glw whoami                             # confirma autenticação
  glw projects list --params 'per_page=5'
  glw issues get <project_id> <iid>
  glw raw --path user                    # GET /api/v4/user
  glw batch < operations.ndjson
OUTPUT: stdout = dados (json/jsonl/table); stderr = logs/erros. Token nunca aparece na saída.
EXIT: 0 ok | 2 uso/config | 3 API | 4 I/O.
RULES: sempre use --dry-run antes de mutar; 'glw help <verbo>' ensina exemplos e escopo; '--params' aceita k=v,k=v, JSON ou @-.
";

/// Protocolo completo: gramática e referência de comandos.
pub const PRIME_FULL: &str = "GLW PRIME v1 — static usage protocol (full).
CYCLE: glw prime → glw config check → glw whoami → <verbo> [--params] → glw batch.
WHEN (use):
- consultas rápidas à API GitLab v4 com credenciais já configuradas;
- lote determinístico de operações via stdin (NDJSON);
- dry-run para inspecionar method/path/query sem abrir conexão.
WHEN (don't):
- não use para fluxos OAuth completos (a lib tem helpers próprios);
- não faça mutações sem antes validar com --dry-run;
- não use --token em shells com histórico compartilhado.
AUTH (precedência): --token > --token-file (@- = stdin) > GLW_TOKEN > GITLAB_TOKEN > GLW_TOKEN_FILE > perfil ([profiles.NOME]) > config base. URL: --base-url > GLW_BASE_URL > GITLAB_URL > perfil > config > https://gitlab.com. Métodos: bearer (Authorization: Bearer) ou header (Private-Token).
GRAMMAR:
  glw <verbo> [args] [--params SPEC] [flags globais]
  SPEC = k=v,k=v | {\"k\":v} | @- (linhas k=v no stdin)
  batch = uma linha JSON por operação: {\"op\":\"projects.list\",\"args\":{},\"params\":{\"per_page\":5}}
VERBS:
  projects     list | get <id> | get-by-path <path>
  issues       list | get <project_id> <iid> | for-project <id>
  merge-requests list | get <project_id> <iid> | for-project <id>
  users        list | get <id>
  groups       list | get <id> | get-by-path <path>
  pipelines    list <project_id> | get <project_id> <pipeline_id> | latest <project_id>
  whoami       usuário autenticado (GET /user)
  raw          -X METHOD --path PATH [--query SPEC] [--body JSON]
  batch        operações NDJSON do stdin
  config       path | check
  prime        este protocolo (--long = completo)
  help         ajuda por verbo
OUTPUT: stdout = dados (json/jsonl/table); stderr = logs/erros. Token nunca aparece na saída.
EXIT: 0 ok | 2 uso/config | 3 API | 4 I/O.
RULES: dry-run antes de mutar; 'glw help <verbo>' para escopo e exemplos; --help mostra flags globais e credenciais.
";

/// Retorna o texto do protocolo conforme o formato solicitado.
pub const fn text(long: bool) -> &'static str {
    if long { PRIME_FULL } else { PRIME_COMPACT }
}
