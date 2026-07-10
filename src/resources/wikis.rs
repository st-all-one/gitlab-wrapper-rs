use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use std::sync::Arc;
use crate::types::*;
use crate::utils::encoding::encode_query_param;

#[derive(Debug)]
pub struct WikisResource {
    http: Arc<HttpClient>,
}

impl WikisResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    pub fn list(&self, project_id: u64) -> Result<Vec<WikiPage>, GitLabError> {
        let path = format!("projects/{}/wikis", project_id);
        self.http.get(&path, &[], "wikis.list")
    }

    pub fn get(&self, project_id: u64, slug: &str) -> Result<WikiPage, GitLabError> {
        let path = format!("projects/{}/wikis/{}", project_id, encode_query_param(slug));
        self.http.get(&path, &[], "wikis.get")
    }

    pub fn create(&self, project_id: u64, payload: &CreateWikiPagePayload) -> Result<WikiPage, GitLabError> {
        let path = format!("projects/{}/wikis", project_id);
        self.http.post(&path, &payload, "wikis.create")
    }

    pub fn update(&self, project_id: u64, slug: &str, payload: &UpdateWikiPagePayload) -> Result<WikiPage, GitLabError> {
        let path = format!("projects/{}/wikis/{}", project_id, encode_query_param(slug));
        self.http.put(&path, &payload, "wikis.update")
    }

    pub fn delete(&self, project_id: u64, slug: &str) -> Result<(), GitLabError> {
        let path = format!("projects/{}/wikis/{}", project_id, encode_query_param(slug));
        self.http.delete(&path, &[], "wikis.delete")
    }

    pub fn upload_attachment(&self, _project_id: u64, _file_path: &str) -> Result<serde_json::Value, GitLabError> {
        Err(GitLabError::Config("Wiki attachment upload requires multipart - not supported via blocking HTTP client".into()))
    }
}
