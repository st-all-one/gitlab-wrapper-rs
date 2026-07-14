use crate::types::base::GitLabId;
use serde::{Deserialize, Serialize};

/// Resposta da API GitLab representando um badge (emblema) de projeto/grupo.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Badge {
    /// ID do badge.
    pub id: GitLabId,
    /// Nome do badge.
    pub name: Option<String>,
    /// URL de link do badge.
    pub link_url: Option<String>,
    /// URL da imagem do badge.
    pub image_url: Option<String>,
    /// URL de link renderizada do badge.
    pub rendered_link_url: Option<String>,
    /// URL da imagem renderizada do badge.
    pub rendered_image_url: Option<String>,
    /// Tipo do badge ("project" ou "group").
    pub kind: Option<String>,
}

/// Payload para criar um badge na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateBadgePayload {
    /// Nome do badge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// URL de link do badge (obrigatório).
    pub link_url: String,
    /// URL da imagem do badge (obrigatório).
    pub image_url: String,
}

/// Payload para atualizar um badge na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateBadgePayload {
    /// Nome do badge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// URL de link do badge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_url: Option<String>,
    /// URL da imagem do badge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
}
