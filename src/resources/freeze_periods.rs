use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use std::sync::Arc;

/// Recurso de API para operações com períodos de congelamento (freeze periods) no GitLab.
#[derive(Debug)]
pub struct FreezePeriodsResource {
    http: Arc<HttpClient>,
}

impl FreezePeriodsResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista todos os períodos de congelamento de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<FreezePeriod>, GitLabError>` — lista de períodos de congelamento.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list(&self, project_id: u64) -> Result<Vec<FreezePeriod>, GitLabError> {
        let path = format!("projects/{}/freeze_periods", project_id);
        self.http.get(&path, &[], "freeze_periods.list").await
    }

    /// Obtém um período de congelamento pelo ID.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `freeze_period_id`: ID do período de congelamento.
    ///
    /// ## Returns
    /// `Result<FreezePeriod, GitLabError>` — dados do período de congelamento.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get(
        &self,
        project_id: u64,
        freeze_period_id: u64,
    ) -> Result<FreezePeriod, GitLabError> {
        let path = format!("projects/{}/freeze_periods/{}", project_id, freeze_period_id);
        self.http.get(&path, &[], "freeze_periods.get").await
    }

    /// Cria um novo período de congelamento.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `payload`: Dados do período de congelamento a criar.
    ///
    /// ## Returns
    /// `Result<FreezePeriod, GitLabError>` — dados do período criado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn create(
        &self,
        project_id: u64,
        payload: &CreateFreezePeriodPayload,
    ) -> Result<FreezePeriod, GitLabError> {
        let path = format!("projects/{}/freeze_periods", project_id);
        self.http.post(&path, payload, "freeze_periods.create").await
    }

    /// Atualiza um período de congelamento existente.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `freeze_period_id`: ID do período de congelamento.
    /// - `payload`: Dados do período a atualizar.
    ///
    /// ## Returns
    /// `Result<FreezePeriod, GitLabError>` — dados do período atualizado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn update(
        &self,
        project_id: u64,
        freeze_period_id: u64,
        payload: &UpdateFreezePeriodPayload,
    ) -> Result<FreezePeriod, GitLabError> {
        let path = format!("projects/{}/freeze_periods/{}", project_id, freeze_period_id);
        self.http.put(&path, payload, "freeze_periods.update").await
    }

    /// Remove um período de congelamento.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `freeze_period_id`: ID do período de congelamento.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn delete(&self, project_id: u64, freeze_period_id: u64) -> Result<(), GitLabError> {
        let path = format!("projects/{}/freeze_periods/{}", project_id, freeze_period_id);
        self.http.delete(&path, &[], "freeze_periods.delete").await
    }
}
