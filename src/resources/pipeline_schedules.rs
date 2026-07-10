use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use std::sync::Arc;
use crate::types::*;

#[derive(Debug)]
pub struct PipelineSchedulesResource {
    http: Arc<HttpClient>,
}

impl PipelineSchedulesResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    pub fn list(&self, project_id: u64) -> Result<Vec<PipelineSchedule>, GitLabError> {
        let path = format!("projects/{}/pipeline_schedules", project_id);
        self.http.get(&path, &[], "pipeline_schedules.list")
    }

    pub fn get(&self, project_id: u64, schedule_id: u64) -> Result<PipelineSchedule, GitLabError> {
        let path = format!("projects/{}/pipeline_schedules/{}", project_id, schedule_id);
        self.http.get(&path, &[], "pipeline_schedules.get")
    }

    pub fn create(&self, project_id: u64, payload: &CreatePipelineSchedulePayload) -> Result<PipelineSchedule, GitLabError> {
        let path = format!("projects/{}/pipeline_schedules", project_id);
        self.http.post(&path, &payload, "pipeline_schedules.create")
    }

    pub fn update(&self, project_id: u64, schedule_id: u64, payload: &UpdatePipelineSchedulePayload) -> Result<PipelineSchedule, GitLabError> {
        let path = format!("projects/{}/pipeline_schedules/{}", project_id, schedule_id);
        self.http.put(&path, &payload, "pipeline_schedules.update")
    }

    pub fn delete(&self, project_id: u64, schedule_id: u64) -> Result<(), GitLabError> {
        let path = format!("projects/{}/pipeline_schedules/{}", project_id, schedule_id);
        self.http.delete(&path, &[], "pipeline_schedules.delete")
    }

    pub fn take_ownership(&self, project_id: u64, schedule_id: u64) -> Result<PipelineSchedule, GitLabError> {
        let path = format!("projects/{}/pipeline_schedules/{}/take_ownership", project_id, schedule_id);
        self.http.post(&path, &serde_json::Value::Null, "pipeline_schedules.take_ownership")
    }

    pub fn create_variable(&self, project_id: u64, schedule_id: u64, key: &str, value: &str) -> Result<PipelineScheduleVariable, GitLabError> {
        let path = format!("projects/{}/pipeline_schedules/{}/variables", project_id, schedule_id);
        let body = serde_json::json!({ "key": key, "value": value });
        self.http.post(&path, &body, "pipeline_schedules.create_variable")
    }

    pub fn update_variable(&self, project_id: u64, schedule_id: u64, variable_id: u64, value: &str) -> Result<PipelineScheduleVariable, GitLabError> {
        let path = format!("projects/{}/pipeline_schedules/{}/variables/{}", project_id, schedule_id, variable_id);
        let body = serde_json::json!({ "value": value });
        self.http.put(&path, &body, "pipeline_schedules.update_variable")
    }

    pub fn delete_variable(&self, project_id: u64, schedule_id: u64, variable_id: u64) -> Result<(), GitLabError> {
        let path = format!("projects/{}/pipeline_schedules/{}/variables/{}", project_id, schedule_id, variable_id);
        self.http.delete(&path, &[], "pipeline_schedules.delete_variable")
    }
}
