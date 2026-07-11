use serde::{Deserialize, Serialize};
use crate::types::base::GitLabId;

/// Resposta da API GitLab representando um marco (milestone).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Milestone {
    /// ID do marco no GitLab.
    pub id: GitLabId,
    /// ID interno do marco (escopo do projeto/grupo).
    pub iid: Option<u32>,
    /// ID do projeto ao qual o marco pertence.
    pub project_id: Option<GitLabId>,
    /// ID do grupo ao qual o marco pertence.
    pub group_id: Option<GitLabId>,
    /// Título do marco.
    pub title: String,
    /// Descrição do marco.
    pub description: Option<String>,
    /// Estado do marco (ex: "active", "closed").
    pub state: Option<String>,
    /// Data de conclusão do marco (formato ISO 8601).
    pub due_date: Option<String>,
    /// Data de início do marco (formato ISO 8601).
    pub start_date: Option<String>,
    /// Data de criação do marco (formato ISO 8601).
    pub created_at: Option<String>,
    /// Data da última atualização do marco (formato ISO 8601).
    pub updated_at: Option<String>,
    /// URL da página do marco no GitLab.
    pub web_url: Option<String>,
}

/// Payload para criar um marco na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateMilestonePayload {
    /// Título do marco (obrigatório).
    pub title: String,
    /// Descrição do marco.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Data de conclusão do marco (formato ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    /// Data de início do marco (formato ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
}

/// Payload para atualizar um marco na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateMilestonePayload {
    /// Novo título do marco.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Nova descrição do marco.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Nova data de conclusão do marco (formato ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    /// Nova data de início do marco (formato ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// Evento de transição de estado (ex: "close", "activate").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_event: Option<String>,
}

/// Filtros para listar marcos. Use `..Default::default()` para valores padrão.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MilestoneFilter {
    /// Filtra pelo estado do marco (ex: "active", "closed").
    pub state: Option<String>,
    /// Termo de busca para filtrar marcos pelo título ou descrição.
    pub search: Option<String>,
    /// Número da página para paginação.
    pub page: Option<u32>,
    /// Número de itens por página.
    pub per_page: Option<u32>,
}
