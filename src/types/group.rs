use serde::{Deserialize, Serialize};
use crate::types::base::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Group {
    pub id: GitLabId,
    pub name: String,
    pub path: String,
    pub description: Option<String>,
    pub visibility: Option<String>,
    pub avatar_url: Option<String>,
    pub web_url: Option<String>,
    pub full_name: Option<String>,
    pub full_path: Option<String>,
    pub parent_id: Option<GitLabId>,
    pub projects_count: Option<u32>,
    pub subgroup_count: Option<u32>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub shared_with_groups: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateGroupPayload {
    pub name: String,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<GitLabId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateGroupPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GroupFilter {
    pub search: Option<String>,
    pub top_level_only: Option<bool>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}
