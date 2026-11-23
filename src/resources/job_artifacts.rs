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
    pub(crate) const fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Executa a operação .
    /// Executa a operacao `download_by_ref`.
    ///
    /// ## Errors
    /// Retorna `GitLabError::Api` se o job não existir ou não tiver artefatos,
    /// ou `GitLabError::Http` em erro de transporte do download.
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
        self.http
            .get(&path, &query, "job_artifacts.download_by_ref")
            .await
    }

    /// Executa a operação .
    /// Executa a operacao `keep`.
    ///
    /// ## Errors
    /// Retorna `GitLabError::Api` se o artefato não puder ser mantido ou
    /// `GitLabError::Http` em erro de transporte da requisição.
    pub async fn keep(
        &self,
        project_id: u64,
        job_id: u64,
    ) -> Result<serde_json::Value, GitLabError> {
        let path = format!("projects/{project_id}/jobs/{job_id}/artifacts/keep");
        self.http
            .post(&path, &serde_json::json!({}), "job_artifacts.keep")
            .await
    }

    /// Executa a operação .
    /// Executa a operacao `delete`.
    ///
    /// ## Errors
    /// Retorna `GitLabError::Api` se o artefato não existir ou `GitLabError::Http`
    /// em erro de transporte da exclusão.
    pub async fn delete(&self, project_id: u64, job_id: u64) -> Result<(), GitLabError> {
        let path = format!("projects/{project_id}/jobs/{job_id}/artifacts");
        self.http.delete(&path, &[], "job_artifacts.delete").await
    }

    /// Executa a operação .
    /// Executa a operacao `delete_all`.
    ///
    /// ## Errors
    /// Retorna `GitLabError::Api` se a exclusão for rejeitada ou `GitLabError::Http`
    /// em erro de transporte da requisição.
    pub async fn delete_all(&self, project_id: u64) -> Result<(), GitLabError> {
        let path = format!("projects/{project_id}/artifacts");
        self.http
            .delete(&path, &[], "job_artifacts.delete_all")
            .await
    }
}
