use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use std::sync::Arc;
use crate::types::*;

#[derive(Debug)]
pub struct MembersResource {
    http: Arc<HttpClient>,
}

impl MembersResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    pub fn list_project_members(&self, project_id: u64) -> Result<Vec<Member>, GitLabError> {
        let path = format!("projects/{}/members", project_id);
        self.http.get(&path, &[], "members.list_project")
    }

    pub fn get_project_member(&self, project_id: u64, user_id: u64) -> Result<Member, GitLabError> {
        let path = format!("projects/{}/members/{}", project_id, user_id);
        self.http.get(&path, &[], "members.get_project")
    }

    pub fn add_project_member(&self, project_id: u64, payload: &AddMemberPayload) -> Result<Member, GitLabError> {
        let path = format!("projects/{}/members", project_id);
        self.http.post(&path, &payload, "members.add_project")
    }

    pub fn update_project_member(&self, project_id: u64, user_id: u64, payload: &UpdateMemberPayload) -> Result<Member, GitLabError> {
        let path = format!("projects/{}/members/{}", project_id, user_id);
        self.http.put(&path, &payload, "members.update_project")
    }

    pub fn delete_project_member(&self, project_id: u64, user_id: u64) -> Result<(), GitLabError> {
        let path = format!("projects/{}/members/{}", project_id, user_id);
        self.http.delete(&path, &[], "members.delete_project")
    }

    pub fn list_project_inherited_members(&self, project_id: u64) -> Result<Vec<Member>, GitLabError> {
        let path = format!("projects/{}/members/all", project_id);
        self.http.get(&path, &[], "members.list_project_inherited")
    }

    pub fn list_group_members(&self, group_id: u64) -> Result<Vec<Member>, GitLabError> {
        let path = format!("groups/{}/members", group_id);
        self.http.get(&path, &[], "members.list_group")
    }

    pub fn get_group_member(&self, group_id: u64, user_id: u64) -> Result<Member, GitLabError> {
        let path = format!("groups/{}/members/{}", group_id, user_id);
        self.http.get(&path, &[], "members.get_group")
    }

    pub fn add_group_member(&self, group_id: u64, payload: &AddMemberPayload) -> Result<Member, GitLabError> {
        let path = format!("groups/{}/members", group_id);
        self.http.post(&path, &payload, "members.add_group")
    }

    pub fn update_group_member(&self, group_id: u64, user_id: u64, payload: &UpdateMemberPayload) -> Result<Member, GitLabError> {
        let path = format!("groups/{}/members/{}", group_id, user_id);
        self.http.put(&path, &payload, "members.update_group")
    }

    pub fn delete_group_member(&self, group_id: u64, user_id: u64) -> Result<(), GitLabError> {
        let path = format!("groups/{}/members/{}", group_id, user_id);
        self.http.delete(&path, &[], "members.delete_group")
    }

    pub fn list_group_inherited_members(&self, group_id: u64) -> Result<Vec<Member>, GitLabError> {
        let path = format!("groups/{}/members/all", group_id);
        self.http.get(&path, &[], "members.list_group_inherited")
    }
}
