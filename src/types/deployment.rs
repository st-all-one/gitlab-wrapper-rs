use crate::types::base::GitLabId;
use serde::{Deserialize, Serialize};

/// Deployment no GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Deployment {
    /// Campo `pub id`.
    pub id: GitLabId,
    /// Campo `pub iid`.
    pub iid: Option<u32>,
    /// Campo `pub ref_`.
    pub ref_: Option<String>,
    /// Campo `pub sha`.
    pub sha: Option<String>,
    /// Campo `pub status`.
    pub status: Option<String>,
    /// Campo `pub created_at`.
    pub created_at: Option<String>,
    /// Campo `pub updated_at`.
    pub updated_at: Option<String>,
    /// Campo `pub finished_at`.
    pub finished_at: Option<String>,
    /// Campo `pub environment`.
    pub environment: Option<serde_json::Value>,
    /// Campo `pub deployable`.
    pub deployable: Option<serde_json::Value>,
    /// Campo `pub user`.
    pub user: Option<serde_json::Value>,
}

/// Filtro para listar deployments.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DeploymentFilter {
    /// Campo `pub page`.
    pub page: Option<u32>,
    /// Campo `pub per_page`.
    pub per_page: Option<u32>,
    /// Campo `pub order_by`.
    pub order_by: Option<String>,
    /// Campo `pub sort`.
    pub sort: Option<String>,
    /// Campo `pub environment`.
    pub environment: Option<String>,
    /// Campo `pub status`.
    pub status: Option<String>,
}
