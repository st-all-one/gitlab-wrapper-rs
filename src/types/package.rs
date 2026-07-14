use crate::types::base::GitLabId;
use serde::{Deserialize, Serialize};

/// Resposta da API GitLab representando um pacote.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Package {
    /// ID do pacote.
    pub id: GitLabId,
    /// Nome do pacote.
    pub name: Option<String>,
    /// Versão do pacote.
    pub version: Option<String>,
    /// Tipo do pacote (npm, maven, pypi, etc.).
    pub package_type: Option<String>,
    /// Status do pacote (default, hidden, etc.).
    pub status: Option<String>,
    /// Data de criação.
    pub created_at: Option<String>,
    /// Data da última atualização.
    pub updated_at: Option<String>,
    /// ID do projeto ao qual o pacote pertence.
    pub project_id: Option<GitLabId>,
    /// Tags associadas ao pacote.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<PackageTag>>,
    /// Pipeline associada à criação do pacote.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline: Option<PackagePipeline>,
}

/// Tag associada a um pacote no GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PackageTag {
    /// Nome da tag.
    pub name: Option<String>,
    /// Caminho da tag.
    pub path: Option<String>,
}

/// Pipeline associada a um pacote no GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PackagePipeline {
    /// ID da pipeline.
    pub id: GitLabId,
    /// IID da pipeline.
    pub iid: Option<u32>,
    /// ID do projeto.
    pub project_id: Option<GitLabId>,
    /// SHA do commit.
    pub sha: Option<String>,
    /// Referência (branch/tag).
    #[serde(rename = "ref")]
    pub ref_: Option<String>,
    /// Status da pipeline.
    pub status: Option<String>,
    /// URL da pipeline.
    pub web_url: Option<String>,
    /// Data de criação.
    pub created_at: Option<String>,
    /// Data de atualização.
    pub updated_at: Option<String>,
}
