use serde::{Deserialize, Serialize};

/// Identificador numérico de recurso no GitLab.
use crate::types::base::GitLabId;

/// Mensagem broadcast no GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BroadcastMessage {
    /// ID único da mensagem.
    pub id: GitLabId,
    /// Conteúdo da mensagem.
    pub message: String,
    /// Data e hora de início da exibição.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starts_at: Option<String>,
    /// Data e hora de término da exibição.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ends_at: Option<String>,
    /// Cor de fundo da mensagem.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// Fonte da mensagem.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,
    /// Níveis de acesso alvo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_access_levels: Option<Vec<u32>>,
    /// Tipo de broadcast ("banner" ou "notification").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broadcast_type: Option<String>,
    /// Indica se a mensagem pode ser dispensada.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dismissable: Option<bool>,
}

/// Payload para criar uma nova mensagem broadcast.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateBroadcastMessagePayload {
    /// Conteúdo da mensagem.
    pub message: String,
    /// Data e hora de início da exibição.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starts_at: Option<String>,
    /// Data e hora de término da exibição.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ends_at: Option<String>,
    /// Cor de fundo da mensagem.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// Fonte da mensagem.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,
    /// Níveis de acesso alvo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_access_levels: Option<Vec<u32>>,
    /// Tipo de broadcast ("banner" ou "notification").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broadcast_type: Option<String>,
    /// Indica se a mensagem pode ser dispensada.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dismissable: Option<bool>,
}

/// Payload para atualizar uma mensagem broadcast existente.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateBroadcastMessagePayload {
    /// Conteúdo da mensagem.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Data e hora de início da exibição.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starts_at: Option<String>,
    /// Data e hora de término da exibição.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ends_at: Option<String>,
    /// Cor de fundo da mensagem.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// Fonte da mensagem.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,
    /// Níveis de acesso alvo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_access_levels: Option<Vec<u32>>,
    /// Tipo de broadcast ("banner" ou "notification").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broadcast_type: Option<String>,
    /// Indica se a mensagem pode ser dispensada.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dismissable: Option<bool>,
}
