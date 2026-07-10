use serde::{Deserialize, Serialize};
use crate::types::base::GitLabId;
use crate::types::commit::Commit;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Job {
    pub id: GitLabId,
    pub pipeline: Option<JobPipeline>,
    pub ref_: Option<String>,
    pub stage: Option<String>,
    pub name: Option<String>,
    pub status: Option<String>,
    pub created_at: Option<String>,
    pub started_at: Option<String>,
    pub finished_at: Option<String>,
    pub duration: Option<f64>,
    pub queued_duration: Option<f64>,
    pub user: Option<serde_json::Value>,
    pub runner: Option<JobRunner>,
    pub artifacts: Option<Vec<JobArtifact>>,
    pub commit: Option<Commit>,
    pub web_url: Option<String>,
    pub tag: Option<bool>,
    pub allow_failure: Option<bool>,
    pub retried: Option<bool>,
    pub playable: Option<bool>,
    pub retryable: Option<bool>,
    pub cancelable: Option<bool>,
    pub erased_at: Option<String>,
    pub artifacts_expire_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct JobPipeline {
    pub id: GitLabId,
    pub ref_: Option<String>,
    pub sha: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct JobRunner {
    pub id: Option<GitLabId>,
    pub description: Option<String>,
    pub active: Option<bool>,
    pub is_shared: Option<bool>,
    pub runner_type: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct JobArtifact {
    pub file_type: Option<String>,
    pub size: Option<u64>,
    pub filename: Option<String>,
    pub file_format: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct JobFilter {
    pub scope: Option<Vec<String>>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}
