use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use std::sync::Arc;
use crate::types::*;
use crate::utils::encoding::encode_query_param;

#[derive(Debug)]
pub struct ReleasesResource {
    http: Arc<HttpClient>,
}

impl ReleasesResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    pub fn list(&self, project_id: u64) -> Result<Vec<Release>, GitLabError> {
        let path = format!("projects/{}/releases", project_id);
        self.http.get(&path, &[], "releases.list")
    }

    pub fn get(&self, project_id: u64, tag_name: &str) -> Result<Release, GitLabError> {
        let path = format!("projects/{}/releases/{}", project_id, encode_query_param(tag_name));
        self.http.get(&path, &[], "releases.get")
    }

    pub fn create(&self, project_id: u64, payload: &CreateReleasePayload) -> Result<Release, GitLabError> {
        let path = format!("projects/{}/releases", project_id);
        self.http.post(&path, &payload, "releases.create")
    }

    pub fn update(&self, project_id: u64, tag_name: &str, payload: &UpdateReleasePayload) -> Result<Release, GitLabError> {
        let path = format!("projects/{}/releases/{}", project_id, encode_query_param(tag_name));
        self.http.put(&path, &payload, "releases.update")
    }

    pub fn delete(&self, project_id: u64, tag_name: &str) -> Result<(), GitLabError> {
        let path = format!("projects/{}/releases/{}", project_id, encode_query_param(tag_name));
        self.http.delete(&path, &[], "releases.delete")
    }

    pub fn create_link(&self, project_id: u64, tag_name: &str, payload: &CreateReleaseLinkPayload) -> Result<ReleaseLinkItem, GitLabError> {
        let path = format!("projects/{}/releases/{}/assets/links", project_id, encode_query_param(tag_name));
        self.http.post(&path, &payload, "releases.create_link")
    }

    pub fn delete_link(&self, project_id: u64, tag_name: &str, link_id: u64) -> Result<(), GitLabError> {
        let path = format!("projects/{}/releases/{}/assets/links/{}", project_id, encode_query_param(tag_name), link_id);
        self.http.delete(&path, &[], "releases.delete_link")
    }
}
