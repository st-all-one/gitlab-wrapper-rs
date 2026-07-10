use serde::{Deserialize, Serialize};
use crate::types::base::*;
use crate::types::note::Note;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Event {
    pub id: GitLabId,
    pub title: Option<String>,
    pub project_id: Option<GitLabId>,
    pub group_id: Option<GitLabId>,
    pub action_name: Option<String>,
    pub target_id: Option<GitLabId>,
    pub target_iid: Option<u32>,
    pub target_type: Option<String>,
    pub author_id: Option<GitLabId>,
    pub author: Option<AuthorInfo>,
    pub target_title: Option<String>,
    pub created_at: Option<String>,
    pub note: Option<Note>,
    pub push_data: Option<EventPushData>,
    pub author_username: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventPushData {
    pub commit_count: Option<u32>,
    pub action: Option<String>,
    pub ref_: Option<String>,
    pub ref_type: Option<String>,
    pub commit_title: Option<String>,
    pub ref_count: Option<u32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventFilter {
    pub action: Option<String>,
    pub target_type: Option<String>,
    pub after: Option<String>,
    pub before: Option<String>,
    pub scope: Option<String>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}
