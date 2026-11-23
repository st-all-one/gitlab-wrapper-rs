use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::{ProtectEnvironmentPayload, ProtectedEnvironment, ProtectedEnvironmentFilter};
use crate::utils::encoding::{encode_query_param, filter_to_query};
use std::sync::Arc;

/// Recurso de API para ambientes protegidos no GitLab.
#[derive(Debug)]
pub struct ProtectedEnvironmentsResource {
    http: Arc<HttpClient>,
}

impl ProtectedEnvironmentsResource {
    pub(crate) const fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Executa a operação .
    /// Executa a operacao `list`.
    ///
    /// ## Errors
    /// Retorna `GitLabError::Api` se o filtro for inválido ou o projeto não existir,
    /// ou `GitLabError::Http` em erro de transporte.
    pub async fn list(
        &self,
        project_id: u64,
        filter: Option<&ProtectedEnvironmentFilter>,
    ) -> Result<Vec<ProtectedEnvironment>, GitLabError> {
        let path = format!("projects/{project_id}/protected_environments");
        let query = filter_to_query(filter);
        self.http
            .get(&path, &query, "protected_environments.list")
            .await
    }

    /// Executa a operação .
    /// Executa a operacao `get`.
    ///
    /// ## Errors
    /// Retorna `GitLabError::Api` se o ambiente não existir ou `GitLabError::Http`
    /// em erro de transporte da requisição.
    pub async fn get(
        &self,
        project_id: u64,
        name: &str,
    ) -> Result<ProtectedEnvironment, GitLabError> {
        let path = format!(
            "projects/{}/protected_environments/{}",
            project_id,
            encode_query_param(name)
        );
        self.http
            .get(&path, &[], "protected_environments.get")
            .await
    }

    /// Executa a operação .
    /// Executa a operacao `protect`.
    ///
    /// ## Errors
    /// Retorna `GitLabError::Api` se o payload for inválido ou `GitLabError::Http`
    /// em erro de transporte da proteção.
    pub async fn protect(
        &self,
        project_id: u64,
        payload: &ProtectEnvironmentPayload,
    ) -> Result<ProtectedEnvironment, GitLabError> {
        let path = format!("projects/{project_id}/protected_environments");
        self.http
            .post(&path, payload, "protected_environments.protect")
            .await
    }

    /// Executa a operação .
    /// Executa a operacao `unprotect`.
    ///
    /// ## Errors
    /// Retorna `GitLabError::Api` se o ambiente não existir ou `GitLabError::Http`
    /// em erro de transporte da exclusão.
    pub async fn unprotect(&self, project_id: u64, name: &str) -> Result<(), GitLabError> {
        let path = format!(
            "projects/{}/protected_environments/{}",
            project_id,
            encode_query_param(name)
        );
        self.http
            .delete(&path, &[], "protected_environments.unprotect")
            .await
    }
}
