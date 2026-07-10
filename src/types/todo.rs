use serde::{Deserialize, Serialize};
use crate::types::base::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Todo {
    pub id: GitLabId,
    pub project: Option<serde_json::Value>,
    pub author: Option<AuthorInfo>,
    pub action_name: Option<String>,
    pub target_type: Option<String>,
    pub target: Option<serde_json::Value>,
    pub target_url: Option<String>,
    pub body: Option<String>,
    pub state: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TodoFilter {
    pub state: Option<String>,
    pub r#type: Option<String>,
    pub action: Option<String>,
    pub author_id: Option<GitLabId>,
    pub project_id: Option<GitLabId>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}
