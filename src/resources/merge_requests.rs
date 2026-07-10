use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use std::sync::Arc;
use crate::types::*;
use crate::utils::encoding::filter_to_query;

#[derive(Debug)]
pub struct MergeRequestsResource {
    http: Arc<HttpClient>,
}

impl MergeRequestsResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    pub fn list(&self, filter: Option<&MergeRequestFilter>) -> Result<Vec<MergeRequest>, GitLabError> {
        let query = filter_to_query(filter);
        self.http.get("merge_requests", &query, "merge_requests.list")
    }

    pub fn list_for_project(&self, project_id: u64, filter: Option<&MergeRequestFilter>) -> Result<Vec<MergeRequest>, GitLabError> {
        let path = format!("projects/{}/merge_requests", project_id);
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "merge_requests.list_for_project")
    }

    pub fn get(&self, project_id: u64, mr_iid: u32) -> Result<MergeRequest, GitLabError> {
        let path = format!("projects/{}/merge_requests/{}", project_id, mr_iid);
        self.http.get(&path, &[], "merge_requests.get")
    }

    pub fn create(&self, project_id: u64, payload: &CreateMergeRequestPayload) -> Result<MergeRequest, GitLabError> {
        let path = format!("projects/{}/merge_requests", project_id);
        self.http.post(&path, &payload, "merge_requests.create")
    }

    pub fn update(&self, project_id: u64, mr_iid: u32, payload: &UpdateMergeRequestPayload) -> Result<MergeRequest, GitLabError> {
        let path = format!("projects/{}/merge_requests/{}", project_id, mr_iid);
        self.http.put(&path, &payload, "merge_requests.update")
    }

    pub fn delete(&self, project_id: u64, mr_iid: u32) -> Result<(), GitLabError> {
        let path = format!("projects/{}/merge_requests/{}", project_id, mr_iid);
        self.http.delete(&path, &[], "merge_requests.delete")
    }

    pub fn merge(&self, project_id: u64, mr_iid: u32, payload: Option<&MergePayload>) -> Result<MergeRequest, GitLabError> {
        let path = format!("projects/{}/merge_requests/{}/merge", project_id, mr_iid);
        let body = payload.unwrap_or(&MergePayload {
            merge_commit_message: None,
            squash_commit_message: None,
            should_remove_source_branch: None,
        });
        self.http.put(&path, &body, "merge_requests.merge")
    }

    pub fn cancel_merge_when_pipeline_succeeds(&self, project_id: u64, mr_iid: u32) -> Result<MergeRequest, GitLabError> {
        let path = format!("projects/{}/merge_requests/{}/cancel_merge_when_pipeline_succeeds", project_id, mr_iid);
        self.http.post(&path, &serde_json::Value::Null, "merge_requests.cancel_merge_when_pipeline_succeeds")
    }

    pub fn commits(&self, project_id: u64, mr_iid: u32) -> Result<Vec<Commit>, GitLabError> {
        let path = format!("projects/{}/merge_requests/{}/commits", project_id, mr_iid);
        self.http.get(&path, &[], "merge_requests.commits")
    }

    pub fn changes(&self, project_id: u64, mr_iid: u32) -> Result<serde_json::Value, GitLabError> {
        let path = format!("projects/{}/merge_requests/{}/changes", project_id, mr_iid);
        self.http.get(&path, &[], "merge_requests.changes")
    }

    pub fn list_by_group(&self, group_id: u64, filter: Option<&MergeRequestFilter>) -> Result<Vec<MergeRequest>, GitLabError> {
        let path = format!("groups/{}/merge_requests", group_id);
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "merge_requests.list_by_group")
    }
}
