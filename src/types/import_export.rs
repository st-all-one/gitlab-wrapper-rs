use crate::types::base::GitLabId;
use serde::{Deserialize, Serialize};

/// Payload para importar um projeto no GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ImportPayload {
    /// Nome do projeto a ser importado.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Caminho do projeto no GitLab.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// ID do namespace onde o projeto será importado.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_id: Option<GitLabId>,
    /// Parâmetros de override do projeto.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_params: Option<serde_json::Value>,
    /// Arquivo de exportação a ser importado.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
}

/// Status de uma importação de projeto no GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ImportStatus {
    /// ID da importação.
    pub id: GitLabId,
    /// Status atual da importação.
    pub status: String,
    /// Mensagem descritiva do status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Data de criação da importação.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Data da última atualização.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}
