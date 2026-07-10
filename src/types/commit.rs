use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Commit {
    pub id: String,
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
    pub stats: Option<CommitStats>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CommitStats {
    pub additions: Option<u32>,
    pub deletions: Option<u32>,
    pub total: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateCommitPayload {
    pub branch: String,
    pub commit_message: String,
    pub actions: Vec<CommitAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CommitAction {
    pub action: String,
    pub file_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CommitDiff {
    pub diff: Option<String>,
    pub new_path: Option<String>,
    pub old_path: Option<String>,
    pub a_mode: Option<String>,
    pub b_mode: Option<String>,
    pub new_file: Option<bool>,
    pub renamed_file: Option<bool>,
    pub deleted_file: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CommitFilter {
    pub ref_name: Option<String>,
    pub path: Option<String>,
    pub since: Option<String>,
    pub until: Option<String>,
    pub author: Option<String>,
    pub with_stats: Option<bool>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}
