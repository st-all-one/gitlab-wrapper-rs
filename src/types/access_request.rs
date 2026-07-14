use crate::types::base::GitLabId;
use serde::{Deserialize, Serialize};

/// Resposta da API GitLab representando uma solicitação de acesso a um projeto.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AccessRequest {
    /// ID da solicitação de acesso.
    pub id: GitLabId,
    /// Nome de usuário do solicitante.
    pub username: String,
    /// Nome completo do solicitante.
    pub name: String,
    /// Estado da solicitação (ex.: "pending", "approved").
    pub state: String,
    /// Data de criação da solicitação (formato ISO 8601).
    pub created_at: Option<String>,
    /// Data em que o acesso foi solicitado (formato ISO 8601).
    pub requested_at: Option<String>,
    /// Nível de acesso concedido.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_level: Option<u32>,
}

/// Filtros disponíveis para listar solicitações de acesso.
/// Use `..Default::default()` para valores padrão.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AccessRequestFilter {
    /// Número da página para recuperação.
    pub page: Option<u32>,
    /// Número de itens por página.
    pub per_page: Option<u32>,
}
