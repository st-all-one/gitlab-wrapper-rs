use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use std::sync::Arc;
use crate::types::*;
use crate::utils::encoding::filter_to_query;

#[derive(Debug)]
pub struct GroupsResource {
    http: Arc<HttpClient>,
}

impl GroupsResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    pub fn list(&self, filter: Option<&GroupFilter>) -> Result<Vec<Group>, GitLabError> {
        let query = filter_to_query(filter);
        self.http.get("groups", &query, "groups.list")
    }

    pub fn get(&self, group_id: u64) -> Result<Group, GitLabError> {
        let path = format!("groups/{}", group_id);
        self.http.get(&path, &[], "groups.get")
    }

    pub fn create(&self, payload: &CreateGroupPayload) -> Result<Group, GitLabError> {
        self.http.post("groups", &payload, "groups.create")
    }

    pub fn delete(&self, group_id: u64) -> Result<(), GitLabError> {
        let path = format!("groups/{}", group_id);
        self.http.delete(&path, &[], "groups.delete")
    }

    pub fn subgroups(&self, group_id: u64) -> Result<Vec<Group>, GitLabError> {
        let path = format!("groups/{}/subgroups", group_id);
        self.http.get(&path, &[], "groups.subgroups")
    }

    pub fn descendant_groups(&self, group_id: u64) -> Result<Vec<Group>, GitLabError> {
        let path = format!("groups/{}/descendant_groups", group_id);
        self.http.get(&path, &[], "groups.descendant_groups")
    }

    pub fn projects(&self, group_id: u64) -> Result<Vec<Project>, GitLabError> {
        let path = format!("groups/{}/projects", group_id);
        self.http.get(&path, &[], "groups.projects")
    }
}
