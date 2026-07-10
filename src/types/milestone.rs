use serde::{Deserialize, Serialize};
use crate::types::base::GitLabId;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Milestone {
    pub id: GitLabId,
    pub iid: Option<u32>,
    pub project_id: Option<GitLabId>,
    pub group_id: Option<GitLabId>,
    pub title: String,
    pub description: Option<String>,
    pub state: Option<String>,
    pub due_date: Option<String>,
    pub start_date: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub web_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateMilestonePayload {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateMilestonePayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_event: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MilestoneFilter {
    pub state: Option<String>,
    pub search: Option<String>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}
