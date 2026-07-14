use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use std::sync::Arc;

/// Recurso de API para configurações de Error Tracking.
#[derive(Debug)]
pub struct ErrorTrackingResource {
    http: Arc<HttpClient>,
}

impl ErrorTrackingResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Executa a operação .
    /// Executa a operacao `get_settings`.
    pub async fn get_settings(&self, project_id: u64) -> Result<serde_json::Value, GitLabError> {
        let path = format!("projects/{}/error_tracking/settings", project_id);
        self.http.get(&path, &[], "error_tracking.get_settings").await
    }

    /// Executa a operação .
    /// Executa a operacao `update_settings`.
    pub async fn update_settings(
        &self,
        project_id: u64,
        payload: &serde_json::Value,
    ) -> Result<serde_json::Value, GitLabError> {
        let path = format!("projects/{}/error_tracking/settings", project_id);
        self.http.put(&path, payload, "error_tracking.update_settings").await
    }
}
