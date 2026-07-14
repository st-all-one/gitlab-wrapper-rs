use serde::{Deserialize, Serialize};

/// Identificador numérico de recurso no GitLab.
use crate::types::base::GitLabId;

/// System hook (webhook global) no GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SystemHook {
    /// ID único do system hook.
    pub id: GitLabId,
    /// URL de destino do hook.
    pub url: String,
    /// Data de criação.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Dispara em eventos de push.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_events: Option<bool>,
    /// Dispara em eventos de push de tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_push_events: Option<bool>,
    /// Dispara em eventos de merge request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_requests_events: Option<bool>,
    /// Se a verificação SSL está habilitada.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_ssl_verification: Option<bool>,
}

/// Payload para criar um novo system hook.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateSystemHookPayload {
    /// URL de destino do hook.
    pub url: String,
    /// Token de autenticação.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// Dispara em eventos de push.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_events: Option<bool>,
    /// Dispara em eventos de push de tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_push_events: Option<bool>,
    /// Dispara em eventos de merge request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_requests_events: Option<bool>,
    /// Se a verificação SSL está habilitada.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_ssl_verification: Option<bool>,
}
