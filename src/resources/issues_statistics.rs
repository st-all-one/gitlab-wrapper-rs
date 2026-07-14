use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use std::sync::Arc;

/// Recurso de API para estatísticas de issues.
#[derive(Debug)]
pub struct IssuesStatisticsResource {
    http: Arc<HttpClient>,
}

impl IssuesStatisticsResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Executa a operação .
    /// Executa a operacao `get_global`.
    pub async fn get_global(&self) -> Result<serde_json::Value, GitLabError> {
        self.http.get("issues_statistics", &[], "issues_statistics.get_global").await
    }

    /// Executa a operação .
    /// Executa a operacao `get_project`.
    pub async fn get_project(&self, project_id: u64) -> Result<serde_json::Value, GitLabError> {
        let path = format!("projects/{}/issues_statistics", project_id);
        self.http.get(&path, &[], "issues_statistics.get_project").await
    }

    /// Executa a operação .
    /// Executa a operacao `get_group`.
    pub async fn get_group(&self, group_id: u64) -> Result<serde_json::Value, GitLabError> {
        let path = format!("groups/{}/issues_statistics", group_id);
        self.http.get(&path, &[], "issues_statistics.get_group").await
    }
}
