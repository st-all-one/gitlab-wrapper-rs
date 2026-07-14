use serde::{Deserialize, Serialize};

/// Resposta genérica de um tipo de pacote no GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PackageTypeVersion {
    /// Campo `pub version`.
    pub version: String,
    /// Campo `pub created_at`.
    pub created_at: Option<String>,
    /// Campo `pub status`.
    pub status: Option<String>,
    /// Campo `pub tags`.
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// Tipo `PackageTypeFilter`.
pub struct PackageTypeFilter {
    /// Campo `pub page`.
    pub page: Option<u32>,
    /// Campo `pub per_page`.
    pub per_page: Option<u32>,
}
