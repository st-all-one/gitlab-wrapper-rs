use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use std::sync::Arc;
use crate::types::*;
use crate::utils::encoding::filter_to_query;

#[derive(Debug)]
pub struct IssuesResource {
    http: Arc<HttpClient>,
}

impl IssuesResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    pub fn list(&self, filter: Option<&IssueFilter>) -> Result<Vec<Issue>, GitLabError> {
        let query = filter_to_query(filter);
        self.http.get("issues", &query, "issues.list")
    }

    pub fn list_for_project(&self, project_id: u64, filter: Option<&IssueFilter>) -> Result<Vec<Issue>, GitLabError> {
        let path = format!("projects/{}/issues", project_id);
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "issues.list_for_project")
    }

    pub fn get(&self, project_id: u64, issue_iid: u32) -> Result<Issue, GitLabError> {
        let path = format!("projects/{}/issues/{}", project_id, issue_iid);
        self.http.get(&path, &[], "issues.get")
    }

    pub fn create(&self, project_id: u64, payload: &CreateIssuePayload) -> Result<Issue, GitLabError> {
        let path = format!("projects/{}/issues", project_id);
        self.http.post(&path, &payload, "issues.create")
    }

    pub fn update(&self, project_id: u64, issue_iid: u32, payload: &UpdateIssuePayload) -> Result<Issue, GitLabError> {
        let path = format!("projects/{}/issues/{}", project_id, issue_iid);
        self.http.put(&path, &payload, "issues.update")
    }

    pub fn delete(&self, project_id: u64, issue_iid: u32) -> Result<(), GitLabError> {
        let path = format!("projects/{}/issues/{}", project_id, issue_iid);
        self.http.delete(&path, &[], "issues.delete")
    }

    pub fn subscribe(&self, project_id: u64, issue_iid: u32) -> Result<Issue, GitLabError> {
        let path = format!("projects/{}/issues/{}/subscribe", project_id, issue_iid);
        self.http.post(&path, &serde_json::Value::Null, "issues.subscribe")
    }

    pub fn unsubscribe(&self, project_id: u64, issue_iid: u32) -> Result<Issue, GitLabError> {
        let path = format!("projects/{}/issues/{}/unsubscribe", project_id, issue_iid);
        self.http.post(&path, &serde_json::Value::Null, "issues.unsubscribe")
    }

    pub fn set_time_estimate(&self, project_id: u64, issue_iid: u32, duration: &str) -> Result<Issue, GitLabError> {
        let path = format!("projects/{}/issues/{}/time_estimate", project_id, issue_iid);
        let body = serde_json::json!({ "duration": duration });
        self.http.post(&path, &body, "issues.set_time_estimate")
    }

    pub fn add_spent_time(&self, project_id: u64, issue_iid: u32, duration: &str) -> Result<Issue, GitLabError> {
        let path = format!("projects/{}/issues/{}/add_spent_time", project_id, issue_iid);
        let body = serde_json::json!({ "duration": duration });
        self.http.post(&path, &body, "issues.add_spent_time")
    }

    pub fn move_issue(&self, project_id: u64, issue_iid: u32, to_project_id: u64) -> Result<Issue, GitLabError> {
        let path = format!("projects/{}/issues/{}/move", project_id, issue_iid);
        let body = serde_json::json!({ "to_project_id": to_project_id });
        self.http.post(&path, &body, "issues.move")
    }

    pub fn get_by_group(&self, group_id: u64, filter: Option<&IssueFilter>) -> Result<Vec<Issue>, GitLabError> {
        let path = format!("groups/{}/issues", group_id);
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "issues.get_by_group")
    }
}
