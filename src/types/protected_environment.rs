use crate::types::base::GitLabId;
use serde::{Deserialize, Serialize};

/// Ambiente protegido no GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ProtectedEnvironment {
    /// ID do ambiente protegido.
    /// Campo `pub id`.
    pub id: GitLabId,
    /// Nome do ambiente.
    /// Campo `pub name`.
    pub name: String,
    /// Níveis de acesso para deploy.
    /// Campo `pub deploy_access_levels`.
    pub deploy_access_levels: Vec<ProtectedEnvAccessLevel>,
    /// Aprovações necessárias.
    /// Campo `pub required_approval_count`.
    pub required_approval_count: Option<u32>,
}

/// Nível de acesso para deploy em ambiente protegido.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ProtectedEnvAccessLevel {
    /// Valor numérico do nível de acesso.
    /// Campo `pub access_level`.
    pub access_level: Option<u32>,
    /// Descrição textual.
    /// Campo `pub access_level_description`.
    pub access_level_description: Option<String>,
}

/// Payload para proteger um ambiente.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ProtectEnvironmentPayload {
    /// Nome do ambiente.
    /// Campo `pub name`.
    pub name: String,
    /// Nível de acesso para deploy.
    /// Campo `pub deploy_access_levels`.
    pub deploy_access_levels: Vec<serde_json::Value>,
    /// Aprovações necessárias.
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Campo `pub required_approval_count`.
    pub required_approval_count: Option<u32>,
}

/// Filtro para listar ambientes protegidos.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ProtectedEnvironmentFilter {
    /// Campo `pub page`.
    pub page: Option<u32>,
    /// Campo `pub per_page`.
    pub per_page: Option<u32>,
}
