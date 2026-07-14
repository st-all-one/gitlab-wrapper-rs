use crate::types::base::GitLabId;
use serde::{Deserialize, Serialize};

/// Epic no GitLab (Ultimate).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Epic {
    /// Campo `pub id`.
    pub id: GitLabId,
    /// Campo `pub iid`.
    pub iid: Option<u32>,
    /// Campo `pub group_id`.
    pub group_id: GitLabId,
    /// Campo `pub title`.
    pub title: String,
    /// Campo `pub description`.
    pub description: Option<String>,
    /// Campo `pub state`.
    pub state: Option<String>,
    /// Campo `pub created_at`.
    pub created_at: Option<String>,
    /// Campo `pub updated_at`.
    pub updated_at: Option<String>,
    /// Campo `pub labels`.
    pub labels: Option<Vec<String>>,
    /// Campo `pub author`.
    pub author: Option<serde_json::Value>,
    /// Campo `pub start_date`.
    pub start_date: Option<String>,
    /// Campo `pub due_date`.
    pub due_date: Option<String>,
    /// Campo `pub web_url`.
    pub web_url: Option<String>,
}

/// Payload para criar/atualizar um epic.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateEpicPayload {
    /// Campo `pub title`.
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Campo `pub description`.
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Campo `pub labels`.
    pub labels: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Campo `pub start_date`.
    pub start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Campo `pub due_date`.
    pub due_date: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// Tipo `UpdateEpicPayload`.
pub struct UpdateEpicPayload {
    /// Campo `pub title`.
    pub title: Option<String>,
    /// Campo `pub description`.
    pub description: Option<String>,
    /// Campo `pub labels`.
    pub labels: Option<Vec<String>>,
    /// Campo `pub start_date`.
    pub start_date: Option<String>,
    /// Campo `pub due_date`.
    pub due_date: Option<String>,
    /// Campo `pub state_event`.
    pub state_event: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// Tipo `EpicFilter`.
pub struct EpicFilter {
    /// Campo `pub page`.
    pub page: Option<u32>,
    /// Campo `pub per_page`.
    pub per_page: Option<u32>,
    /// Campo `pub state`.
    pub state: Option<String>,
    /// Campo `pub labels`.
    pub labels: Option<String>,
}

/// Link entre epics.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EpicLink {
    /// Campo `pub id`.
    pub id: GitLabId,
    /// Campo `pub source_epic`.
    pub source_epic: Option<serde_json::Value>,
    /// Campo `pub target_epic`.
    pub target_epic: Option<serde_json::Value>,
    /// Campo `pub link_type`.
    pub link_type: Option<String>,
    /// Campo `pub created_at`.
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// Tipo `CreateEpicLinkPayload`.
pub struct CreateEpicLinkPayload {
    /// Campo `pub target_epic_iid`.
    pub target_epic_iid: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Campo `pub link_type`.
    pub link_type: Option<String>,
}
