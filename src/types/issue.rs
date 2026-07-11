use serde::{Deserialize, Serialize};
use crate::types::base::*;

/// Resposta da API GitLab representando uma issue.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Issue {
    /// ID único da issue (global).
    pub id: GitLabId,
    /// ID interno da issue (por projeto).
    pub iid: u32,
    /// ID do projeto ao qual a issue pertence.
    pub project_id: GitLabId,
    /// Título da issue.
    pub title: String,
    /// Descrição da issue.
    pub description: Option<String>,
    /// Estado da issue ("opened", "closed").
    pub state: Option<String>,
    /// Data de criação no formato ISO 8601.
    pub created_at: Option<String>,
    /// Data da última atualização no formato ISO 8601.
    pub updated_at: Option<String>,
    /// Data de fechamento no formato ISO 8601.
    pub closed_at: Option<String>,
    /// Lista de labels associadas à issue.
    pub labels: Option<Vec<String>>,
    /// Milestone associado à issue.
    pub milestone: Option<serde_json::Value>,
    /// Usuários designados para a issue.
    pub assignees: Option<Vec<AuthorInfo>>,
    /// Autor da issue.
    pub author: Option<AuthorInfo>,
    /// URL da issue no GitLab.
    pub web_url: Option<String>,
    /// Indica se a issue é confidencial.
    pub confidential: Option<bool>,
    /// Indica se a discussão está bloqueada.
    pub discussion_locked: Option<bool>,
    /// Tipo da issue ("issue", "incident", "test_case").
    pub issue_type: Option<String>,
    /// Severidade da issue.
    pub severity: Option<String>,
    /// Estatísticas de tempo da issue.
    pub time_stats: Option<TimeStats>,
    /// Status de conclusão de tarefas na descrição.
    pub task_completion_status: Option<TaskCompletionStatus>,
    /// Referências da issue (short, relative, full).
    pub references: Option<IssueReferences>,
    /// ID da issue para a qual esta foi movida.
    pub moved_to_id: Option<GitLabId>,
    /// ID da issue duplicada para a qual esta aponta.
    pub duplicated_to_id: Option<GitLabId>,
    /// ID do usuário que fez a última atualização.
    pub updated_by_id: Option<GitLabId>,
    /// Data da última edição no formato ISO 8601.
    pub last_edited_at: Option<String>,
    /// Usuário que fez a última edição.
    pub last_edited_by: Option<AuthorInfo>,
    /// Número de comentários na issue.
    pub user_notes_count: Option<u32>,
    /// Número de votos positivos.
    pub upvotes: Option<u32>,
    /// Número de votos negativos.
    pub downvotes: Option<u32>,
    /// Número de merge requests que referenciam esta issue.
    pub merge_requests_count: Option<u32>,
    /// Data de vencimento no formato ISO 8601.
    pub due_date: Option<String>,
    /// Peso da issue.
    pub weight: Option<i32>,
    /// Links relacionados à issue.
    pub _links: Option<Links>,
}

/// Resposta da API GitLab representando referências de uma issue
/// (atalhos no formato `#id`, `grupo/projeto#id`, URL completa).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IssueReferences {
    /// Referência curta (ex.: "#123").
    pub short: Option<String>,
    /// Referência relativa (ex.: "grupo/projeto#123").
    pub relative: Option<String>,
    /// Referência completa (ex.: "https://gitlab.com/grupo/projeto/-/issues/123").
    pub full: Option<String>,
}

/// Payload para criar uma issue na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateIssuePayload {
    /// Título da issue.
    pub title: String,
    /// Descrição da issue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Indica se a issue é confidencial.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidential: Option<bool>,
    /// Labels separadas por vírgula.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<String>,
    /// IDs dos usuários designados.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee_ids: Option<Vec<GitLabId>>,
    /// ID do milestone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milestone_id: Option<GitLabId>,
    /// Peso da issue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    /// Data de vencimento no formato ISO 8601.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
}

/// Payload para atualizar uma issue na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateIssuePayload {
    /// Novo título da issue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Nova descrição da issue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Indica se a issue é confidencial.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidential: Option<bool>,
    /// Evento de transição de estado ("close", "reopen").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_event: Option<String>,
    /// Labels separadas por vírgula.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<String>,
    /// IDs dos usuários designados.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee_ids: Option<Vec<GitLabId>>,
    /// ID do milestone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milestone_id: Option<GitLabId>,
    /// Peso da issue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    /// Data de vencimento no formato ISO 8601.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
}

/// Filtros disponíveis para listar issues. Use `..Default::default()` para valores padrão.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IssueFilter {
    /// Filtrar por estado ("opened", "closed").
    pub state: Option<String>,
    /// Filtrar por labels (separadas por vírgula).
    pub labels: Option<String>,
    /// ID do usuário designado.
    pub assignee_id: Option<GitLabId>,
    /// Título do milestone para filtrar.
    pub milestone: Option<String>,
    /// Termo de busca no título e descrição.
    pub search: Option<String>,
    /// Escopo da busca ("created-by-me", "assigned-to-me", "all").
    pub scope: Option<String>,
    /// Filtrar issues confidenciais.
    pub confidential: Option<bool>,
    /// Campo pelo qual ordenar os resultados.
    pub order_by: Option<String>,
    /// Direção da ordenação ("asc" ou "desc").
    pub sort: Option<String>,
    /// Número da página para recuperação.
    pub page: Option<u32>,
    /// Número de itens por página.
    pub per_page: Option<u32>,
}
