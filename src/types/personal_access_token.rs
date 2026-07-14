use crate::types::base::GitLabId;
use serde::{Deserialize, Serialize};

/// Resposta da API GitLab representando um token de acesso pessoal.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PersonalAccessToken {
    /// ID do token.
    pub id: GitLabId,
    /// Nome do token.
    pub name: Option<String>,
    /// Escopos de permissão do token.
    pub scopes: Option<Vec<String>>,
    /// ID do usuário proprietário do token.
    pub user_id: Option<GitLabId>,
    /// Data de expiração do token (formato ISO 8601).
    pub expires_at: Option<String>,
    /// Indica se o token está ativo.
    pub active: Option<bool>,
    /// Data de criação do token (formato ISO 8601).
    pub created_at: Option<String>,
    /// Indica se o token foi revogado.
    pub revoked: Option<bool>,
    /// Valor do token (omitido na serialização por segurança).
    #[serde(skip_serializing)]
    pub token: Option<String>,
}

/// Filtros disponíveis para listar tokens de acesso pessoal.
/// Use `..Default::default()` para valores padrão.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PersonalAccessTokenFilter {
    /// Número da página para recuperação.
    pub page: Option<u32>,
    /// Número de itens por página.
    pub per_page: Option<u32>,
    /// ID do usuário para filtrar tokens.
    pub user_id: Option<GitLabId>,
    /// Estado do token ("active", "inactive", "expired").
    pub state: Option<String>,
    /// Indica se o token foi revogado.
    pub revoked: Option<bool>,
}
