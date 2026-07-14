use serde::{Deserialize, Serialize};

use crate::types::base::GitLabId;

/// Feature flag no GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FeatureFlag {
    /// ID único da feature flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<GitLabId>,
    /// Nome da feature flag.
    pub name: String,
    /// Descrição da feature flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Indica se a feature flag está ativa.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// Versão da feature flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Data de criação.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Data da última atualização.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Estratégias de ativação da feature flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategies: Option<Vec<serde_json::Value>>,
}

/// Payload para criar uma nova feature flag.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateFeatureFlagPayload {
    /// Nome da feature flag.
    pub name: String,
    /// Versão da feature flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Indica se a feature flag deve ser ativada.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// Estratégias de ativação.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategies: Option<Vec<serde_json::Value>>,
}

/// Payload para atualizar uma feature flag existente.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateFeatureFlagPayload {
    /// Indica se a feature flag está ativa.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// Estratégias de ativação.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategies: Option<Vec<serde_json::Value>>,
    /// Descrição da feature flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Filtros para listar feature flags.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FeatureFlagFilter {
    /// Número da página.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Quantidade de itens por página.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>,
    /// Escopo de filtro ("enabled", "disabled").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}
