use crate::types::base::GitLabId;
use serde::{Deserialize, Serialize};

/// Informações da licença do GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LicenseInfo {
    /// ID da licença.
    pub id: GitLabId,
    /// Plano da licença.
    pub plan: String,
    /// Email associado à licença.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Data de início da licença.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starts_at: Option<String>,
    /// Data de expiração da licença.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Número de usuários ativos.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_users: Option<u64>,
    /// Limite de usuários da licença.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    /// Contagem de usuários.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_count: Option<u64>,
    /// Informações adicionais do licenciado.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub licensee: Option<serde_json::Value>,
}

/// Payload para criar ou atualizar uma licença no GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateLicensePayload {
    /// String da licença (código da licença).
    pub license: String,
}
