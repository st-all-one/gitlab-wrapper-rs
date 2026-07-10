use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use std::sync::Arc;
use crate::types::*;
use crate::utils::encoding::encode_query_param;

#[derive(Debug)]
pub struct BranchesResource {
    http: Arc<HttpClient>,
}

impl BranchesResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    pub fn list(&self, project_id: u64) -> Result<Vec<Branch>, GitLabError> {
        let path = format!("projects/{}/repository/branches", project_id);
        self.http.get(&path, &[], "branches.list")
    }

    pub fn get(&self, project_id: u64, branch: &str) -> Result<Branch, GitLabError> {
        let path = format!("projects/{}/repository/branches/{}", project_id, encode_query_param(branch));
        self.http.get(&path, &[], "branches.get")
    }

    pub fn create(&self, project_id: u64, payload: &CreateBranchPayload) -> Result<Branch, GitLabError> {
        let path = format!("projects/{}/repository/branches", project_id);
        self.http.post(&path, &payload, "branches.create")
    }

    pub fn delete(&self, project_id: u64, branch: &str) -> Result<(), GitLabError> {
        let path = format!("projects/{}/repository/branches/{}", project_id, encode_query_param(branch));
        self.http.delete(&path, &[], "branches.delete")
    }

    pub fn delete_merged(&self, project_id: u64) -> Result<(), GitLabError> {
        let path = format!("projects/{}/repository/merged_branches", project_id);
        self.http.delete(&path, &[], "branches.delete_merged")
    }
}
