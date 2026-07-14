use crate::types::base::*;
use serde::{Deserialize, Serialize};

/// Resposta da API GitLab representando uma tarefa (todo).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Todo {
    /// ID da tarefa.
    pub id: GitLabId,
    /// Projeto relacionado à tarefa.
    pub project: Option<serde_json::Value>,
    /// Autor da tarefa.
    pub author: Option<AuthorInfo>,
    /// Nome da ação que gerou a tarefa.
    pub action_name: Option<String>,
    /// Tipo do alvo da tarefa.
    pub target_type: Option<String>,
    /// Alvo da tarefa (issue, merge request, etc.).
    pub target: Option<serde_json::Value>,
    /// URL do alvo da tarefa.
    pub target_url: Option<String>,
    /// Corpo da tarefa.
    pub body: Option<String>,
    /// Estado da tarefa (done, pending).
    pub state: Option<String>,
    /// Data de criação.
    pub created_at: Option<String>,
}

/// Filtros para listar tarefas. Use `..Default::default()`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TodoFilter {
    /// Filtro por estado (done, pending).
    pub state: Option<String>,
    /// Filtro por tipo de alvo (Issue, MergeRequest, etc.).
    pub r#type: Option<String>,
    /// Filtro por ação (assigned, mentioned, etc.).
    pub action: Option<String>,
    /// Filtro por ID do autor.
    pub author_id: Option<GitLabId>,
    /// Filtro por ID do projeto.
    pub project_id: Option<GitLabId>,
    /// Número da página para paginação.
    pub page: Option<u32>,
    /// Número de itens por página.
    pub per_page: Option<u32>,
}
