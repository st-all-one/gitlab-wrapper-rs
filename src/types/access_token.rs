use crate::types::base::GitLabId;
use serde::{Deserialize, Serialize};

/// Resposta da API GitLab representando um token de acesso (projeto/grupo).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AccessToken {
    /// ID do token.
    pub id: GitLabId,
    /// ID do usuário associado ao token.
    pub user_id: Option<GitLabId>,
    /// Nome do token.
    pub name: Option<String>,
    /// Escopos de permissão do token.
    pub scopes: Option<Vec<String>>,
    /// Data de expiração do token (formato ISO 8601).
    pub expires_at: Option<String>,
    /// Indica se o token está ativo.
    pub active: Option<bool>,
    /// Data de criação do token (formato ISO 8601).
    pub created_at: Option<String>,
    /// Indica se o token foi revogado.
    pub revoked: Option<bool>,
    /// Nível de acesso do token.
    pub access_level: Option<u32>,
    /// Valor do token (omitido na serialização por segurança).
    #[serde(skip_serializing)]
    pub token: Option<String>,
}

/// Payload para criar um token de acesso na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateAccessTokenPayload {
    /// Nome do token (obrigatório).
    pub name: String,
    /// Escopos de permissão do token (obrigatório).
    pub scopes: Vec<String>,
    /// Data de expiração do token (formato ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Nível de acesso do token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_level: Option<u32>,
}

/// Filtros disponíveis para listar tokens de acesso.
/// Use `..Default::default()` para valores padrão.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AccessTokenFilter {
    /// Número da página para recuperação.
    pub page: Option<u32>,
    /// Número de itens por página.
    pub per_page: Option<u32>,
}
