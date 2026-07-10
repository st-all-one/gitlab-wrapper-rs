use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use std::sync::Arc;
use crate::types::*;

#[derive(Debug)]
pub struct EnvironmentsResource {
    http: Arc<HttpClient>,
}

impl EnvironmentsResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    pub fn list(&self, project_id: u64) -> Result<Vec<Environment>, GitLabError> {
        let path = format!("projects/{}/environments", project_id);
        self.http.get(&path, &[], "environments.list")
    }

    pub fn get(&self, project_id: u64, env_id: u64) -> Result<Environment, GitLabError> {
        let path = format!("projects/{}/environments/{}", project_id, env_id);
        self.http.get(&path, &[], "environments.get")
    }

    pub fn create(&self, project_id: u64, payload: &CreateEnvironmentPayload) -> Result<Environment, GitLabError> {
        let path = format!("projects/{}/environments", project_id);
        self.http.post(&path, &payload, "environments.create")
    }

    pub fn update(&self, project_id: u64, env_id: u64, payload: &UpdateEnvironmentPayload) -> Result<Environment, GitLabError> {
        let path = format!("projects/{}/environments/{}", project_id, env_id);
        self.http.put(&path, &payload, "environments.update")
    }

    pub fn delete(&self, project_id: u64, env_id: u64) -> Result<(), GitLabError> {
        let path = format!("projects/{}/environments/{}", project_id, env_id);
        self.http.delete(&path, &[], "environments.delete")
    }

    pub fn stop(&self, project_id: u64, env_id: u64) -> Result<Environment, GitLabError> {
        let path = format!("projects/{}/environments/{}/stop", project_id, env_id);
        self.http.post(&path, &serde_json::Value::Null, "environments.stop")
    }
}
