use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Branch {
    pub name: String,
    pub merged: Option<bool>,
    pub protected: Option<bool>,
    pub default: Option<bool>,
    pub can_push: Option<bool>,
    pub developers_can_push: Option<bool>,
    pub developers_can_merge: Option<bool>,
    pub commit: Option<BranchCommit>,
    pub web_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BranchCommit {
    pub id: Option<String>,
    pub short_id: Option<String>,
    pub title: Option<String>,
    pub message: Option<String>,
    pub author_name: Option<String>,
    pub author_email: Option<String>,
    pub authored_date: Option<String>,
    pub committer_name: Option<String>,
    pub committer_email: Option<String>,
    pub committed_date: Option<String>,
    pub web_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateBranchPayload {
    pub branch: String,
    pub ref_: String,
}
