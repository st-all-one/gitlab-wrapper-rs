use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use crate::utils::encoding::filter_to_query;
use std::sync::Arc;

/// Recurso de API para operações com webhooks de projeto no GitLab.
#[derive(Debug)]
pub struct ProjectHooksResource {
    http: Arc<HttpClient>,
}

impl ProjectHooksResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista todos os webhooks de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `filter`: Filtro opcional (paginação).
    ///
    /// ## Returns
    /// `Result<Vec<Hook>, GitLabError>` — lista de webhooks.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list(
        &self,
        project_id: u64,
        filter: Option<&HookFilter>,
    ) -> Result<Vec<Hook>, GitLabError> {
        let path = format!("projects/{}/hooks", project_id);
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "project_hooks.list").await
    }

    /// Obtém um webhook de projeto por ID.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `hook_id`: ID do webhook no GitLab.
    ///
    /// ## Returns
    /// `Result<Hook, GitLabError>` — dados do webhook.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get(&self, project_id: u64, hook_id: u64) -> Result<Hook, GitLabError> {
        let path = format!("projects/{}/hooks/{}", project_id, hook_id);
        self.http.get(&path, &[], "project_hooks.get").await
    }

    /// Cria um novo webhook de projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `payload`: Dados do webhook a criar.
    ///
    /// ## Returns
    /// `Result<Hook, GitLabError>` — dados do webhook criado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn create(
        &self,
        project_id: u64,
        payload: &CreateHookPayload,
    ) -> Result<Hook, GitLabError> {
        let path = format!("projects/{}/hooks", project_id);
        self.http.post(&path, payload, "project_hooks.create").await
    }

    /// Atualiza um webhook de projeto existente.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `hook_id`: ID do webhook no GitLab.
    /// - `payload`: Dados do webhook a atualizar.
    ///
    /// ## Returns
    /// `Result<Hook, GitLabError>` — dados do webhook atualizado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn update(
        &self,
        project_id: u64,
        hook_id: u64,
        payload: &UpdateHookPayload,
    ) -> Result<Hook, GitLabError> {
        let path = format!("projects/{}/hooks/{}", project_id, hook_id);
        self.http.put(&path, payload, "project_hooks.update").await
    }

    /// Remove um webhook de projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `hook_id`: ID do webhook no GitLab.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn delete(&self, project_id: u64, hook_id: u64) -> Result<(), GitLabError> {
        let path = format!("projects/{}/hooks/{}", project_id, hook_id);
        self.http.delete(&path, &[], "project_hooks.delete").await
    }
}
