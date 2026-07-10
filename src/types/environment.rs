use serde::{Deserialize, Serialize};
use crate::types::base::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Environment {
    pub id: GitLabId,
    pub name: Option<String>,
    pub slug: Option<String>,
    pub external_url: Option<String>,
    pub state: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub last_deployment: Option<EnvironmentDeployment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EnvironmentDeployment {
    pub id: Option<GitLabId>,
    pub iid: Option<u32>,
    pub ref_: Option<String>,
    pub sha: Option<String>,
    pub tag: Option<bool>,
    pub status: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub user: Option<AuthorInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateEnvironmentPayload {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateEnvironmentPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}
