use crate::types::base::GitLabId;
use serde::{Deserialize, Serialize};

/// Resposta da API GitLab representando um token de deploy.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeployToken {
    /// ID do token de deploy.
    pub id: GitLabId,
    /// Nome do token de deploy.
    pub name: Option<String>,
    /// Nome de usuário associado ao token de deploy.
    pub username: Option<String>,
    /// Data de expiração do token (formato ISO 8601).
    pub expires_at: Option<String>,
    /// Escopos de permissão do token.
    pub scopes: Option<Vec<String>>,
    /// Valor do token (omitido na serialização por segurança).
    #[serde(skip_serializing)]
    pub token: Option<String>,
}

/// Payload para criar um token de deploy na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateDeployTokenPayload {
    /// Nome do token de deploy (obrigatório).
    pub name: String,
    /// Escopos de permissão do token (obrigatório).
    pub scopes: Vec<String>,
    /// Data de expiração do token (formato ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Nome de usuário associado ao token de deploy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}
