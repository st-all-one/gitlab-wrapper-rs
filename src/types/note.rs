use serde::{Deserialize, Serialize};
use crate::types::base::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Note {
    pub id: GitLabId,
    pub body: Option<String>,
    pub attachment: Option<String>,
    pub author: Option<AuthorInfo>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub system: Option<bool>,
    pub noteable_id: Option<GitLabId>,
    pub noteable_type: Option<String>,
    pub project_id: Option<GitLabId>,
    pub resolvable: Option<bool>,
    pub resolved: Option<bool>,
    pub resolved_by: Option<AuthorInfo>,
    pub resolved_at: Option<String>,
    pub position: Option<NotePosition>,
    pub _links: Option<Links>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NotePosition {
    pub base_sha: Option<String>,
    pub start_sha: Option<String>,
    pub head_sha: Option<String>,
    pub position_type: Option<String>,
    pub new_path: Option<String>,
    pub old_path: Option<String>,
    pub new_line: Option<u32>,
    pub old_line: Option<u32>,
    pub line_range: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateNotePayload {
    pub body: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidential: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateNotePayload {
    pub body: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidential: Option<bool>,
}
