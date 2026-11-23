use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use std::sync::Arc;

/// Recurso de API para external status checks.
#[derive(Debug)]
pub struct ExternalStatusChecksResource {
    http: Arc<HttpClient>,
}

impl ExternalStatusChecksResource {
    pub(crate) const fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Executa a operação .
    /// Executa a operacao `list`.
    ///
    /// ## Errors
    /// Retorna `GitLabError::Api` se o projeto não existir ou `GitLabError::Http`
    /// em erro de transporte da listagem.
    pub async fn list(&self, project_id: u64) -> Result<Vec<serde_json::Value>, GitLabError> {
        let path = format!("projects/{project_id}/external_status_checks");
        self.http
            .get(&path, &[], "external_status_checks.list")
            .await
    }

    /// Executa a operação .
    /// Executa a operacao `create`.
    ///
    /// ## Errors
    /// Retorna `GitLabError::Api` se o payload for inválido ou `GitLabError::Http`
    /// em erro de transporte da criação.
    pub async fn create(
        &self,
        project_id: u64,
        payload: &serde_json::Value,
    ) -> Result<serde_json::Value, GitLabError> {
        let path = format!("projects/{project_id}/external_status_checks");
        self.http
            .post(&path, payload, "external_status_checks.create")
            .await
    }

    /// Executa a operação .
    /// Executa a operacao `delete`.
    ///
    /// ## Errors
    /// Retorna `GitLabError::Api` se a verificação não existir ou `GitLabError::Http`
    /// em erro de transporte da exclusão.
    pub async fn delete(&self, project_id: u64, check_id: u64) -> Result<(), GitLabError> {
        let path = format!("projects/{project_id}/external_status_checks/{check_id}");
        self.http
            .delete(&path, &[], "external_status_checks.delete")
            .await
    }
}
