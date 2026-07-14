use crate::types::base::GitLabId;
use serde::{Deserialize, Serialize};

/// Webhook de grupo no GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GroupHook {
    /// ID do webhook.
    pub id: GitLabId,
    /// URL de destino do webhook.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// ID do grupo associado.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<GitLabId>,
    /// Dispara em pushes de código.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_events: Option<bool>,
    /// Dispara em eventos de issue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issues_events: Option<bool>,
    /// Dispara em eventos de merge request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_requests_events: Option<bool>,
    /// Dispara em eventos de nota/comentário.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_events: Option<bool>,
    /// Dispara em eventos de pipeline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_events: Option<bool>,
    /// Dispara em eventos de página wiki.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wiki_page_events: Option<bool>,
    /// Se a verificação SSL está habilitada.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_ssl_verification: Option<bool>,
    /// Data de criação.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Data da última atualização.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Token de autenticação do webhook (não enviado em listagens).
    #[serde(skip_serializing)]
    pub token: Option<String>,
}

/// Payload para criar um novo webhook de grupo.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateGroupHookPayload {
    /// URL de destino do webhook.
    pub url: String,
    /// Dispara em pushes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_events: Option<bool>,
    /// Dispara em eventos de issue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issues_events: Option<bool>,
    /// Dispara em eventos de merge request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_requests_events: Option<bool>,
    /// Dispara em eventos de nota.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_events: Option<bool>,
    /// Dispara em eventos de pipeline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_events: Option<bool>,
    /// Dispara em eventos de página wiki.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wiki_page_events: Option<bool>,
    /// Se a verificação SSL está habilitada.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_ssl_verification: Option<bool>,
    /// Token de autenticação.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

/// Payload para atualizar um webhook de grupo.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateGroupHookPayload {
    /// Nova URL de destino.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Dispara em pushes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_events: Option<bool>,
    /// Dispara em eventos de issue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issues_events: Option<bool>,
    /// Dispara em eventos de merge request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_requests_events: Option<bool>,
    /// Dispara em eventos de nota.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_events: Option<bool>,
    /// Dispara em eventos de pipeline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_events: Option<bool>,
    /// Dispara em eventos de página wiki.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wiki_page_events: Option<bool>,
    /// Se a verificação SSL está habilitada.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_ssl_verification: Option<bool>,
    /// Novo token de autenticação.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

/// Filtros para listar webhooks de grupo.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GroupHookFilter {
    /// Número da página.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Quantidade de itens por página.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>,
}
