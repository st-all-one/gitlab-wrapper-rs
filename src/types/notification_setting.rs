use crate::types::base::GitLabId;
use serde::{Deserialize, Serialize};

/// Resposta da API GitLab representando configurações de notificação.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NotificationSetting {
    /// ID da entidade (global, projeto ou grupo).
    pub id: GitLabId,
    /// Nível de notificação (disabled, participating, watch, global, mention, custom).
    pub level: Option<String>,
    /// Email de notificação.
    pub notification_email: Option<String>,
    /// Eventos de notificação (mapeamento nome -> booleano).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<serde_json::Value>,
}

/// Payload para atualizar configurações de notificação na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateNotificationPayload {
    /// Nível de notificação.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    /// Email de notificação.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_email: Option<String>,
    /// Evento: nova issue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_issue: Option<bool>,
    /// Evento: nova nota.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_note: Option<bool>,
    /// Evento: novo merge request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_merge_request: Option<bool>,
    /// Evento: pipeline com falha.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_pipeline: Option<bool>,
    /// Evento: pipeline bem-sucedida.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_pipeline: Option<bool>,
    /// Evento: merge request concluído.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_request_merge: Option<bool>,
    /// Evento: branch removida.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_branch: Option<bool>,
    /// Evento: push.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_to_merge_request: Option<bool>,
    /// Evento: issue encerrada.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_issue: Option<bool>,
    /// Evento: issue reaberta.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reopen_issue: Option<bool>,
}
