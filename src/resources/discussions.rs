use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use std::sync::Arc;
use crate::types::*;

#[derive(Debug)]
pub struct DiscussionsResource {
    http: Arc<HttpClient>,
}

impl DiscussionsResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    fn base_issue(project_id: u64, issue_iid: u32) -> String {
        format!("projects/{}/issues/{}/discussions", project_id, issue_iid)
    }

    fn base_mr(project_id: u64, mr_iid: u32) -> String {
        format!("projects/{}/merge_requests/{}/discussions", project_id, mr_iid)
    }

    fn base_commit(project_id: u64, sha: &str) -> String {
        format!("projects/{}/repository/commits/{}/discussions", project_id, sha)
    }

    pub fn list_issue_discussions(&self, project_id: u64, issue_iid: u32) -> Result<Vec<Discussion>, GitLabError> {
        let path = Self::base_issue(project_id, issue_iid);
        self.http.get(&path, &[], "discussions.list_issue")
    }

    pub fn create_issue_discussion(&self, project_id: u64, issue_iid: u32, body: &CreateDiscussionPayload) -> Result<Discussion, GitLabError> {
        let path = Self::base_issue(project_id, issue_iid);
        self.http.post(&path, &body, "discussions.create_issue")
    }

    pub fn add_issue_discussion_note(&self, project_id: u64, issue_iid: u32, discussion_id: &str, note: &CreateNotePayload) -> Result<Note, GitLabError> {
        let path = format!("{}/{}/notes", Self::base_issue(project_id, issue_iid), discussion_id);
        self.http.post(&path, &note, "discussions.add_issue_note")
    }

    pub fn list_mr_discussions(&self, project_id: u64, mr_iid: u32) -> Result<Vec<Discussion>, GitLabError> {
        let path = Self::base_mr(project_id, mr_iid);
        self.http.get(&path, &[], "discussions.list_mr")
    }

    pub fn create_mr_discussion(&self, project_id: u64, mr_iid: u32, body: &CreateDiscussionPayload) -> Result<Discussion, GitLabError> {
        let path = Self::base_mr(project_id, mr_iid);
        self.http.post(&path, &body, "discussions.create_mr")
    }

    pub fn add_mr_discussion_note(&self, project_id: u64, mr_iid: u32, discussion_id: &str, note: &CreateNotePayload) -> Result<Note, GitLabError> {
        let path = format!("{}/{}/notes", Self::base_mr(project_id, mr_iid), discussion_id);
        self.http.post(&path, &note, "discussions.add_mr_note")
    }

    pub fn list_commit_discussions(&self, project_id: u64, sha: &str) -> Result<Vec<Discussion>, GitLabError> {
        let path = Self::base_commit(project_id, sha);
        self.http.get(&path, &[], "discussions.list_commit")
    }

    pub fn create_commit_discussion(&self, project_id: u64, sha: &str, body: &CreateDiscussionPayload) -> Result<Discussion, GitLabError> {
        let path = Self::base_commit(project_id, sha);
        self.http.post(&path, &body, "discussions.create_commit")
    }

    pub fn add_commit_discussion_note(&self, project_id: u64, sha: &str, discussion_id: &str, note: &CreateNotePayload) -> Result<Note, GitLabError> {
        let path = format!("{}/{}/notes", Self::base_commit(project_id, sha), discussion_id);
        self.http.post(&path, &note, "discussions.add_commit_note")
    }
}
