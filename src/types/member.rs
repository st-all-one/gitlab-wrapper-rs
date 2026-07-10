use serde::{Deserialize, Serialize};
use crate::types::base::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Member {
    pub id: GitLabId,
    pub username: String,
    pub name: String,
    pub state: Option<String>,
    pub avatar_url: Option<String>,
    pub web_url: Option<String>,
    pub access_level: Option<u32>,
    pub expires_at: Option<String>,
    pub created_at: Option<String>,
    pub created_by: Option<AuthorInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddMemberPayload {
    pub user_id: GitLabId,
    pub access_level: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateMemberPayload {
    pub access_level: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}
