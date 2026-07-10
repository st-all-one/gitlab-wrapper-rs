use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use std::sync::Arc;
use crate::types::*;
use crate::utils::encoding::encode_query_param;

#[derive(Debug)]
pub struct LabelsResource {
    http: Arc<HttpClient>,
}

impl LabelsResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    pub fn list_project_labels(&self, project_id: u64) -> Result<Vec<Label>, GitLabError> {
        let path = format!("projects/{}/labels", project_id);
        self.http.get(&path, &[], "labels.list_project")
    }

    pub fn create_project_label(&self, project_id: u64, payload: &CreateLabelPayload) -> Result<Label, GitLabError> {
        let path = format!("projects/{}/labels", project_id);
        self.http.post(&path, &payload, "labels.create_project")
    }

    pub fn update_project_label(&self, project_id: u64, payload: &UpdateLabelPayload) -> Result<Label, GitLabError> {
        let path = format!("projects/{}/labels", project_id);
        self.http.put(&path, &payload, "labels.update_project")
    }

    pub fn delete_project_label(&self, project_id: u64, name: &str) -> Result<(), GitLabError> {
        let path = format!("projects/{}/labels/{}", project_id, encode_query_param(name));
        self.http.delete(&path, &[], "labels.delete_project")
    }

    pub fn promote_project_label(&self, project_id: u64, name: &str) -> Result<GroupLabel, GitLabError> {
        let path = format!("projects/{}/labels/{}/promote", project_id, encode_query_param(name));
        self.http.put(&path, &serde_json::Value::Null, "labels.promote_project")
    }

    pub fn list_group_labels(&self, group_id: u64) -> Result<Vec<GroupLabel>, GitLabError> {
        let path = format!("groups/{}/labels", group_id);
        self.http.get(&path, &[], "labels.list_group")
    }

    pub fn create_group_label(&self, group_id: u64, payload: &CreateLabelPayload) -> Result<GroupLabel, GitLabError> {
        let path = format!("groups/{}/labels", group_id);
        self.http.post(&path, &payload, "labels.create_group")
    }

    pub fn update_group_label(&self, group_id: u64, payload: &UpdateLabelPayload) -> Result<GroupLabel, GitLabError> {
        let path = format!("groups/{}/labels", group_id);
        self.http.put(&path, &payload, "labels.update_group")
    }

    pub fn delete_group_label(&self, group_id: u64, name: &str) -> Result<(), GitLabError> {
        let path = format!("groups/{}/labels/{}", group_id, encode_query_param(name));
        self.http.delete(&path, &[], "labels.delete_group")
    }
}
