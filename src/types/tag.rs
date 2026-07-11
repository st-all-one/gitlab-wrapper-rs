use serde::{Deserialize, Serialize};

/// Resposta da API GitLab representando uma tag de um repositório.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Tag {
    /// Nome da tag.
    pub name: String,
    /// Mensagem associada à tag (null para tags leves).
    pub message: Option<String>,
    /// SHA do commit alvo da tag.
    pub target: Option<String>,
    /// Commit associado à tag.
    pub commit: Option<TagCommit>,
    /// Release associado à tag.
    pub release: Option<TagRelease>,
    /// Indica se a tag é protegida.
    pub protected: Option<bool>,
}

/// Resposta da API GitLab representando o commit associado a uma tag.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TagCommit {
    /// Hash SHA-1 completo do commit.
    pub id: Option<String>,
    /// Hash SHA-1 abreviado do commit.
    pub short_id: Option<String>,
    /// Título do commit.
    pub title: Option<String>,
    /// Data de criação no formato ISO 8601.
    pub created_at: Option<String>,
    /// Nome do autor do commit.
    pub author_name: Option<String>,
    /// E-mail do autor do commit.
    pub author_email: Option<String>,
}

/// Resposta da API GitLab representando um release associado a uma tag.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TagRelease {
    /// Nome da tag associada ao release.
    pub tag_name: Option<String>,
    /// Descrição do release.
    pub description: Option<String>,
}

/// Payload para criar uma tag na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateTagPayload {
    /// Nome da nova tag.
    pub tag_name: String,
    /// Nome da branch, tag ou SHA do commit de referência.
    #[serde(rename = "ref")]
    pub ref_: String,
    /// Mensagem para tag anotada (opcional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Descrição do release associado à tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_description: Option<String>,
}
