use serde::{Deserialize, Serialize};
use crate::types::base::*;

/// Resposta da API GitLab representando um merge request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MergeRequest {
    /// ID único do merge request (global).
    pub id: GitLabId,
    /// ID interno do merge request (por projeto).
    pub iid: u32,
    /// ID do projeto ao qual o merge request pertence.
    pub project_id: GitLabId,
    /// Título do merge request.
    pub title: String,
    /// Descrição do merge request.
    pub description: Option<String>,
    /// Estado do merge request ("opened", "closed", "merged", "locked").
    pub state: Option<String>,
    /// Data de criação no formato ISO 8601.
    pub created_at: Option<String>,
    /// Data da última atualização no formato ISO 8601.
    pub updated_at: Option<String>,
    /// Data de merge no formato ISO 8601.
    pub merged_at: Option<String>,
    /// Data de fechamento no formato ISO 8601.
    pub closed_at: Option<String>,
    /// Nome da branch de origem.
    pub source_branch: Option<String>,
    /// Nome da branch de destino.
    pub target_branch: Option<String>,
    /// ID do projeto de origem.
    pub source_project_id: Option<GitLabId>,
    /// ID do projeto de destino.
    pub target_project_id: Option<GitLabId>,
    /// Autor do merge request.
    pub author: Option<AuthorInfo>,
    /// Usuários designados.
    pub assignees: Option<Vec<AuthorInfo>>,
    /// Revisores do merge request.
    pub reviewers: Option<Vec<AuthorInfo>>,
    /// URL do merge request no GitLab.
    pub web_url: Option<String>,
    /// Status do merge ("can_be_merged", "cannot_be_merged", "unchecked", "checking").
    pub merge_status: Option<String>,
    /// Indica se o merge deve ocorrer automaticamente quando o pipeline passar.
    pub merge_when_pipeline_succeeds: Option<bool>,
    /// Indica se o merge request é um rascunho (draft).
    pub draft: Option<bool>,
    /// Indica se o merge request é um work in progress (WIP).
    pub work_in_progress: Option<bool>,
    /// Labels associadas ao merge request.
    pub labels: Option<Vec<String>>,
    /// Milestone associado.
    pub milestone: Option<serde_json::Value>,
    /// Estatísticas de tempo do merge request.
    pub time_stats: Option<TimeStats>,
    /// Status de conclusão de tarefas na descrição.
    pub task_completion_status: Option<TaskCompletionStatus>,
    /// Número de votos positivos.
    pub upvotes: Option<u32>,
    /// Número de votos negativos.
    pub downvotes: Option<u32>,
    /// Número de comentários.
    pub user_notes_count: Option<u32>,
    /// Número de alterações no merge request (como string).
    pub changes_count: Option<String>,
    /// Indica se o squash está habilitado.
    pub squash: Option<bool>,
    /// Pipeline associado ao merge request.
    pub pipeline: Option<MergeRequestPipeline>,
    /// Referências de diff do merge request.
    pub diff_refs: Option<MergeRequestDiffRefs>,
    /// Indica se a branch de origem deve ser removida após o merge.
    pub force_remove_source_branch: Option<bool>,
    /// SHA do commit de merge.
    pub merge_commit_sha: Option<String>,
    /// SHA do commit de squash.
    pub squash_commit_sha: Option<String>,
    /// Indica se a branch de origem deve ser removida.
    pub should_remove_source_branch: Option<bool>,
    /// Usuário que realizou o merge.
    pub merge_user: Option<AuthorInfo>,
    /// Links relacionados ao merge request.
    pub _links: Option<Links>,
}

/// Resposta da API GitLab representando um pipeline associado a um merge request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MergeRequestPipeline {
    /// ID do pipeline.
    pub id: GitLabId,
    /// Nome da branch ou tag do pipeline.
    #[serde(rename = "ref")]
    pub ref_: Option<String>,
    /// SHA do commit do pipeline.
    pub sha: Option<String>,
    /// Status do pipeline ("pending", "running", "passed", "failed", etc.).
    pub status: Option<String>,
    /// URL do pipeline no GitLab.
    pub web_url: Option<String>,
}

/// Resposta da API GitLab representando as referências de diff de um merge request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MergeRequestDiffRefs {
    /// SHA base do diff.
    pub base_sha: Option<String>,
    /// SHA head (mais recente) do diff.
    pub head_sha: Option<String>,
    /// SHA inicial do diff.
    pub start_sha: Option<String>,
}

/// Payload para criar um merge request na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateMergeRequestPayload {
    /// Nome da branch de origem.
    pub source_branch: String,
    /// Nome da branch de destino.
    pub target_branch: String,
    /// Título do merge request.
    pub title: String,
    /// Descrição do merge request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// IDs dos usuários designados.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee_ids: Option<Vec<GitLabId>>,
    /// IDs dos revisores.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviewer_ids: Option<Vec<GitLabId>>,
    /// ID do milestone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milestone_id: Option<GitLabId>,
    /// Labels separadas por vírgula.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<String>,
    /// Indica se a branch de origem deve ser removida após o merge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_source_branch: Option<bool>,
    /// Indica se o squash deve ser aplicado.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub squash: Option<bool>,
    /// Indica se o merge request é um rascunho (draft).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
}

/// Payload para atualizar um merge request na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateMergeRequestPayload {
    /// Novo título do merge request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Nova descrição do merge request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Evento de transição de estado ("close", "reopen", "merge").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_event: Option<String>,
    /// Labels separadas por vírgula.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<String>,
    /// IDs dos usuários designados.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee_ids: Option<Vec<GitLabId>>,
    /// IDs dos revisores.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviewer_ids: Option<Vec<GitLabId>>,
    /// ID do milestone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milestone_id: Option<GitLabId>,
    /// Indica se a branch de origem deve ser removida após o merge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_source_branch: Option<bool>,
    /// Indica se o squash deve ser aplicado.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub squash: Option<bool>,
    /// Indica se a discussão está bloqueada.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discussion_locked: Option<bool>,
}

/// Filtros disponíveis para listar merge requests. Use `..Default::default()` para valores padrão.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MergeRequestFilter {
    /// Filtrar por estado ("opened", "closed", "merged", "all").
    pub state: Option<String>,
    /// Filtrar por labels (separadas por vírgula).
    pub labels: Option<String>,
    /// Título do milestone para filtrar.
    pub milestone: Option<String>,
    /// ID do usuário designado.
    pub assignee_id: Option<GitLabId>,
    /// ID do autor.
    pub author_id: Option<GitLabId>,
    /// ID do revisor.
    pub reviewer_id: Option<GitLabId>,
    /// Nome da branch de origem.
    pub source_branch: Option<String>,
    /// Nome da branch de destino.
    pub target_branch: Option<String>,
    /// Termo de busca no título e descrição.
    pub search: Option<String>,
    /// Filtrar por rascunho (draft).
    pub draft: Option<bool>,
    /// Escopo da busca ("created-by-me", "assigned-to-me", "all").
    pub scope: Option<String>,
    /// Campo pelo qual ordenar os resultados.
    pub order_by: Option<String>,
    /// Direção da ordenação ("asc" ou "desc").
    pub sort: Option<String>,
    /// Número da página para recuperação.
    pub page: Option<u32>,
    /// Número de itens por página.
    pub per_page: Option<u32>,
}

/// Payload para realizar o merge de um merge request na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MergePayload {
    /// Mensagem personalizada para o commit de merge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_commit_message: Option<String>,
    /// Mensagem personalizada para o commit de squash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub squash_commit_message: Option<String>,
    /// Indica se a branch de origem deve ser removida após o merge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub should_remove_source_branch: Option<bool>,
}
