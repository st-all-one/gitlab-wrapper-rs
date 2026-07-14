use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use crate::utils::encoding::filter_to_query;
use std::sync::Arc;

/// Recurso de API para deployments no GitLab.
#[derive(Debug)]
pub struct DeploymentsResource {
    http: Arc<HttpClient>,
}

impl DeploymentsResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Executa a operação .
    /// Executa a operacao `list`.
    pub async fn list(
        &self,
        project_id: u64,
        filter: Option<&DeploymentFilter>,
    ) -> Result<Vec<Deployment>, GitLabError> {
        let path = format!("projects/{}/deployments", project_id);
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "deployments.list").await
    }

    /// Executa a operação .
    /// Executa a operacao `get`.
    pub async fn get(
        &self,
        project_id: u64,
        deployment_id: u64,
    ) -> Result<Deployment, GitLabError> {
        let path = format!("projects/{}/deployments/{}", project_id, deployment_id);
        self.http.get(&path, &[], "deployments.get").await
    }

    /// Executa a operação .
    /// Executa a operacao `approve`.
    pub async fn approve(
        &self,
        project_id: u64,
        deployment_id: u64,
    ) -> Result<Deployment, GitLabError> {
        let path = format!("projects/{}/deployments/{}/approve", project_id, deployment_id);
        self.http.post(&path, &serde_json::json!({}), "deployments.approve").await
    }
}
