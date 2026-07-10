use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use std::sync::Arc;
use crate::types::*;
use crate::utils::encoding::encode_query_param;

#[derive(Debug)]
pub struct TagsResource {
    http: Arc<HttpClient>,
}

impl TagsResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    pub fn list(&self, project_id: u64) -> Result<Vec<Tag>, GitLabError> {
        let path = format!("projects/{}/repository/tags", project_id);
        self.http.get(&path, &[], "tags.list")
    }

    pub fn get(&self, project_id: u64, tag: &str) -> Result<Tag, GitLabError> {
        let path = format!("projects/{}/repository/tags/{}", project_id, encode_query_param(tag));
        self.http.get(&path, &[], "tags.get")
    }

    pub fn create(&self, project_id: u64, payload: &CreateTagPayload) -> Result<Tag, GitLabError> {
        let path = format!("projects/{}/repository/tags", project_id);
        self.http.post(&path, &payload, "tags.create")
    }

    pub fn delete(&self, project_id: u64, tag: &str) -> Result<(), GitLabError> {
        let path = format!("projects/{}/repository/tags/{}", project_id, encode_query_param(tag));
        self.http.delete(&path, &[], "tags.delete")
    }
}
