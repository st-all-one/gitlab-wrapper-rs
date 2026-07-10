use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RepositoryFile {
    pub id: Option<String>,
    pub file_name: Option<String>,
    pub file_path: Option<String>,
    pub size: Option<u64>,
    pub encoding: Option<String>,
    pub content: Option<String>,
    pub ref_: Option<String>,
    pub blob_id: Option<String>,
    pub commit_id: Option<String>,
    pub last_commit_id: Option<String>,
    pub execute_filemode: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateFilePayload {
    pub branch: String,
    pub content: String,
    pub commit_message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateFilePayload {
    pub branch: String,
    pub content: String,
    pub commit_message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_commit_id: Option<String>,
}
