use crate::types::base::*;
use serde::{Deserialize, Serialize};

/// Resposta da API GitLab representando uma iteração de grupo.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GroupIteration {
    /// ID único da iteração.
    pub id: GitLabId,
    /// IID (internal ID) da iteração.
    pub iid: Option<u32>,
    /// ID do grupo ao qual a iteração pertence.
    pub group_id: Option<GitLabId>,
    /// Título da iteração.
    pub title: Option<String>,
    /// Descrição da iteração.
    pub description: Option<String>,
    /// Estado da iteração ("upcoming", "current", "closed").
    pub state: Option<String>,
    /// Data de início no formato ISO 8601.
    pub start_date: Option<String>,
    /// Data de término no formato ISO 8601.
    pub due_date: Option<String>,
    /// Data de criação no formato ISO 8601.
    pub created_at: Option<String>,
    /// Data da última atualização no formato ISO 8601.
    pub updated_at: Option<String>,
}

/// Filtros disponíveis para listar iterações de grupo.
/// Use `..Default::default()` para valores padrão.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GroupIterationFilter {
    /// Filtrar por estado da iteração ("upcoming", "current", "closed").
    pub state: Option<String>,
    /// Termo de busca para filtrar iterações.
    pub search: Option<String>,
    /// Número da página para recuperação.
    pub page: Option<u32>,
    /// Número de itens por página.
    pub per_page: Option<u32>,
}
