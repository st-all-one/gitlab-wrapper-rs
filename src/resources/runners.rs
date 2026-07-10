use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use std::sync::Arc;
use crate::types::*;

#[derive(Debug)]
pub struct RunnersResource {
    http: Arc<HttpClient>,
}

impl RunnersResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    pub fn list(&self) -> Result<Vec<Runner>, GitLabError> {
        self.http.get("runners", &[], "runners.list")
    }

    pub fn get(&self, runner_id: u64) -> Result<Runner, GitLabError> {
        let path = format!("runners/{}", runner_id);
        self.http.get(&path, &[], "runners.get")
    }

    pub fn create(&self, payload: &CreateRunnerPayload) -> Result<Runner, GitLabError> {
        self.http.post("runners", &payload, "runners.create")
    }

    pub fn update(&self, runner_id: u64, payload: &UpdateRunnerPayload) -> Result<Runner, GitLabError> {
        let path = format!("runners/{}", runner_id);
        self.http.put(&path, &payload, "runners.update")
    }

    pub fn delete(&self, runner_id: u64) -> Result<(), GitLabError> {
        let path = format!("runners/{}", runner_id);
        self.http.delete(&path, &[], "runners.delete")
    }

    pub fn list_jobs(&self, runner_id: u64) -> Result<Vec<Job>, GitLabError> {
        let path = format!("runners/{}/jobs", runner_id);
        self.http.get(&path, &[], "runners.list_jobs")
    }
}
