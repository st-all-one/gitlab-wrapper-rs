use crate::types::base::{AuthorInfo, GitLabId};
use serde::{Deserialize, Serialize};

/// Trigger de pipeline no GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PipelineTrigger {
    /// ID do trigger.
    pub id: GitLabId,
    /// Descrição do trigger.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Token de autenticação do trigger (não enviado em requisições).
    #[serde(skip_serializing)]
    pub token: Option<String>,
    /// Data de criação.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Data da última atualização.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Data do último uso.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_used: Option<String>,
    /// Dono do trigger.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<AuthorInfo>,
    /// Se o trigger pode acessar o projeto.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_access_project: Option<bool>,
}

/// Payload para criar um novo trigger de pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreatePipelineTriggerPayload {
    /// Descrição do trigger.
    pub description: String,
    /// Token opcional para o trigger.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

/// Payload para atualizar um trigger de pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdatePipelineTriggerPayload {
    /// Nova descrição do trigger.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Novo token opcional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

/// Filtros para listar triggers de pipeline.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PipelineTriggerFilter {
    /// Número da página.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Quantidade de itens por página.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>,
}
