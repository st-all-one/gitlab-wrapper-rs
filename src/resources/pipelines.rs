use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use std::sync::Arc;
use crate::types::*;
use crate::utils::encoding::filter_to_query;

#[derive(Debug)]
pub struct PipelinesResource {
    http: Arc<HttpClient>,
}

impl PipelinesResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    pub fn list(&self, project_id: u64, filter: Option<&PipelineFilter>) -> Result<Vec<Pipeline>, GitLabError> {
        let path = format!("projects/{}/pipelines", project_id);
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "pipelines.list")
    }

    pub fn get(&self, project_id: u64, pipeline_id: u64) -> Result<Pipeline, GitLabError> {
        let path = format!("projects/{}/pipelines/{}", project_id, pipeline_id);
        self.http.get(&path, &[], "pipelines.get")
    }

    pub fn get_latest(&self, project_id: u64) -> Result<Pipeline, GitLabError> {
        let path = format!("projects/{}/pipelines/latest", project_id);
        self.http.get(&path, &[], "pipelines.get_latest")
    }

    pub fn create(&self, project_id: u64, payload: &CreatePipelinePayload) -> Result<Pipeline, GitLabError> {
        let path = format!("projects/{}/pipeline", project_id);
        self.http.post(&path, &payload, "pipelines.create")
    }

    pub fn retry(&self, project_id: u64, pipeline_id: u64) -> Result<Pipeline, GitLabError> {
        let path = format!("projects/{}/pipelines/{}/retry", project_id, pipeline_id);
        self.http.post(&path, &serde_json::Value::Null, "pipelines.retry")
    }

    pub fn cancel(&self, project_id: u64, pipeline_id: u64) -> Result<Pipeline, GitLabError> {
        let path = format!("projects/{}/pipelines/{}/cancel", project_id, pipeline_id);
        self.http.post(&path, &serde_json::Value::Null, "pipelines.cancel")
    }

    pub fn delete(&self, project_id: u64, pipeline_id: u64) -> Result<(), GitLabError> {
        let path = format!("projects/{}/pipelines/{}", project_id, pipeline_id);
        self.http.delete(&path, &[], "pipelines.delete")
    }

    pub fn variables(&self, project_id: u64, pipeline_id: u64) -> Result<Vec<PipelineVariable>, GitLabError> {
        let path = format!("projects/{}/pipelines/{}/variables", project_id, pipeline_id);
        self.http.get(&path, &[], "pipelines.variables")
    }

    pub fn test_report(&self, project_id: u64, pipeline_id: u64) -> Result<serde_json::Value, GitLabError> {
        let path = format!("projects/{}/pipelines/{}/test_report", project_id, pipeline_id);
        self.http.get(&path, &[], "pipelines.test_report")
    }
}
