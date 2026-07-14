use crate::types::base::GitLabId;
use serde::{Deserialize, Serialize};

/// Merge train no GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MergeTrain {
    /// Campo `pub id`.
    pub id: GitLabId,
    /// Campo `pub merge_request`.
    pub merge_request: Option<serde_json::Value>,
    /// Campo `pub user`.
    pub user: Option<serde_json::Value>,
    /// Campo `pub pipeline`.
    pub pipeline: Option<serde_json::Value>,
    /// Campo `pub status`.
    pub status: Option<String>,
    /// Campo `pub created_at`.
    pub created_at: Option<String>,
    /// Campo `pub updated_at`.
    pub updated_at: Option<String>,
    /// Campo `pub target_branch`.
    pub target_branch: Option<String>,
    /// Campo `pub target_project_id`.
    pub target_project_id: Option<GitLabId>,
}

/// Filtro para listar merge trains.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MergeTrainFilter {
    /// Campo `pub page`.
    pub page: Option<u32>,
    /// Campo `pub per_page`.
    pub per_page: Option<u32>,
    /// Campo `pub scope`.
    pub scope: Option<String>,
    /// Campo `pub sort`.
    pub sort: Option<String>,
}
