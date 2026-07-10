use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use std::sync::Arc;
use crate::types::*;

#[derive(Debug)]
pub struct NotesResource {
    http: Arc<HttpClient>,
}

impl NotesResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    fn base_issue(project_id: u64, issue_iid: u32) -> String {
        format!("projects/{}/issues/{}/notes", project_id, issue_iid)
    }

    fn base_mr(project_id: u64, mr_iid: u32) -> String {
        format!("projects/{}/merge_requests/{}/notes", project_id, mr_iid)
    }

    fn base_commit(project_id: u64, sha: &str) -> String {
        format!("projects/{}/repository/commits/{}/notes", project_id, sha)
    }

    fn base_snippet(project_id: u64, snippet_id: u64) -> String {
        format!("projects/{}/snippets/{}/notes", project_id, snippet_id)
    }

    pub fn list_issue_notes(&self, project_id: u64, issue_iid: u32) -> Result<Vec<Note>, GitLabError> {
        let path = Self::base_issue(project_id, issue_iid);
        self.http.get(&path, &[], "notes.list_issue")
    }

    pub fn create_issue_note(&self, project_id: u64, issue_iid: u32, payload: &CreateNotePayload) -> Result<Note, GitLabError> {
        let path = Self::base_issue(project_id, issue_iid);
        self.http.post(&path, &payload, "notes.create_issue")
    }

    pub fn update_issue_note(&self, project_id: u64, issue_iid: u32, note_id: u64, payload: &UpdateNotePayload) -> Result<Note, GitLabError> {
        let path = format!("{}/{}", Self::base_issue(project_id, issue_iid), note_id);
        self.http.put(&path, &payload, "notes.update_issue")
    }

    pub fn delete_issue_note(&self, project_id: u64, issue_iid: u32, note_id: u64) -> Result<(), GitLabError> {
        let path = format!("{}/{}", Self::base_issue(project_id, issue_iid), note_id);
        self.http.delete(&path, &[], "notes.delete_issue")
    }

    pub fn list_mr_notes(&self, project_id: u64, mr_iid: u32) -> Result<Vec<Note>, GitLabError> {
        let path = Self::base_mr(project_id, mr_iid);
        self.http.get(&path, &[], "notes.list_mr")
    }

    pub fn create_mr_note(&self, project_id: u64, mr_iid: u32, payload: &CreateNotePayload) -> Result<Note, GitLabError> {
        let path = Self::base_mr(project_id, mr_iid);
        self.http.post(&path, &payload, "notes.create_mr")
    }

    pub fn update_mr_note(&self, project_id: u64, mr_iid: u32, note_id: u64, payload: &UpdateNotePayload) -> Result<Note, GitLabError> {
        let path = format!("{}/{}", Self::base_mr(project_id, mr_iid), note_id);
        self.http.put(&path, &payload, "notes.update_mr")
    }

    pub fn list_commit_notes(&self, project_id: u64, sha: &str) -> Result<Vec<Note>, GitLabError> {
        let path = Self::base_commit(project_id, sha);
        self.http.get(&path, &[], "notes.list_commit")
    }

    pub fn create_commit_note(&self, project_id: u64, sha: &str, payload: &CreateNotePayload) -> Result<Note, GitLabError> {
        let path = Self::base_commit(project_id, sha);
        self.http.post(&path, &payload, "notes.create_commit")
    }

    pub fn list_snippet_notes(&self, project_id: u64, snippet_id: u64) -> Result<Vec<Note>, GitLabError> {
        let path = Self::base_snippet(project_id, snippet_id);
        self.http.get(&path, &[], "notes.list_snippet")
    }

    pub fn create_snippet_note(&self, project_id: u64, snippet_id: u64, payload: &CreateNotePayload) -> Result<Note, GitLabError> {
        let path = Self::base_snippet(project_id, snippet_id);
        self.http.post(&path, &payload, "notes.create_snippet")
    }
}
