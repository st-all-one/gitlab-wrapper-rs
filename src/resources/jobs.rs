use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use crate::utils::encoding::filter_to_query;
use std::sync::Arc;

/// Recurso de API para operações com jobs no GitLab.
#[derive(Debug)]
pub struct JobsResource {
    http: Arc<HttpClient>,
}

impl JobsResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista jobs de um projeto com filtros opcionais.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `filter`: Filtros opcionais para a consulta.
    ///
    /// ## Returns
    /// `Result<Vec<Job>, GitLabError>` — lista de jobs.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list(
        &self,
        project_id: u64,
        filter: Option<&JobFilter>,
    ) -> Result<Vec<Job>, GitLabError> {
        let path = format!("projects/{}/jobs", project_id);
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "jobs.list").await
    }

    /// Lista jobs de uma pipeline específica.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `pipeline_id`: ID da pipeline no GitLab.
    /// - `filter`: Filtros opcionais para a consulta.
    ///
    /// ## Returns
    /// `Result<Vec<Job>, GitLabError>` — lista de jobs da pipeline.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list_by_pipeline(
        &self,
        project_id: u64,
        pipeline_id: u64,
        filter: Option<&JobFilter>,
    ) -> Result<Vec<Job>, GitLabError> {
        let path = format!("projects/{}/pipelines/{}/jobs", project_id, pipeline_id);
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "jobs.list_by_pipeline").await
    }

    /// Obtém um job pelo ID.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `job_id`: ID do job no GitLab.
    ///
    /// ## Returns
    /// `Result<Job, GitLabError>` — dados do job solicitado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get(&self, project_id: u64, job_id: u64) -> Result<Job, GitLabError> {
        let path = format!("projects/{}/jobs/{}", project_id, job_id);
        self.http.get(&path, &[], "jobs.get").await
    }

    /// Obtém o log (trace) de um job.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `job_id`: ID do job no GitLab.
    ///
    /// ## Returns
    /// `Result<String, GitLabError>` — conteúdo do log do job.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn trace(&self, project_id: u64, job_id: u64) -> Result<String, GitLabError> {
        let path = format!("projects/{}/jobs/{}/trace", project_id, job_id);
        self.http.get_raw_text(&path, &[], "jobs.trace").await
    }

    /// Cancela um job em execução.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `job_id`: ID do job no GitLab.
    ///
    /// ## Returns
    /// `Result<Job, GitLabError>` — dados do job cancelado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn cancel(&self, project_id: u64, job_id: u64) -> Result<Job, GitLabError> {
        let path = format!("projects/{}/jobs/{}/cancel", project_id, job_id);
        self.http.post(&path, &serde_json::json!({}), "jobs.cancel").await
    }

    /// Tenta novamente um job com falha.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `job_id`: ID do job no GitLab.
    ///
    /// ## Returns
    /// `Result<Job, GitLabError>` — dados do job re-executado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn retry(&self, project_id: u64, job_id: u64) -> Result<Job, GitLabError> {
        let path = format!("projects/{}/jobs/{}/retry", project_id, job_id);
        self.http.post(&path, &serde_json::json!({}), "jobs.retry").await
    }

    /// Executa um job manual (ação "play").
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `job_id`: ID do job no GitLab.
    ///
    /// ## Returns
    /// `Result<Job, GitLabError>` — dados do job executado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn play(&self, project_id: u64, job_id: u64) -> Result<Job, GitLabError> {
        let path = format!("projects/{}/jobs/{}/play", project_id, job_id);
        self.http.post(&path, &serde_json::json!({}), "jobs.play").await
    }

    /// Apaga o log (trace) de um job.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `job_id`: ID do job no GitLab.
    ///
    /// ## Returns
    /// `Result<Job, GitLabError>` — dados do job com log apagado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn erase(&self, project_id: u64, job_id: u64) -> Result<Job, GitLabError> {
        let path = format!("projects/{}/jobs/{}/erase", project_id, job_id);
        self.http.post(&path, &serde_json::json!({}), "jobs.erase").await
    }

    /// Obtém os artefatos de um job.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `job_id`: ID do job no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<u8>, GitLabError>` — conteúdo binário dos artefatos.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn artifacts(&self, project_id: u64, job_id: u64) -> Result<Vec<u8>, GitLabError> {
        let path = format!("projects/{}/jobs/{}/artifacts", project_id, job_id);
        self.http.get_raw(&path, &[], "jobs.artifacts").await
    }
}
