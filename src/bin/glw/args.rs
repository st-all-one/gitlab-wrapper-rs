//! Definição da interface de linha de comando (`clap`).

use clap::{Parser, Subcommand};

const CREDENTIALS: &str = "AUTENTICAÇÃO (antes de qualquer comando que chame a API): configure ~/.config/glw/config.toml com token_file, use GLW_TOKEN/GLW_BASE_URL (GITLAB_TOKEN legado), --token-file @- (stdin) ou --token. Precedência: --token > --token-file > GLW_TOKEN > GITLAB_TOKEN > GLW_TOKEN_FILE > perfil > config. Veja 'glw config check'.";

/// Argumentos globais da CLI (aceitos antes ou depois do subcomando).
#[derive(Debug, clap::Args)]
#[command(after_help = CREDENTIALS)]
pub struct GlobalArgs {
    /// Token explícito (maior precedência). Evite em shells com histórico.
    #[arg(
        long,
        global = true,
        value_name = "TOKEN",
        help = "Token de acesso GitLab. Ex.: --token \"$GLW_TOKEN\". Prefira --token-file ou variáveis de ambiente."
    )]
    pub token: Option<String>,

    /// Arquivo com o token; @- lê a primeira linha do stdin.
    #[arg(
        long,
        global = true,
        value_name = "PATH",
        help = "Caminho do arquivo com o token (uma linha). Use --token-file @- para ler do stdin. Ex.: glw --token-file /run/secrets/glw-token whoami"
    )]
    pub token_file: Option<String>,

    /// URL base da instância GitLab (padrão: <https://gitlab.com>).
    #[arg(
        long,
        global = true,
        value_name = "URL",
        help = "URL base da instância. Ex.: --base-url https://gitlab.example.com (equivale a GLW_BASE_URL)."
    )]
    pub base_url: Option<String>,

    /// Arquivo TOML com configuração e perfis.
    #[arg(
        long,
        global = true,
        value_name = "PATH",
        help = "Arquivo TOML. Use 'glw config path' para ver o padrão. Ex.: --config ./glw.toml"
    )]
    pub config: Option<String>,

    /// Método de autenticação: bearer (padrão) ou header.
    #[arg(
        long,
        global = true,
        value_name = "METHOD",
        value_parser = ["bearer", "header", "private-token"],
        help = "bearer => Authorization: Bearer <token>; header => Private-Token: <token>. Ex.: --auth-method header"
    )]
    pub auth_method: Option<String>,

    /// Usuário para impersonação (cabeçalho Sudo).
    #[arg(
        long,
        global = true,
        value_name = "USER",
        help = "Define o cabeçalho Sudo para agir como outro usuário. Requer permissão no GitLab."
    )]
    pub sudo: Option<String>,

    /// Filtros/query: k=v,k=v, JSON objeto ou @- (stdin).
    #[arg(
        long,
        global = true,
        value_name = "SPEC",
        help = "Parâmetros de consulta. Ex.: --params 'search=api,per_page=5' ou --params @- com 'search=api\\nper_page=5' no stdin."
    )]
    pub params: Option<String>,

    /// Formato de saída: json (padrão), jsonl ou table.
    #[arg(
        long,
        global = true,
        value_enum,
        default_value_t = OutputFormat::Json,
        help = "Formato em stdout: json (padrão), jsonl (um objeto por linha) ou table (legível). Logs ficam em stderr."
    )]
    pub format: OutputFormat,

    /// Timeout por requisição em segundos (padrão: 30).
    #[arg(
        long,
        global = true,
        value_name = "SECS",
        help = "Timeout HTTP em segundos. Ex.: --timeout 60"
    )]
    pub timeout: Option<u64>,

    /// Limite de requisições por segundo (padrão: 10).
    #[arg(
        long,
        global = true,
        value_name = "RPS",
        help = "Máximo de requisições por segundo. Ex.: --max-rps 20"
    )]
    pub max_rps: Option<u32>,

    /// Suprime diagnósticos em stderr.
    #[arg(
        long,
        global = true,
        help = " Não imprime logs/erros em stderr. Use apenas quando o chamador tratar falhas pelo código de saída."
    )]
    pub quiet: bool,

    /// Aumenta a verbosidade dos logs (repetível: -v, -vv, -vvv).
    #[arg(
        long,
        global = true,
        action = clap::ArgAction::Count,
        help = " -v: INFO, -vv: DEBUG, -vvv: TRACE. Logs vão para stderr; use GLW_LOG para filtros avançados."
    )]
    pub verbose: u8,

    /// Mostra a requisição planejada sem abrir conexão.
    #[arg(
        long,
        global = true,
        help = " Não chama a API: imprime method/path/query/base_url em stdout. Útil para conferir parâmetros e URLs antes de executar."
    )]
    pub dry_run: bool,

    /// Perfil do arquivo de configuração (seção [profiles.NOME]).
    #[arg(
        long,
        global = true,
        value_name = "NAME",
        help = "Seleciona [profiles.NOME] do TOML. Ex.: --profile ci projects list"
    )]
    pub profile: Option<String>,
}

/// Formatos de saída suportados.
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum OutputFormat {
    /// JSON completo (array para listas).
    Json,
    /// Um objeto JSON por linha.
    Jsonl,
    /// Tabela legível, determinística.
    Table,
}
/// CLI `glw`.
#[derive(Debug, Parser)]
#[command(
    version,
    about = "CLI utilitária para a API GitLab v4 (stdin/stdout). Use 'glw prime' (protocolo) ou 'glw help' (ajuda).",
    long_about = "glw consulta e executa ações na API GitLab v4. Dados vão para stdout; logs e erros para stderr.\n\nCredenciais: ~/.config/glw/config.toml (token_file), GLW_TOKEN/GLW_BASE_URL, --token-file @- (stdin) ou --token. Comece com 'glw config check' e 'glw whoami'.\n\nProtocolo: 'glw prime' imprime instruções estáticas para agentes; 'glw help' mostra a ajuda completa.\n\nExemplos:\n  glw whoami\n  glw projects list --params 'search=api,per_page=5'\n  glw batch < operations.ndjson",
    after_help = CREDENTIALS,
    propagate_version = true,
    arg_required_else_help = true,
    subcommand_required = true
)]
pub struct Cli {
    #[command(flatten)]
    pub global: GlobalArgs,

    /// Subcomando a executar.
    #[command(subcommand)]
    pub command: Command,
}

/// Subcomandos disponíveis.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Projetos: list, get, get-by-path.
    #[command(
        long_about = "Operações sobre projetos.\n\nExemplos:\n  glw projects list --params 'membership=true,per_page=20'\n  glw projects get 123\n  glw projects get-by-path 'grupo/projeto'"
    )]
    Projects {
        #[command(subcommand)]
        command: ResourceCommand,
    },
    /// Issues: list, get, for-project.
    #[command(
        long_about = "Operações sobre issues.\n\nExemplos:\n  glw issues list --params 'state=opened'\n  glw issues get 123 45        # projeto 123, issue #45\n  glw issues for-project 123   # alias de list com projeto fixo"
    )]
    Issues {
        #[command(subcommand)]
        command: ResourceCommand,
    },
    /// Merge requests: list, get, for-project.
    #[command(
        name = "merge-requests",
        long_about = "Operações sobre merge requests.\n\nExemplos:\n  glw merge-requests list --params 'state=opened'\n  glw merge-requests get 123 7   # projeto 123, MR !7\n  glw merge-requests for-project 123"
    )]
    MergeRequests {
        #[command(subcommand)]
        command: ResourceCommand,
    },
    /// Usuários: list, get.
    #[command(
        long_about = "Operações sobre usuários.\n\nExemplos:\n  glw users list --params 'username=alice'\n  glw users get 42"
    )]
    Users {
        #[command(subcommand)]
        command: ResourceCommand,
    },
    /// Pipelines: list, get, latest.
    #[command(
        long_about = "Operações sobre pipelines de um projeto.\n\nExemplos:\n  glw pipelines list 123 --params 'status=success'\n  glw pipelines get 123 456\n  glw pipelines latest 123"
    )]
    Pipelines {
        #[command(subcommand)]
        command: ResourceCommand,
    },
    /// Grupos: list, get, get-by-path.
    #[command(
        long_about = "Operações sobre grupos.\n\nExemplos:\n  glw groups list --params 'search=infra'\n  glw groups get 10\n  glw groups get-by-path 'empresa/time'"
    )]
    Groups {
        #[command(subcommand)]
        command: ResourceCommand,
    },
    /// Mostra o usuário autenticado (GET /user).
    #[command(
        name = "whoami",
        long_about = "Retorna o usuário associado às credenciais resolvidas (GET /user).\n\nExemplos:\n  glw whoami\n  glw whoami --format jsonl\n  glw whoami --dry-run",
        after_help = CREDENTIALS
    )]
    WhoAmI,
    /// Requisição HTTP arbitrária à API (verbos GET/POST/PUT/DELETE).
    #[command(
        long_about = "Faz uma chamada HTTP arbitrária à API v4.\n\nExemplos:\n  glw raw --path user\n  glw raw GET /api/v4/projects --query 'per_page=5'\n  glw raw POST /api/v4/projects --body '{\"name\":\"demo\"}'\n\nUse --dry-run primeiro para conferir method/path/query."
    )]
    Raw(RawArgs),
    /// Executa operações NDJSON lidas do stdin.
    #[command(
        long_about = "Lê uma operação JSON por linha do stdin e executa em ordem.\n\nFormato: {\"op\":\"projects.list\",\"args\":{},\"params\":{\"per_page\":5}}\n\nExemplo:\n  glw batch <<'EOF'\n  {\"op\":\"whoami\",\"args\":{},\"params\":{}}\n  {\"op\":\"projects.list\",\"args\":{},\"params\":{\"per_page\":5}}\n  EOF\n\nFalhas parciais não interrompem o lote; cada linha de saída tem ok/op/result ou error."
    )]
    Batch,
    /// Inspeciona configuração e credenciais resolvidas (utilitário local).
    #[command(
        long_about = "Utilitários locais para conferir configuração. Não fazem chamadas à API.\n\nUse 'glw config path' para descobrir o arquivo e 'glw config check' para validar TOML, URL e credencial sem expor o token.",
        after_help = CREDENTIALS
    )]
    Config {
        #[command(subcommand)]
        command: ConfigCommand,
    },
    /// Protocolo estático de uso (help para agentes/IA).
    #[command(
        long_about = "Imprime instruções estáticas de uso do glw: fluxo, quando usar cada verbo, autenticação, stdout/stderr e códigos de saída. Não lê credenciais nem chama a API.",
        after_help = "Use 'glw prime --long' para o protocolo completo e 'glw help' para ajuda em texto."
    )]
    Prime(PrimeArgs),
}

/// Argumentos do protocolo `prime`.
#[derive(Debug, clap::Args)]
pub struct PrimeArgs {
    /// Inclui gramática de --params/batch e referência completa de comandos.
    #[arg(
        long,
        help = "Imprime o protocolo completo (gramática de --params, batch e lista de verbos). Padrão: versão compacta."
    )]
    pub long: bool,
}

/// Subcomandos comuns a recursos (list/get e aliases).
#[derive(Debug, Subcommand)]
pub enum ResourceCommand {
    /// Lista recursos (aceita --params e, quando disponível, --all).
    #[command(
        long_about = "Lista recursos do GitLab.\n\nExemplos:\n  glw projects list --params 'search=api,per_page=5'\n  glw issues list --params 'state=opened' --all",
        after_help = "Use --params 'chave=valor,...' para filtrar. Ex.: --params 'per_page=20,page=2'."
    )]
    List(ListArgs),
    /// Obtém um recurso pelo ID; issues/MRs usam `<project_id>` `<iid>`.
    #[command(
        long_about = "Obtém um recurso pelo ID.\n\nExemplos:\n  glw projects get 123\n  glw users get 42\n  glw issues get 123 45   # <project_id> <iid>\n  glw pipelines get 123 456"
    )]
    Get(GetArgs),
    /// Obtém um projeto/grupo pelo caminho codificado na URL.
    #[command(
        name = "get-by-path",
        long_about = "Obtém um projeto ou grupo pelo caminho (será codificado como paths/... na API).\n\nExemplos:\n  glw projects get-by-path 'grupo/projeto'\n  glw groups get-by-path 'empresa/time'"
    )]
    GetByPath(PathArgs),
    /// Lista issues/MRs de um projeto (alias de list com ID fixo).
    #[command(
        name = "for-project",
        long_about = "Lista issues ou merge requests de um projeto específico.\n\nExemplos:\n  glw issues for-project 123 --params 'state=opened'\n  glw merge-requests for-project 123"
    )]
    ForProject(ForProjectArgs),
    /// Obtém a pipeline mais recente de um projeto.
    #[command(
        long_about = "Obtém a pipeline mais recente de um projeto.\n\nExemplo:\n  glw pipelines latest 123"
    )]
    Latest(GetArgs),
}

/// Argumentos para listagens.
#[derive(Debug, clap::Args)]
pub struct ListArgs {
    /// ID do projeto (somente para pipelines).
    #[arg(
        help = "ID numérico do projeto; obrigatório apenas em 'glw pipelines list <id>'. Para outros recursos, use --params."
    )]
    pub project_id: Option<u64>,
    /// Percorre todas as páginas automaticamente, quando suportado.
    #[arg(
        long,
        help = " Busca todas as páginas automaticamente em recursos suportados. Pode gerar muitas requisições."
    )]
    pub all: bool,
}

/// Argumentos para obter um recurso pelo ID.
#[derive(Debug, clap::Args)]
#[command(after_help = "Dica: use 'glw <recurso> get --help' para ver a ordem correta dos IDs.")]
pub struct GetArgs {
    /// ID do projeto, ou ID do recurso para projects/groups/users.
    #[arg(
        help = "Primeiro ID: projeto para issues/MRs/pipelines; ID do recurso para projects/groups/users. Ex.: 123"
    )]
    pub project_id: u64,
    /// IID da issue/MR ou ID da pipeline (quando aplicável).
    #[arg(
        help = "Segundo ID: IID de issue/MR ou ID de pipeline. Ex.: 'glw issues get 123 45'. Omita para projects/groups/users."
    )]
    pub resource_id: Option<u64>,
}

/// Argumentos para obter um recurso pelo caminho.
#[derive(Debug, clap::Args)]
pub struct PathArgs {
    /// Caminho URL-encoded (ex.: grupo/projeto).
    #[arg(
        help = "Caminho do recurso no GitLab. Ex.: 'grupo/projeto'. A CLI codifica automaticamente para a API."
    )]
    pub path: String,
}

/// Argumentos para listagens por projeto.
#[derive(Debug, clap::Args)]
pub struct ForProjectArgs {
    /// ID numérico do projeto.
    #[arg(help = "ID numérico do projeto. Ex.: 123")]
    pub project_id: u64,
    /// Percorre todas as páginas, quando suportado.
    #[arg(
        long,
        help = " Busca todas as páginas automaticamente em recursos suportados. Pode gerar muitas requisições."
    )]
    pub all: bool,
}

/// Argumentos do comando `raw`.
#[derive(Debug, clap::Args)]
#[command(
    after_help = "Dica: 'glw raw --path user' é equivalente a 'glw whoami'. Use --dry-run para inspecionar a requisição."
)]
pub struct RawArgs {
    /// Método HTTP (GET, POST, PUT, DELETE; PATCH é mapeado para PUT).
    #[arg(
        short = 'X',
        long,
        default_value = "GET",
        value_name = "METHOD",
        value_parser = ["GET", "POST", "PUT", "DELETE", "PATCH", "HEAD", "get", "post", "put", "delete", "patch", "head"],
        help = "Método HTTP. Padrão: GET. Ex.: -X POST ou --method DELETE. PATCH é enviado como PUT."
    )]
    pub method: String,

    /// Caminho da API (com ou sem /api/v4/).
    #[arg(
        long,
        value_name = "PATH",
        help = "Caminho da API. Ex.: --path user ou --path /api/v4/projects. Prefira iniciar com / para clareza."
    )]
    pub path: String,

    /// Corpo JSON; @- lê do stdin.
    #[arg(
        long,
        value_name = "JSON",
        help = "Corpo JSON para POST/PUT. Ex.: --body '{\"name\":\"demo\"}'. Use --body @- para ler do stdin."
    )]
    pub body: Option<String>,

    /// Query string: JSON objeto ou k=v,k=v.
    #[arg(
        long,
        value_name = "QUERY",
        help = "Parâmetros de query. Ex.: --query 'per_page=5,page=2' ou --query '{\"per_page\":5}'. Equivale a --params."
    )]
    pub query: Option<String>,
}

/// Subcomandos de configuração.
#[derive(Debug, Subcommand)]
pub enum ConfigCommand {
    /// Mostra o caminho do arquivo de configuração.
    #[command(
        long_about = "Mostra onde a CLI procura o TOML (respeita GLW_CONFIG).\n\nExemplo:\n  glw config path"
    )]
    Path,
    /// Valida o TOML e mostra as credenciais resolvidas sem expor o token.
    #[command(
        long_about = "Valida arquivo/perfil, URL e credenciais sem chamar a API. Imprime 'token: set' ou 'unset', nunca o segredo.\n\nExemplos:\n  glw config check\n  glw --profile ci config check\n  glw config check --token-file /run/secrets/glw-token",
        after_help = CREDENTIALS
    )]
    Check,
}
