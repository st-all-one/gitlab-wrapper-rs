use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use crate::utils::encoding::filter_to_query;
use std::sync::Arc;

/// Recurso de API para epics no GitLab (Ultimate).
#[derive(Debug)]
pub struct EpicsResource {
    http: Arc<HttpClient>,
}

impl EpicsResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Executa a operação .
    /// Executa a operacao `list`.
    pub async fn list(
        &self,
        group_id: u64,
        filter: Option<&EpicFilter>,
    ) -> Result<Vec<Epic>, GitLabError> {
        let path = format!("groups/{}/epics", group_id);
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "epics.list").await
    }

    /// Executa a operação .
    /// Executa a operacao `get`.
    pub async fn get(&self, group_id: u64, epic_iid: u32) -> Result<Epic, GitLabError> {
        let path = format!("groups/{}/epics/{}", group_id, epic_iid);
        self.http.get(&path, &[], "epics.get").await
    }

    /// Executa a operação .
    /// Executa a operacao `create`.
    pub async fn create(
        &self,
        group_id: u64,
        payload: &CreateEpicPayload,
    ) -> Result<Epic, GitLabError> {
        let path = format!("groups/{}/epics", group_id);
        self.http.post(&path, payload, "epics.create").await
    }

    /// Executa a operação .
    /// Executa a operacao `update`.
    pub async fn update(
        &self,
        group_id: u64,
        epic_iid: u32,
        payload: &UpdateEpicPayload,
    ) -> Result<Epic, GitLabError> {
        let path = format!("groups/{}/epics/{}", group_id, epic_iid);
        self.http.put(&path, payload, "epics.update").await
    }

    /// Executa a operação .
    /// Executa a operacao `delete`.
    pub async fn delete(&self, group_id: u64, epic_iid: u32) -> Result<(), GitLabError> {
        let path = format!("groups/{}/epics/{}", group_id, epic_iid);
        self.http.delete(&path, &[], "epics.delete").await
    }

    // ── Epic Issues ──

    /// Executa a operação .
    /// Executa a operacao `assign_issue`.
    pub async fn assign_issue(
        &self,
        group_id: u64,
        epic_iid: u32,
        issue_id: u64,
    ) -> Result<serde_json::Value, GitLabError> {
        let path = format!("groups/{}/epics/{}/issues/{}", group_id, epic_iid, issue_id);
        self.http.post(&path, &serde_json::json!({}), "epics.assign_issue").await
    }

    /// Executa a operação .
    /// Executa a operacao `unassign_issue`.
    pub async fn unassign_issue(
        &self,
        group_id: u64,
        epic_iid: u32,
        issue_id: u64,
    ) -> Result<(), GitLabError> {
        let path = format!("groups/{}/epics/{}/issues/{}", group_id, epic_iid, issue_id);
        self.http.delete(&path, &[], "epics.unassign_issue").await
    }

    // ── Epic Links ──

    /// Executa a operação .
    /// Executa a operacao `add_child_epic`.
    pub async fn add_child_epic(
        &self,
        group_id: u64,
        epic_iid: u32,
        payload: &CreateEpicLinkPayload,
    ) -> Result<EpicLink, GitLabError> {
        let path = format!("groups/{}/epics/{}/epics", group_id, epic_iid);
        self.http.post(&path, payload, "epics.add_child").await
    }

    /// Executa a operação .
    /// Executa a operacao `remove_child_epic`.
    pub async fn remove_child_epic(
        &self,
        group_id: u64,
        epic_iid: u32,
        child_id: u64,
    ) -> Result<(), GitLabError> {
        let path = format!("groups/{}/epics/{}/epics/{}", group_id, epic_iid, child_id);
        self.http.delete(&path, &[], "epics.remove_child").await
    }

    // ── Linked Epics ──

    /// Executa a operação .
    /// Executa a operacao `list_related_epics`.
    pub async fn list_related_epics(
        &self,
        group_id: u64,
        epic_iid: u32,
    ) -> Result<Vec<EpicLink>, GitLabError> {
        let path = format!("groups/{}/epics/{}/related_epics", group_id, epic_iid);
        self.http.get(&path, &[], "epics.list_related").await
    }

    /// Executa a operação .
    /// Executa a operacao `create_related_epic`.
    pub async fn create_related_epic(
        &self,
        group_id: u64,
        epic_iid: u32,
        payload: &CreateEpicLinkPayload,
    ) -> Result<EpicLink, GitLabError> {
        let path = format!("groups/{}/epics/{}/related_epics", group_id, epic_iid);
        self.http.post(&path, payload, "epics.create_related").await
    }

    /// Executa a operação .
    /// Executa a operacao `delete_related_epic`.
    pub async fn delete_related_epic(
        &self,
        group_id: u64,
        epic_iid: u32,
        link_id: u64,
    ) -> Result<(), GitLabError> {
        let path = format!("groups/{}/epics/{}/related_epics/{}", group_id, epic_iid, link_id);
        self.http.delete(&path, &[], "epics.delete_related").await
    }
}
