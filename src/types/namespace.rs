use crate::types::base::GitLabId;
use serde::{Deserialize, Serialize};

/// Namespace no GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Namespace {
    /// Campo `pub id`.
    pub id: GitLabId,
    /// Campo `pub name`.
    pub name: String,
    /// Campo `pub path`.
    pub path: String,
    /// Campo `pub kind`.
    pub kind: Option<String>,
    /// Campo `pub full_path`.
    pub full_path: Option<String>,
    /// Campo `pub parent_id`.
    pub parent_id: Option<GitLabId>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// Tipo `NamespaceFilter`.
pub struct NamespaceFilter {
    /// Campo `pub page`.
    pub page: Option<u32>,
    /// Campo `pub per_page`.
    pub per_page: Option<u32>,
    /// Campo `pub search`.
    pub search: Option<String>,
}
