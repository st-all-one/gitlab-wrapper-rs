use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use std::sync::Arc;
use crate::types::*;
use crate::utils::encoding::filter_to_query;

#[derive(Debug)]
pub struct MilestonesResource {
    http: Arc<HttpClient>,
}

impl MilestonesResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    pub fn list_project_milestones(&self, project_id: u64, filter: Option<&MilestoneFilter>) -> Result<Vec<Milestone>, GitLabError> {
        let path = format!("projects/{}/milestones", project_id);
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "milestones.list_project")
    }

    pub fn get_project_milestone(&self, project_id: u64, milestone_id: u64) -> Result<Milestone, GitLabError> {
        let path = format!("projects/{}/milestones/{}", project_id, milestone_id);
        self.http.get(&path, &[], "milestones.get_project")
    }

    pub fn create_project_milestone(&self, project_id: u64, payload: &CreateMilestonePayload) -> Result<Milestone, GitLabError> {
        let path = format!("projects/{}/milestones", project_id);
        self.http.post(&path, &payload, "milestones.create_project")
    }

    pub fn update_project_milestone(&self, project_id: u64, milestone_id: u64, payload: &UpdateMilestonePayload) -> Result<Milestone, GitLabError> {
        let path = format!("projects/{}/milestones/{}", project_id, milestone_id);
        self.http.put(&path, &payload, "milestones.update_project")
    }

    pub fn delete_project_milestone(&self, project_id: u64, milestone_id: u64) -> Result<(), GitLabError> {
        let path = format!("projects/{}/milestones/{}", project_id, milestone_id);
        self.http.delete(&path, &[], "milestones.delete_project")
    }

    pub fn list_project_milestone_issues(&self, project_id: u64, milestone_id: u64) -> Result<Vec<Issue>, GitLabError> {
        let path = format!("projects/{}/milestones/{}/issues", project_id, milestone_id);
        self.http.get(&path, &[], "milestones.list_project_issues")
    }

    pub fn list_group_milestones(&self, group_id: u64, filter: Option<&MilestoneFilter>) -> Result<Vec<Milestone>, GitLabError> {
        let path = format!("groups/{}/milestones", group_id);
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "milestones.list_group")
    }

    pub fn get_group_milestone(&self, group_id: u64, milestone_id: u64) -> Result<Milestone, GitLabError> {
        let path = format!("groups/{}/milestones/{}", group_id, milestone_id);
        self.http.get(&path, &[], "milestones.get_group")
    }

    pub fn create_group_milestone(&self, group_id: u64, payload: &CreateMilestonePayload) -> Result<Milestone, GitLabError> {
        let path = format!("groups/{}/milestones", group_id);
        self.http.post(&path, &payload, "milestones.create_group")
    }

    pub fn update_group_milestone(&self, group_id: u64, milestone_id: u64, payload: &UpdateMilestonePayload) -> Result<Milestone, GitLabError> {
        let path = format!("groups/{}/milestones/{}", group_id, milestone_id);
        self.http.put(&path, &payload, "milestones.update_group")
    }

    pub fn delete_group_milestone(&self, group_id: u64, milestone_id: u64) -> Result<(), GitLabError> {
        let path = format!("groups/{}/milestones/{}", group_id, milestone_id);
        self.http.delete(&path, &[], "milestones.delete_group")
    }

    pub fn list_group_milestone_issues(&self, group_id: u64, milestone_id: u64) -> Result<Vec<Issue>, GitLabError> {
        let path = format!("groups/{}/milestones/{}/issues", group_id, milestone_id);
        self.http.get(&path, &[], "milestones.list_group_issues")
    }
}
