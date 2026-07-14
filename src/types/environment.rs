use crate::types::base::*;
use serde::{Deserialize, Serialize};

/// Resposta da API GitLab representando um ambiente (environment).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Environment {
    /// ID do ambiente.
    pub id: GitLabId,
    /// Nome do ambiente.
    pub name: Option<String>,
    /// Slug do ambiente.
    pub slug: Option<String>,
    /// URL externa do ambiente.
    pub external_url: Option<String>,
    /// Estado do ambiente (available, stopped, etc.).
    pub state: Option<String>,
    /// Data de criação.
    pub created_at: Option<String>,
    /// Data de atualização.
    pub updated_at: Option<String>,
    /// Último deployment do ambiente.
    pub last_deployment: Option<EnvironmentDeployment>,
}

/// Deployment associado a um ambiente.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EnvironmentDeployment {
    /// ID do deployment.
    pub id: Option<GitLabId>,
    /// IID do deployment.
    pub iid: Option<u32>,
    /// Referência (branch/tag) — renomeado de `ref` para `ref_` devido a palavra reservada.
    #[serde(rename = "ref")]
    pub ref_: Option<String>,
    /// SHA do commit do deployment.
    pub sha: Option<String>,
    /// Indica se o deployment é de uma tag.
    pub tag: Option<bool>,
    /// Status do deployment.
    pub status: Option<String>,
    /// Data de criação.
    pub created_at: Option<String>,
    /// Data de atualização.
    pub updated_at: Option<String>,
    /// Usuário que realizou o deployment.
    pub user: Option<AuthorInfo>,
}

/// Payload para criar um ambiente na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateEnvironmentPayload {
    /// Nome do ambiente (obrigatório).
    pub name: String,
    /// URL externa do ambiente.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_url: Option<String>,
    /// Slug personalizado do ambiente.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    /// Tier do ambiente (production, staging, testing, development, other).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}

/// Payload para atualizar um ambiente na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateEnvironmentPayload {
    /// Nome do ambiente.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// URL externa do ambiente.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_url: Option<String>,
    /// Slug personalizado do ambiente.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    /// Tier do ambiente (production, staging, testing, development, other).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}
