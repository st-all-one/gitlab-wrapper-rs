use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use crate::utils::encoding::{encode_query_param, filter_to_query};
use std::sync::Arc;

/// Recurso de API para convites (project + group).
#[derive(Debug)]
pub struct InvitationsResource {
    http: Arc<HttpClient>,
}

impl InvitationsResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Executa a operação .
    /// Executa a operacao `list_project`.
    pub async fn list_project(
        &self,
        project_id: u64,
        filter: Option<&InvitationFilter>,
    ) -> Result<Vec<Invitation>, GitLabError> {
        let path = format!("projects/{}/invitations", project_id);
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "invitations.list_project").await
    }
    /// Executa a operação .
    /// Executa a operacao `create_project`.
    pub async fn create_project(
        &self,
        project_id: u64,
        payload: &CreateInvitationPayload,
    ) -> Result<Invitation, GitLabError> {
        let path = format!("projects/{}/invitations", project_id);
        self.http.post(&path, payload, "invitations.create_project").await
    }
    /// Executa a operação .
    /// Executa a operacao `delete_project`.
    pub async fn delete_project(&self, project_id: u64, email: &str) -> Result<(), GitLabError> {
        let path = format!("projects/{}/invitations/{}", project_id, encode_query_param(email));
        self.http.delete(&path, &[], "invitations.delete_project").await
    }

    /// Executa a operação .
    /// Executa a operacao `list_group`.
    pub async fn list_group(
        &self,
        group_id: u64,
        filter: Option<&InvitationFilter>,
    ) -> Result<Vec<Invitation>, GitLabError> {
        let path = format!("groups/{}/invitations", group_id);
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "invitations.list_group").await
    }
    /// Executa a operação .
    /// Executa a operacao `create_group`.
    pub async fn create_group(
        &self,
        group_id: u64,
        payload: &CreateInvitationPayload,
    ) -> Result<Invitation, GitLabError> {
        let path = format!("groups/{}/invitations", group_id);
        self.http.post(&path, payload, "invitations.create_group").await
    }
    /// Executa a operação .
    /// Executa a operacao `delete_group`.
    pub async fn delete_group(&self, group_id: u64, email: &str) -> Result<(), GitLabError> {
        let path = format!("groups/{}/invitations/{}", group_id, encode_query_param(email));
        self.http.delete(&path, &[], "invitations.delete_group").await
    }
}
