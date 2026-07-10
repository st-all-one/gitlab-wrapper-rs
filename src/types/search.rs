use serde::{Deserialize, Serialize};
use crate::types::base::GitLabId;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchResultItem {
    pub id: Option<GitLabId>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub r#type: Option<String>,
    pub url: Option<String>,
    pub project_id: Option<GitLabId>,
    pub username: Option<String>,
    pub filename: Option<String>,
    pub basename: Option<String>,
    pub data: Option<String>,
    pub path: Option<String>,
    pub ref_: Option<String>,
    pub startline: Option<u32>,
    pub language: Option<String>,
    pub content: Option<String>,
}
