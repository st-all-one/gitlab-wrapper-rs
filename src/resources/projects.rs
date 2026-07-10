use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use std::sync::Arc;
use crate::types::*;
use crate::utils::encoding::filter_to_query;

#[derive(Debug)]
pub struct ProjectsResource {
    http: Arc<HttpClient>,
}

impl ProjectsResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    pub fn list(&self, filter: Option<&ProjectFilter>) -> Result<Vec<Project>, GitLabError> {
        let query = filter_to_query(filter);
        self.http.get("projects", &query, "projects.list")
    }

    pub fn list_all(&self, filter: Option<&ProjectFilter>) -> Result<Vec<Project>, GitLabError> {
        let query = filter_to_query(filter);
        self.http.paginate_all("projects", &query, "projects.list_all")
    }

    pub fn get(&self, project_id: u64) -> Result<Project, GitLabError> {
        let path = format!("projects/{}", project_id);
        self.http.get(&path, &[], "projects.get")
    }

    pub fn create(&self, payload: &CreateProjectPayload) -> Result<Project, GitLabError> {
        self.http.post("projects", &payload, "projects.create")
    }

    pub fn update(&self, project_id: u64, payload: &UpdateProjectPayload) -> Result<Project, GitLabError> {
        let path = format!("projects/{}", project_id);
        self.http.put(&path, &payload, "projects.update")
    }

    pub fn delete(&self, project_id: u64) -> Result<(), GitLabError> {
        let path = format!("projects/{}", project_id);
        self.http.delete(&path, &[], "projects.delete")
    }

    pub fn archive(&self, project_id: u64) -> Result<Project, GitLabError> {
        let path = format!("projects/{}/archive", project_id);
        self.http.post(&path, &serde_json::Value::Null, "projects.archive")
    }

    pub fn unarchive(&self, project_id: u64) -> Result<Project, GitLabError> {
        let path = format!("projects/{}/unarchive", project_id);
        self.http.post(&path, &serde_json::Value::Null, "projects.unarchive")
    }

    pub fn fork(&self, project_id: u64, _namespace: Option<&str>) -> Result<Project, GitLabError> {
        let path = format!("projects/{}/fork", project_id);
        self.http.post(&path, &serde_json::Value::Null, "projects.fork")
    }

    pub fn upload_avatar(&self, _project_id: u64, _file_path: &str) -> Result<Project, GitLabError> {
        Err(GitLabError::Config("Avatar upload requires multipart - not supported via blocking HTTP client".into()))
    }

    pub fn transfer(&self, project_id: u64, namespace_id: u64) -> Result<Project, GitLabError> {
        let path = format!("projects/{}/transfer", project_id);
        let body = serde_json::json!({ "namespace_id": namespace_id });
        self.http.put(&path, &body, "projects.transfer")
    }
}
