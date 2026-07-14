use crate::types::base::GitLabId;
use serde::{Deserialize, Serialize};

/// Evento de auditoria no GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuditEvent {
    /// ID único do evento de auditoria.
    pub id: GitLabId,
    /// ID do autor do evento.
    pub author_id: GitLabId,
    /// Nome do autor do evento.
    pub author_name: String,
    /// ID da entidade associada.
    pub entity_id: GitLabId,
    /// Tipo da entidade associada.
    pub entity_type: String,
    /// Detalhes adicionais do evento.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
    /// Data de criação do evento.
    pub created_at: String,
    /// Endereço IP de origem.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}
