use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use crate::utils::encoding::filter_to_query;
use std::sync::Arc;

/// Recurso de API para operações com triggers de pipeline no GitLab.
#[derive(Debug)]
pub struct PipelineTriggersResource {
    http: Arc<HttpClient>,
}

impl PipelineTriggersResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista todos os triggers de pipeline de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `filter`: Filtro opcional (paginação).
    ///
    /// ## Returns
    /// `Result<Vec<PipelineTrigger>, GitLabError>` — lista de triggers.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list(
        &self,
        project_id: u64,
        filter: Option<&PipelineTriggerFilter>,
    ) -> Result<Vec<PipelineTrigger>, GitLabError> {
        let path = format!("projects/{}/triggers", project_id);
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "pipeline_triggers.list").await
    }

    /// Obtém um trigger de pipeline por ID.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `trigger_id`: ID do trigger no GitLab.
    ///
    /// ## Returns
    /// `Result<PipelineTrigger, GitLabError>` — dados do trigger.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get(
        &self,
        project_id: u64,
        trigger_id: u64,
    ) -> Result<PipelineTrigger, GitLabError> {
        let path = format!("projects/{}/triggers/{}", project_id, trigger_id);
        self.http.get(&path, &[], "pipeline_triggers.get").await
    }

    /// Cria um novo trigger de pipeline.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `payload`: Dados do trigger a criar.
    ///
    /// ## Returns
    /// `Result<PipelineTrigger, GitLabError>` — dados do trigger criado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn create(
        &self,
        project_id: u64,
        payload: &CreatePipelineTriggerPayload,
    ) -> Result<PipelineTrigger, GitLabError> {
        let path = format!("projects/{}/triggers", project_id);
        self.http.post(&path, payload, "pipeline_triggers.create").await
    }

    /// Atualiza um trigger de pipeline existente.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `trigger_id`: ID do trigger no GitLab.
    /// - `payload`: Dados do trigger a atualizar.
    ///
    /// ## Returns
    /// `Result<PipelineTrigger, GitLabError>` — dados do trigger atualizado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn update(
        &self,
        project_id: u64,
        trigger_id: u64,
        payload: &UpdatePipelineTriggerPayload,
    ) -> Result<PipelineTrigger, GitLabError> {
        let path = format!("projects/{}/triggers/{}", project_id, trigger_id);
        self.http.put(&path, payload, "pipeline_triggers.update").await
    }

    /// Remove um trigger de pipeline.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `trigger_id`: ID do trigger no GitLab.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn delete(&self, project_id: u64, trigger_id: u64) -> Result<(), GitLabError> {
        let path = format!("projects/{}/triggers/{}", project_id, trigger_id);
        self.http.delete(&path, &[], "pipeline_triggers.delete").await
    }

    /// Assume a posse de um trigger de pipeline.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `trigger_id`: ID do trigger no GitLab.
    ///
    /// ## Returns
    /// `Result<PipelineTrigger, GitLabError>` — dados do trigger com novo dono.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn take_ownership(
        &self,
        project_id: u64,
        trigger_id: u64,
    ) -> Result<PipelineTrigger, GitLabError> {
        let path = format!("projects/{}/triggers/{}/take_ownership", project_id, trigger_id);
        self.http.post(&path, &serde_json::json!({}), "pipeline_triggers.take_ownership").await
    }
}
