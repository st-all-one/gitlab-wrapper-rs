use serde::{Deserialize, Serialize};
use crate::types::base::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Pipeline {
    pub id: GitLabId,
    pub project_id: Option<GitLabId>,
    pub ref_: Option<String>,
    pub sha: Option<String>,
    pub before_sha: Option<String>,
    pub status: Option<String>,
    pub detailed_status: Option<String>,
    pub stages: Option<Vec<String>>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub started_at: Option<String>,
    pub finished_at: Option<String>,
    pub committed_at: Option<String>,
    pub duration: Option<f64>,
    pub queued_duration: Option<f64>,
    pub user: Option<AuthorInfo>,
    pub source: Option<String>,
    pub web_url: Option<String>,
    pub yaml_errors: Option<String>,
    pub tag: Option<bool>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PipelineVariable {
    pub key: Option<String>,
    pub value: Option<String>,
    pub variable_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreatePipelinePayload {
    pub ref_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<Vec<PipelineVariable>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PipelineFilter {
    pub scope: Option<String>,
    pub status: Option<String>,
    pub ref_: Option<String>,
    pub sha: Option<String>,
    pub source: Option<String>,
    pub username: Option<String>,
    pub updated_after: Option<String>,
    pub updated_before: Option<String>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}
