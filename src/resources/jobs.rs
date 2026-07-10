use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use std::sync::Arc;
use crate::types::*;
use crate::utils::encoding::filter_to_query;

#[derive(Debug)]
pub struct JobsResource {
    http: Arc<HttpClient>,
}

impl JobsResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    pub fn list(&self, project_id: u64, filter: Option<&JobFilter>) -> Result<Vec<Job>, GitLabError> {
        let path = format!("projects/{}/jobs", project_id);
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "jobs.list")
    }

    pub fn list_by_pipeline(&self, project_id: u64, pipeline_id: u64, filter: Option<&JobFilter>) -> Result<Vec<Job>, GitLabError> {
        let path = format!("projects/{}/pipelines/{}/jobs", project_id, pipeline_id);
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "jobs.list_by_pipeline")
    }

    pub fn get(&self, project_id: u64, job_id: u64) -> Result<Job, GitLabError> {
        let path = format!("projects/{}/jobs/{}", project_id, job_id);
        self.http.get(&path, &[], "jobs.get")
    }

    pub fn trace(&self, project_id: u64, job_id: u64) -> Result<String, GitLabError> {
        let path = format!("projects/{}/jobs/{}/trace", project_id, job_id);
        self.http.get_raw_text(&path, &[], "jobs.trace")
    }

    pub fn cancel(&self, project_id: u64, job_id: u64) -> Result<Job, GitLabError> {
        let path = format!("projects/{}/jobs/{}/cancel", project_id, job_id);
        self.http.post(&path, &serde_json::Value::Null, "jobs.cancel")
    }

    pub fn retry(&self, project_id: u64, job_id: u64) -> Result<Job, GitLabError> {
        let path = format!("projects/{}/jobs/{}/retry", project_id, job_id);
        self.http.post(&path, &serde_json::Value::Null, "jobs.retry")
    }

    pub fn play(&self, project_id: u64, job_id: u64) -> Result<Job, GitLabError> {
        let path = format!("projects/{}/jobs/{}/play", project_id, job_id);
        self.http.post(&path, &serde_json::Value::Null, "jobs.play")
    }

    pub fn erase(&self, project_id: u64, job_id: u64) -> Result<Job, GitLabError> {
        let path = format!("projects/{}/jobs/{}/erase", project_id, job_id);
        self.http.post(&path, &serde_json::Value::Null, "jobs.erase")
    }

    pub fn artifacts(&self, project_id: u64, job_id: u64) -> Result<Vec<u8>, GitLabError> {
        let path = format!("projects/{}/jobs/{}/artifacts", project_id, job_id);
        self.http.get_raw(&path, &[], "jobs.artifacts")
    }
}
