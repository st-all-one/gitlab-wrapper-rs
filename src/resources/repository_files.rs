use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use std::sync::Arc;
use crate::types::*;
use crate::utils::encoding::encode_query_param;

#[derive(Debug)]
pub struct RepositoryFilesResource {
    http: Arc<HttpClient>,
}

impl RepositoryFilesResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    pub fn get(&self, project_id: u64, file_path: &str, ref_: &str) -> Result<RepositoryFile, GitLabError> {
        let path = format!("projects/{}/repository/files/{}", project_id, encode_query_param(file_path));
        let query = vec![("ref".to_string(), ref_.to_string())];
        self.http.get(&path, &query, "repository_files.get")
    }

    pub fn raw(&self, project_id: u64, file_path: &str, ref_: &str) -> Result<String, GitLabError> {
        let path = format!("projects/{}/repository/files/{}/raw", project_id, encode_query_param(file_path));
        let query = vec![("ref".to_string(), ref_.to_string())];
        self.http.get_raw_text(&path, &query, "repository_files.raw")
    }

    pub fn blame(&self, project_id: u64, file_path: &str, ref_: &str) -> Result<serde_json::Value, GitLabError> {
        let path = format!("projects/{}/repository/files/{}/blame", project_id, encode_query_param(file_path));
        let query = vec![("ref".to_string(), ref_.to_string())];
        self.http.get(&path, &query, "repository_files.blame")
    }

    pub fn create(&self, project_id: u64, file_path: &str, payload: &CreateFilePayload) -> Result<RepositoryFile, GitLabError> {
        let path = format!("projects/{}/repository/files/{}", project_id, encode_query_param(file_path));
        self.http.post(&path, &payload, "repository_files.create")
    }

    pub fn update(&self, project_id: u64, file_path: &str, payload: &UpdateFilePayload) -> Result<RepositoryFile, GitLabError> {
        let path = format!("projects/{}/repository/files/{}", project_id, encode_query_param(file_path));
        self.http.put(&path, &payload, "repository_files.update")
    }

    pub fn delete(&self, project_id: u64, file_path: &str, branch: &str, commit_message: &str) -> Result<(), GitLabError> {
        let path = format!("projects/{}/repository/files/{}", project_id, encode_query_param(file_path));
        let body = serde_json::json!({ "branch": branch, "commit_message": commit_message });
        self.http.delete_with_body(&path, &body, "repository_files.delete")
    }
}
