use crate::types::base::*;
use serde::{Deserialize, Serialize};

/// Resposta da API GitLab representando um projeto.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Project {
    /// ID único do projeto.
    pub id: GitLabId,
    /// Nome do projeto.
    pub name: String,
    /// Caminho do projeto (usado na URL).
    pub path: String,
    /// Descrição do projeto.
    pub description: Option<String>,
    /// Nível de visibilidade do projeto ("public", "internal", "private").
    pub visibility: Option<String>,
    /// Namespace ao qual o projeto pertence.
    pub namespace: Option<ProjectNamespace>,
    /// Nome da branch padrão do projeto.
    pub default_branch: Option<String>,
    /// URL SSH para clonar o repositório.
    pub ssh_url_to_repo: Option<String>,
    /// URL HTTP para clonar o repositório.
    pub http_url_to_repo: Option<String>,
    /// URL da página do projeto no GitLab.
    pub web_url: Option<String>,
    /// URL do avatar do projeto.
    pub avatar_url: Option<String>,
    /// Número de estrelas do projeto.
    pub star_count: Option<u32>,
    /// Número de forks do projeto.
    pub forks_count: Option<u32>,
    /// Data da última atividade no formato ISO 8601.
    pub last_activity_at: Option<String>,
    /// Data de criação no formato ISO 8601.
    pub created_at: Option<String>,
    /// Data da última atualização no formato ISO 8601.
    pub updated_at: Option<String>,
    /// Indica se o projeto está arquivado.
    pub archived: Option<bool>,
    /// Indica se o repositório está vazio.
    pub empty_repo: Option<bool>,
    /// Dono do projeto.
    pub owner: Option<AuthorInfo>,
    /// Permissões do projeto (acesso do usuário atual).
    pub permissions: Option<ProjectPermissions>,
    /// Estatísticas do projeto.
    pub statistics: Option<ProjectStatistics>,
    /// Lista de tópicos do projeto.
    pub topics: Option<Vec<String>>,
    /// Lista de tags do projeto.
    pub tag_list: Option<Vec<String>>,
    /// Indica se o sistema de issues está habilitado.
    pub issues_enabled: Option<bool>,
    /// Indica se o sistema de merge requests está habilitado.
    pub merge_requests_enabled: Option<bool>,
    /// Indica se o wiki está habilitado.
    pub wiki_enabled: Option<bool>,
    /// Indica se os jobs de CI estão habilitados.
    pub jobs_enabled: Option<bool>,
    /// Indica se os snippets estão habilitados.
    pub snippets_enabled: Option<bool>,
    /// Indica se o container registry está habilitado.
    pub container_registry_enabled: Option<bool>,
    /// Indica se runners compartilhados estão habilitados.
    pub shared_runners_enabled: Option<bool>,
    /// Indica se os jobs são públicos.
    pub public_jobs: Option<bool>,
    /// Número de issues abertas no projeto.
    pub open_issues_count: Option<u32>,
    /// Profundidade padrão do git clone para CI.
    pub ci_default_git_depth: Option<u32>,
    /// Indica se o forward deployment de CI está habilitado.
    pub ci_forward_deployment_enabled: Option<bool>,
    /// Indica se solicitações de acesso são permitidas.
    pub request_access_enabled: Option<bool>,
    /// Indica se merge só é permitido se o pipeline passar.
    pub only_allow_merge_if_pipeline_succeeds: Option<bool>,
    /// Indica se merge só é permitido se todas as discussões forem resolvidas.
    pub only_allow_merge_if_all_discussions_are_resolved: Option<bool>,
    /// Indica se a branch de origem é removida após o merge.
    pub remove_source_branch_after_merge: Option<bool>,
    /// Indica se o link do merge request é exibido ao imprimir.
    pub printing_merge_request_link_enabled: Option<bool>,
    /// Método de merge utilizado ("merge", "rebase_merge", "ff").
    pub merge_method: Option<String>,
    /// Opção de squash configurada ("never", "always", "default_on", "default_off").
    pub squash_option: Option<String>,
    /// Indica se o Auto DevOps está habilitado.
    pub auto_devops_enabled: Option<bool>,
    /// Estratégia de deploy do Auto DevOps.
    pub auto_devops_deploy_strategy: Option<String>,
    /// Indica se issues referenciadas são fechadas automaticamente.
    pub autoclose_referenced_issues: Option<bool>,
    /// Armazenamento do repositório.
    pub repository_storage: Option<String>,
    /// Grupos com os quais o projeto foi compartilhado.
    pub shared_with_groups: Option<Vec<serde_json::Value>>,
    /// Links relacionados ao projeto.
    pub _links: Option<Links>,
}

/// Resposta da API GitLab representando o namespace de um projeto.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ProjectNamespace {
    /// ID do namespace.
    pub id: GitLabId,
    /// Nome do namespace.
    pub name: String,
    /// Caminho do namespace.
    pub path: String,
    /// Tipo do namespace ("group" ou "user").
    pub kind: Option<String>,
    /// Caminho completo do namespace.
    pub full_path: Option<String>,
    /// ID do namespace pai.
    pub parent_id: Option<GitLabId>,
    /// URL do avatar do namespace.
    pub avatar_url: Option<String>,
    /// URL do namespace no GitLab.
    pub web_url: Option<String>,
}

/// Resposta da API GitLab representando as permissões de um projeto.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ProjectPermissions {
    /// Nível de acesso do projeto.
    pub project_access: Option<ProjectAccessLevel>,
    /// Nível de acesso do grupo.
    pub group_access: Option<ProjectAccessLevel>,
}

/// Resposta da API GitLab representando o nível de acesso de um projeto ou grupo.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ProjectAccessLevel {
    /// Valor numérico do nível de acesso.
    pub access_level: Option<u32>,
    /// Nível de notificação.
    pub notification_level: Option<u32>,
}

/// Resposta da API GitLab representando estatísticas de um projeto
/// (contagem de commits, tamanhos de armazenamento, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ProjectStatistics {
    /// Número de commits no repositório.
    pub commit_count: Option<u64>,
    /// Tamanho total do armazenamento em bytes.
    pub storage_size: Option<u64>,
    /// Tamanho do repositório em bytes.
    pub repository_size: Option<u64>,
    /// Tamanho do wiki em bytes.
    pub wiki_size: Option<u64>,
    /// Tamanho dos objetos LFS em bytes.
    pub lfs_objects_size: Option<u64>,
    /// Tamanho dos artefatos de jobs em bytes.
    pub job_artifacts_size: Option<u64>,
    /// Tamanho dos artefatos de pipeline em bytes.
    pub pipeline_artifacts_size: Option<u64>,
    /// Tamanho dos pacotes em bytes.
    pub packages_size: Option<u64>,
    /// Tamanho dos snippets em bytes.
    pub snippets_size: Option<u64>,
    /// Tamanho dos uploads em bytes.
    pub uploads_size: Option<u64>,
}

/// Payload para criar um projeto na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateProjectPayload {
    /// Nome do projeto.
    pub name: String,
    /// Caminho do projeto (opcional; usa o nome se não fornecido).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Descrição do projeto.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Nível de visibilidade do projeto.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    /// Indica se o repositório deve ser iniciado com um README.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initialize_with_readme: Option<bool>,
    /// ID do namespace onde o projeto será criado.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_id: Option<GitLabId>,
    /// Lista de tópicos do projeto.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
}

/// Payload para atualizar um projeto na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateProjectPayload {
    /// Novo nome do projeto.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Nova descrição do projeto.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Novo nível de visibilidade do projeto.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    /// Nova lista de tópicos do projeto.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    /// Nome da nova branch padrão.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_branch: Option<String>,
}

/// Filtros disponíveis para listar projetos. Use `..Default::default()` para valores padrão.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ProjectFilter {
    /// Termo de busca para filtrar projetos.
    pub search: Option<String>,
    /// Filtrar por visibilidade ("public", "internal", "private").
    pub visibility: Option<String>,
    /// Filtrar apenas projetos dos quais o usuário é membro.
    pub membership: Option<bool>,
    /// Filtrar apenas projetos dos quais o usuário é dono.
    pub owned: Option<bool>,
    /// Filtrar apenas projetos marcados com estrela pelo usuário.
    pub starred: Option<bool>,
    /// Filtrar por estado de arquivamento.
    pub archived: Option<bool>,
    /// Filtrar por tópico.
    pub topic: Option<String>,
    /// Campo pelo qual ordenar os resultados.
    pub order_by: Option<String>,
    /// Direção da ordenação ("asc" ou "desc").
    pub sort: Option<String>,
    /// Incluir estatísticas do projeto na resposta.
    pub statistics: Option<bool>,
    /// Número da página para recuperação.
    pub page: Option<u32>,
    /// Número de itens por página.
    pub per_page: Option<u32>,
}
