use crate::types::base::*;
use serde::{Deserialize, Serialize};

/// Resposta da API GitLab representando um emoji de premiação (award emoji).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AwardEmoji {
    /// ID único do emoji.
    pub id: GitLabId,
    /// Nome do emoji (ex.: "thumbsup", "star").
    pub name: String,
    /// Autor do emoji.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<AuthorInfo>,
    /// Data de criação no formato ISO 8601.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Data da última atualização no formato ISO 8601.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Payload para criar um emoji de premiação.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateEmojiPayload {
    /// Nome do emoji (ex.: "thumbsup", "star").
    pub name: String,
}
