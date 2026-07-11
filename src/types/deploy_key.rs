use serde::{Deserialize, Serialize};
use crate::types::base::GitLabId;

/// Resposta da API GitLab representando uma chave de deploy.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeployKey {
    /// ID da chave de deploy.
    pub id: GitLabId,
    /// Título da chave de deploy.
    pub title: Option<String>,
    /// Conteúdo da chave pública.
    pub key: Option<String>,
    /// Impressão digital da chave.
    pub fingerprint: Option<String>,
    /// Data de criação.
    pub created_at: Option<String>,
    /// Indica se a chave pode fazer push.
    pub can_push: Option<bool>,
    /// Projetos associados à chave de deploy.
    pub deploy_keys_projects: Option<Vec<DeployKeyProject>>,
}

/// Associação entre uma chave de deploy e um projeto.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeployKeyProject {
    /// ID da associação.
    pub id: Option<GitLabId>,
    /// ID da chave de deploy.
    pub deploy_key_id: Option<GitLabId>,
    /// ID do projeto.
    pub project_id: Option<GitLabId>,
    /// Indica se a chave pode fazer push no projeto.
    pub can_push: Option<bool>,
}

/// Payload para criar uma chave de deploy na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateDeployKeyPayload {
    /// Título da chave (obrigatório).
    pub title: String,
    /// Conteúdo da chave pública (obrigatório).
    pub key: String,
    /// Indica se a chave pode fazer push.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_push: Option<bool>,
}

/// Payload para atualizar uma chave de deploy na API GitLab.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateDeployKeyPayload {
    /// Título da chave.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Indica se a chave pode fazer push.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_push: Option<bool>,
}
