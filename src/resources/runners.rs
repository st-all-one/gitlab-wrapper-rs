use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use std::sync::Arc;

/// Recurso de API para operações com runners no GitLab.
#[derive(Debug)]
pub struct RunnersResource {
    http: Arc<HttpClient>,
}

impl RunnersResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista todos os runners disponíveis.
    ///
    /// ## Params
    /// Nenhum.
    ///
    /// ## Returns
    /// `Result<Vec<Runner>, GitLabError>` — lista de runners.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list(&self) -> Result<Vec<Runner>, GitLabError> {
        self.http.get("runners", &[], "runners.list").await
    }

    /// Obtém um runner pelo ID.
    ///
    /// ## Params
    /// - `runner_id`: ID do runner no GitLab.
    ///
    /// ## Returns
    /// `Result<Runner, GitLabError>` — dados do runner solicitado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get(&self, runner_id: u64) -> Result<Runner, GitLabError> {
        let path = format!("runners/{}", runner_id);
        self.http.get(&path, &[], "runners.get").await
    }

    /// Registra um novo runner.
    ///
    /// ## Params
    /// - `payload`: Dados para criar o runner.
    ///
    /// ## Returns
    /// `Result<Runner, GitLabError>` — dados do runner criado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn create(&self, payload: &CreateRunnerPayload) -> Result<Runner, GitLabError> {
        self.http.post("runners", &payload, "runners.create").await
    }

    /// Atualiza um runner existente.
    ///
    /// ## Params
    /// - `runner_id`: ID do runner no GitLab.
    /// - `payload`: Dados para atualizar o runner.
    ///
    /// ## Returns
    /// `Result<Runner, GitLabError>` — dados do runner atualizado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn update(
        &self,
        runner_id: u64,
        payload: &UpdateRunnerPayload,
    ) -> Result<Runner, GitLabError> {
        let path = format!("runners/{}", runner_id);
        self.http.put(&path, &payload, "runners.update").await
    }

    /// Remove um runner.
    ///
    /// ## Params
    /// - `runner_id`: ID do runner no GitLab.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn delete(&self, runner_id: u64) -> Result<(), GitLabError> {
        let path = format!("runners/{}", runner_id);
        self.http.delete(&path, &[], "runners.delete").await
    }

    /// Lista os jobs executados por um runner.
    ///
    /// ## Params
    /// - `runner_id`: ID do runner no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<Job>, GitLabError>` — lista de jobs do runner.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list_jobs(&self, runner_id: u64) -> Result<Vec<Job>, GitLabError> {
        let path = format!("runners/{}/jobs", runner_id);
        self.http.get(&path, &[], "runners.list_jobs").await
    }
}
