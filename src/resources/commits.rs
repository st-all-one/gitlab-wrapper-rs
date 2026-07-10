use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use std::sync::Arc;
use crate::types::*;
use crate::utils::encoding::filter_to_query;

#[derive(Debug)]
pub struct CommitsResource {
    http: Arc<HttpClient>,
}

impl CommitsResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    pub fn list(&self, project_id: u64, filter: Option<&CommitFilter>) -> Result<Vec<Commit>, GitLabError> {
        let path = format!("projects/{}/repository/commits", project_id);
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "commits.list")
    }

    pub fn get(&self, project_id: u64, sha: &str) -> Result<Commit, GitLabError> {
        let path = format!("projects/{}/repository/commits/{}", project_id, sha);
        self.http.get(&path, &[], "commits.get")
    }

    pub fn create(&self, project_id: u64, payload: &CreateCommitPayload) -> Result<Commit, GitLabError> {
        let path = format!("projects/{}/repository/commits", project_id);
        self.http.post(&path, &payload, "commits.create")
    }

    pub fn cherry_pick(&self, project_id: u64, sha: &str, target_branch: &str) -> Result<Commit, GitLabError> {
        let path = format!("projects/{}/repository/commits/{}/cherry_pick", project_id, sha);
        let body = serde_json::json!({ "branch": target_branch });
        self.http.post(&path, &body, "commits.cherry_pick")
    }

    pub fn revert(&self, project_id: u64, sha: &str, target_branch: &str) -> Result<Commit, GitLabError> {
        let path = format!("projects/{}/repository/commits/{}/revert", project_id, sha);
        let body = serde_json::json!({ "branch": target_branch });
        self.http.post(&path, &body, "commits.revert")
    }

    pub fn diff(&self, project_id: u64, sha: &str) -> Result<Vec<CommitDiff>, GitLabError> {
        let path = format!("projects/{}/repository/commits/{}/diff", project_id, sha);
        self.http.get(&path, &[], "commits.diff")
    }

    pub fn refs(&self, project_id: u64, sha: &str) -> Result<serde_json::Value, GitLabError> {
        let path = format!("projects/{}/repository/commits/{}/refs", project_id, sha);
        self.http.get(&path, &[], "commits.refs")
    }

    pub fn comments(&self, project_id: u64, sha: &str) -> Result<Vec<Note>, GitLabError> {
        let path = format!("projects/{}/repository/commits/{}/comments", project_id, sha);
        self.http.get(&path, &[], "commits.comments")
    }

    pub fn add_comment(&self, project_id: u64, sha: &str, note: &str) -> Result<Note, GitLabError> {
        let path = format!("projects/{}/repository/commits/{}/comments", project_id, sha);
        let body = serde_json::json!({ "note": note });
        self.http.post(&path, &body, "commits.add_comment")
    }
}
