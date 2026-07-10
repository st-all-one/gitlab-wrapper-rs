use serde::{Deserialize, Serialize};
use crate::types::base::GitLabId;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Label {
    pub id: GitLabId,
    pub name: String,
    pub color: Option<String>,
    pub text_color: Option<String>,
    pub description: Option<String>,
    pub priority: Option<i32>,
    pub subscribed: Option<bool>,
    pub is_project_label: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GroupLabel {
    pub id: GitLabId,
    pub name: String,
    pub color: Option<String>,
    pub text_color: Option<String>,
    pub description: Option<String>,
    pub priority: Option<i32>,
    pub subscribed: Option<bool>,
    pub group_id: Option<GitLabId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateLabelPayload {
    pub name: String,
    pub color: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateLabelPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
}
