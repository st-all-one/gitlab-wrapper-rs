use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use std::sync::Arc;
use crate::types::*;

#[derive(Debug)]
pub struct DeployKeysResource {
    http: Arc<HttpClient>,
}

impl DeployKeysResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    pub fn list(&self, project_id: u64) -> Result<Vec<DeployKey>, GitLabError> {
        let path = format!("projects/{}/deploy_keys", project_id);
        self.http.get(&path, &[], "deploy_keys.list")
    }

    pub fn get(&self, project_id: u64, key_id: u64) -> Result<DeployKey, GitLabError> {
        let path = format!("projects/{}/deploy_keys/{}", project_id, key_id);
        self.http.get(&path, &[], "deploy_keys.get")
    }

    pub fn create(&self, project_id: u64, payload: &CreateDeployKeyPayload) -> Result<DeployKey, GitLabError> {
        let path = format!("projects/{}/deploy_keys", project_id);
        self.http.post(&path, &payload, "deploy_keys.create")
    }

    pub fn delete(&self, project_id: u64, key_id: u64) -> Result<(), GitLabError> {
        let path = format!("projects/{}/deploy_keys/{}", project_id, key_id);
        self.http.delete(&path, &[], "deploy_keys.delete")
    }

    pub fn enable(&self, project_id: u64, key_id: u64) -> Result<DeployKey, GitLabError> {
        let path = format!("projects/{}/deploy_keys/{}/enable", project_id, key_id);
        self.http.post(&path, &serde_json::Value::Null, "deploy_keys.enable")
    }
}
