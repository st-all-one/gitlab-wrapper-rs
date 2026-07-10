use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Tag {
    pub name: String,
    pub message: Option<String>,
    pub target: Option<String>,
    pub commit: Option<TagCommit>,
    pub release: Option<TagRelease>,
    pub protected: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TagCommit {
    pub id: Option<String>,
    pub short_id: Option<String>,
    pub title: Option<String>,
    pub created_at: Option<String>,
    pub author_name: Option<String>,
    pub author_email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TagRelease {
    pub tag_name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateTagPayload {
    pub tag_name: String,
    pub ref_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_description: Option<String>,
}
