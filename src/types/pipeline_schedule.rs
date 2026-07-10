use serde::{Deserialize, Serialize};
use crate::types::base::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PipelineSchedule {
    pub id: GitLabId,
    pub description: Option<String>,
    pub ref_: Option<String>,
    pub cron: Option<String>,
    pub cron_timezone: Option<String>,
    pub next_run_at: Option<String>,
    pub active: Option<bool>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub owner: Option<AuthorInfo>,
    pub last_pipeline: Option<PipelineScheduleLastPipeline>,
    pub variables: Option<Vec<PipelineScheduleVariable>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PipelineScheduleVariable {
    pub id: Option<GitLabId>,
    pub key: Option<String>,
    pub value: Option<String>,
    pub variable_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PipelineScheduleLastPipeline {
    pub id: Option<GitLabId>,
    pub ref_: Option<String>,
    pub sha: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreatePipelineSchedulePayload {
    pub description: String,
    pub ref_: String,
    pub cron: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cron_timezone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdatePipelineSchedulePayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cron: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cron_timezone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}
