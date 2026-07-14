use crate::types::base::GitLabId;
use serde::{Deserialize, Serialize};

/// Issue board no GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Board {
    /// Campo `pub id`.
    pub id: GitLabId,
    /// Campo `pub name`.
    pub name: Option<String>,
    /// Campo `pub project`.
    pub project: Option<serde_json::Value>,
    /// Campo `pub group`.
    pub group: Option<serde_json::Value>,
    /// Campo `pub lists`.
    pub lists: Option<Vec<BoardList>>,
}

/// Lista de um issue board.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BoardList {
    /// Campo `pub id`.
    pub id: GitLabId,
    /// Campo `pub label`.
    pub label: Option<serde_json::Value>,
    /// Campo `pub position`.
    pub position: Option<i32>,
    /// Campo `pub max_issue_count`.
    pub max_issue_count: Option<i32>,
    /// Campo `pub max_issue_weight`.
    pub max_issue_weight: Option<i32>,
}

/// Payload para criar uma lista em um board.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateBoardListPayload {
    /// Campo `pub label_id`.
    pub label_id: GitLabId,
}

/// Payload para atualizar uma lista de board.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateBoardListPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Campo `pub position`.
    pub position: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Campo `pub max_issue_count`.
    pub max_issue_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Campo `pub max_issue_weight`.
    pub max_issue_weight: Option<i32>,
}
