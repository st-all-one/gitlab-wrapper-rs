use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use crate::utils::encoding::filter_to_query;
use std::sync::Arc;

/// Recurso de API para operações com pipelines no GitLab.
#[derive(Debug)]
pub struct PipelinesResource {
    http: Arc<HttpClient>,
}

impl PipelinesResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista pipelines de um projeto com filtros opcionais.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `filter`: Filtros opcionais para a consulta.
    ///
    /// ## Returns
    /// `Result<Vec<Pipeline>, GitLabError>` — lista de pipelines.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list(
        &self,
        project_id: u64,
        filter: Option<&PipelineFilter>,
    ) -> Result<Vec<Pipeline>, GitLabError> {
        let path = format!("projects/{}/pipelines", project_id);
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "pipelines.list").await
    }

    /// Obtém uma pipeline pelo ID.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `pipeline_id`: ID da pipeline no GitLab.
    ///
    /// ## Returns
    /// `Result<Pipeline, GitLabError>` — dados da pipeline solicitada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get(&self, project_id: u64, pipeline_id: u64) -> Result<Pipeline, GitLabError> {
        let path = format!("projects/{}/pipelines/{}", project_id, pipeline_id);
        self.http.get(&path, &[], "pipelines.get").await
    }

    /// Obtém a pipeline mais recente de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    ///
    /// ## Returns
    /// `Result<Pipeline, GitLabError>` — dados da pipeline mais recente.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get_latest(&self, project_id: u64) -> Result<Pipeline, GitLabError> {
        let path = format!("projects/{}/pipelines/latest", project_id);
        self.http.get(&path, &[], "pipelines.get_latest").await
    }

    /// Cria uma nova pipeline em um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `payload`: Dados para criar a pipeline.
    ///
    /// ## Returns
    /// `Result<Pipeline, GitLabError>` — dados da pipeline criada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn create(
        &self,
        project_id: u64,
        payload: &CreatePipelinePayload,
    ) -> Result<Pipeline, GitLabError> {
        let path = format!("projects/{}/pipeline", project_id);
        self.http.post(&path, &payload, "pipelines.create").await
    }

    /// Tenta novamente uma pipeline com falha.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `pipeline_id`: ID da pipeline no GitLab.
    ///
    /// ## Returns
    /// `Result<Pipeline, GitLabError>` — dados da pipeline reiniciada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn retry(&self, project_id: u64, pipeline_id: u64) -> Result<Pipeline, GitLabError> {
        let path = format!("projects/{}/pipelines/{}/retry", project_id, pipeline_id);
        self.http.post(&path, &serde_json::json!({}), "pipelines.retry").await
    }

    /// Cancela uma pipeline em execução.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `pipeline_id`: ID da pipeline no GitLab.
    ///
    /// ## Returns
    /// `Result<Pipeline, GitLabError>` — dados da pipeline cancelada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn cancel(&self, project_id: u64, pipeline_id: u64) -> Result<Pipeline, GitLabError> {
        let path = format!("projects/{}/pipelines/{}/cancel", project_id, pipeline_id);
        self.http.post(&path, &serde_json::json!({}), "pipelines.cancel").await
    }

    /// Remove uma pipeline.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `pipeline_id`: ID da pipeline no GitLab.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn delete(&self, project_id: u64, pipeline_id: u64) -> Result<(), GitLabError> {
        let path = format!("projects/{}/pipelines/{}", project_id, pipeline_id);
        self.http.delete(&path, &[], "pipelines.delete").await
    }

    /// Lista as variáveis de uma pipeline.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `pipeline_id`: ID da pipeline no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<PipelineVariable>, GitLabError>` — lista de variáveis da pipeline.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn variables(
        &self,
        project_id: u64,
        pipeline_id: u64,
    ) -> Result<Vec<PipelineVariable>, GitLabError> {
        let path = format!("projects/{}/pipelines/{}/variables", project_id, pipeline_id);
        self.http.get(&path, &[], "pipelines.variables").await
    }

    /// Obtém o relatório de testes de uma pipeline.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `pipeline_id`: ID da pipeline no GitLab.
    ///
    /// ## Returns
    /// `Result<serde_json::Value, GitLabError>` — dados do relatório de testes.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn test_report(
        &self,
        project_id: u64,
        pipeline_id: u64,
    ) -> Result<serde_json::Value, GitLabError> {
        let path = format!("projects/{}/pipelines/{}/test_report", project_id, pipeline_id);
        self.http.get(&path, &[], "pipelines.test_report").await
    }

    /// Obtém o sumário do relatório de testes de uma pipeline.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `pipeline_id`: ID da pipeline no GitLab.
    ///
    /// ## Returns
    /// `Result<serde_json::Value, GitLabError>` — dados do sumário do relatório de testes.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn test_report_summary(
        &self,
        project_id: u64,
        pipeline_id: u64,
    ) -> Result<serde_json::Value, GitLabError> {
        let path = format!("projects/{}/pipelines/{}/test_report_summary", project_id, pipeline_id);
        self.http.get(&path, &[], "pipelines.test_report_summary").await
    }
}
