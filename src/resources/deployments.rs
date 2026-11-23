use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::{Deployment, DeploymentFilter};
use crate::utils::encoding::filter_to_query;
use std::sync::Arc;

/// Recurso de API para deployments no GitLab.
#[derive(Debug)]
pub struct DeploymentsResource {
    http: Arc<HttpClient>,
}

impl DeploymentsResource {
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
        filter: Option<&DeploymentFilter>,
    ) -> Result<Vec<Deployment>, GitLabError> {
        let path = format!("projects/{project_id}/deployments");
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "deployments.list").await
    }

    /// Executa a operação .
    /// Executa a operacao `get`.
    ///
    /// ## Errors
    /// Retorna `GitLabError::Api` se o deployment não existir ou `GitLabError::Http`
    /// em erro de transporte da requisição.
    pub async fn get(
        &self,
        project_id: u64,
        deployment_id: u64,
    ) -> Result<Deployment, GitLabError> {
        let path = format!("projects/{project_id}/deployments/{deployment_id}");
        self.http.get(&path, &[], "deployments.get").await
    }

    /// Executa a operação .
    /// Executa a operacao `approve`.
    ///
    /// ## Errors
    /// Retorna `GitLabError::Api` se a aprovação for rejeitada ou o deployment não existir,
    /// ou `GitLabError::Http` em erro de transporte.
    pub async fn approve(
        &self,
        project_id: u64,
        deployment_id: u64,
    ) -> Result<Deployment, GitLabError> {
        let path = format!("projects/{project_id}/deployments/{deployment_id}/approve");
        self.http
            .post(&path, &serde_json::json!({}), "deployments.approve")
            .await
    }
}
