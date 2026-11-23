use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::{CreateInvitationPayload, Invitation, InvitationFilter};
use crate::utils::encoding::{encode_query_param, filter_to_query};
use std::sync::Arc;

/// Recurso de API para convites (project + group).
#[derive(Debug)]
pub struct InvitationsResource {
    http: Arc<HttpClient>,
}

impl InvitationsResource {
    pub(crate) const fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Executa a operação .
    /// Executa a operacao `list_project`.
    ///
    /// ## Errors
    /// Retorna `GitLabError::Api` se o filtro for inválido ou o projeto não existir,
    /// ou `GitLabError::Http` em erro de transporte.
    pub async fn list_project(
        &self,
        project_id: u64,
        filter: Option<&InvitationFilter>,
    ) -> Result<Vec<Invitation>, GitLabError> {
        let path = format!("projects/{project_id}/invitations");
        let query = filter_to_query(filter);
        self.http
            .get(&path, &query, "invitations.list_project")
            .await
    }
    /// Executa a operação .
    /// Executa a operacao `create_project`.
    ///
    /// ## Errors
    /// Retorna `GitLabError::Api` se o convite for rejeitado ou `GitLabError::Http`
    /// em erro de transporte da criação.
    pub async fn create_project(
        &self,
        project_id: u64,
        payload: &CreateInvitationPayload,
    ) -> Result<Invitation, GitLabError> {
        let path = format!("projects/{project_id}/invitations");
        self.http
            .post(&path, payload, "invitations.create_project")
            .await
    }
    /// Executa a operação .
    /// Executa a operacao `delete_project`.
    ///
    /// ## Errors
    /// Retorna `GitLabError::Api` se o convite não existir ou `GitLabError::Http`
    /// em erro de transporte da exclusão.
    pub async fn delete_project(&self, project_id: u64, email: &str) -> Result<(), GitLabError> {
        let path = format!(
            "projects/{}/invitations/{}",
            project_id,
            encode_query_param(email)
        );
        self.http
            .delete(&path, &[], "invitations.delete_project")
            .await
    }

    /// Executa a operação .
    /// Executa a operacao `list_group`.
    ///
    /// ## Errors
    /// Retorna `GitLabError::Api` se o filtro for inválido ou o grupo não existir,
    /// ou `GitLabError::Http` em erro de transporte.
    pub async fn list_group(
        &self,
        group_id: u64,
        filter: Option<&InvitationFilter>,
    ) -> Result<Vec<Invitation>, GitLabError> {
        let path = format!("groups/{group_id}/invitations");
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "invitations.list_group").await
    }
    /// Executa a operação .
    /// Executa a operacao `create_group`.
    ///
    /// ## Errors
    /// Retorna `GitLabError::Api` se o convite for rejeitado ou `GitLabError::Http`
    /// em erro de transporte da criação.
    pub async fn create_group(
        &self,
        group_id: u64,
        payload: &CreateInvitationPayload,
    ) -> Result<Invitation, GitLabError> {
        let path = format!("groups/{group_id}/invitations");
        self.http
            .post(&path, payload, "invitations.create_group")
            .await
    }
    /// Executa a operação .
    /// Executa a operacao `delete_group`.
    ///
    /// ## Errors
    /// Retorna `GitLabError::Api` se o convite não existir ou `GitLabError::Http`
    /// em erro de transporte da exclusão.
    pub async fn delete_group(&self, group_id: u64, email: &str) -> Result<(), GitLabError> {
        let path = format!(
            "groups/{}/invitations/{}",
            group_id,
            encode_query_param(email)
        );
        self.http
            .delete(&path, &[], "invitations.delete_group")
            .await
    }
}
