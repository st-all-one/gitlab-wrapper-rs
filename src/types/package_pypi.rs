use serde::{Deserialize, Serialize};

/// Versão de pacote PyPi no GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PypiPackage {
    /// Campo `pub version`.
    pub version: String,
    /// Campo `pub created_at`.
    pub created_at: Option<String>,
    /// Campo `pub status`.
    pub status: Option<String>,
}
