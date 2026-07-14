use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::utils::encoding::encode_query_param;
use std::sync::Arc;

/// Recurso de API para artefatos de jobs no GitLab.
#[derive(Debug)]
pub struct JobArtifactsResource {
    http: Arc<HttpClient>,
}

impl JobArtifactsResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Executa a operação .
    /// Executa a operacao `download_by_ref`.
    pub async fn download_by_ref(
        &self,
        project_id: u64,
        ref_name: &str,
        job_name: &str,
    ) -> Result<serde_json::Value, GitLabError> {
        let path = format!(
            "projects/{}/jobs/artifacts/{}/download",
            project_id,
            encode_query_param(ref_name)
        );
        let query = vec![("job".to_string(), job_name.to_string())];
        self.http.get(&path, &query, "job_artifacts.download_by_ref").await
    }

    /// Executa a operação .
    /// Executa a operacao `keep`.
    pub async fn keep(
        &self,
        project_id: u64,
        job_id: u64,
    ) -> Result<serde_json::Value, GitLabError> {
        let path = format!("projects/{}/jobs/{}/artifacts/keep", project_id, job_id);
        self.http.post(&path, &serde_json::json!({}), "job_artifacts.keep").await
    }

    /// Executa a operação .
    /// Executa a operacao `delete`.
    pub async fn delete(&self, project_id: u64, job_id: u64) -> Result<(), GitLabError> {
        let path = format!("projects/{}/jobs/{}/artifacts", project_id, job_id);
        self.http.delete(&path, &[], "job_artifacts.delete").await
    }

    /// Executa a operação .
    /// Executa a operacao `delete_all`.
    pub async fn delete_all(&self, project_id: u64) -> Result<(), GitLabError> {
        let path = format!("projects/{}/artifacts", project_id);
        self.http.delete(&path, &[], "job_artifacts.delete_all").await
    }
}
