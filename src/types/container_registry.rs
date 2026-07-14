use crate::types::base::GitLabId;
use serde::{Deserialize, Serialize};

/// Resposta da API GitLab representando um repositório no Container Registry.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ContainerRepository {
    /// ID do repositório.
    pub id: GitLabId,
    /// Nome do repositório.
    pub name: Option<String>,
    /// Caminho completo do repositório.
    pub path: Option<String>,
    /// Localização do repositório no registry.
    pub location: Option<String>,
    /// Data de criação.
    pub created_at: Option<String>,
    /// Número de tags no repositório.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_count: Option<i64>,
    /// Tamanho total em bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<i64>,
}

/// Resposta da API GitLab representando uma tag no Container Registry.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ContainerTag {
    /// Nome da tag.
    pub name: Option<String>,
    /// Caminho da tag.
    pub path: Option<String>,
    /// Localização da tag no registry.
    pub location: Option<String>,
    /// Digest da imagem.
    pub digest: Option<String>,
    /// Data de criação.
    pub created_at: Option<String>,
    /// Revisão da tag.
    pub revision: Option<String>,
    /// Tamanho total da imagem em bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_size: Option<i64>,
    /// Tipo de mídia da imagem.
    pub media_type: Option<String>,
}
