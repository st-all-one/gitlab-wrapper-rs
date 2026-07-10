use serde::{Deserialize, Serialize};
use crate::types::base::GitLabId;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeployKey {
    pub id: GitLabId,
    pub title: Option<String>,
    pub key: Option<String>,
    pub fingerprint: Option<String>,
    pub created_at: Option<String>,
    pub can_push: Option<bool>,
    pub deploy_keys_projects: Option<Vec<DeployKeyProject>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeployKeyProject {
    pub id: Option<GitLabId>,
    pub deploy_key_id: Option<GitLabId>,
    pub project_id: Option<GitLabId>,
    pub can_push: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateDeployKeyPayload {
    pub title: String,
    pub key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_push: Option<bool>,
}
