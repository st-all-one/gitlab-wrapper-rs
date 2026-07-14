use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use crate::utils::encoding::filter_to_query;
use std::sync::Arc;

/// Recurso de API para operações com webhooks de grupo no GitLab.
#[derive(Debug)]
pub struct GroupWebhooksResource {
    http: Arc<HttpClient>,
}

impl GroupWebhooksResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista todos os webhooks de um grupo.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    /// - `filter`: Filtro opcional (paginação).
    ///
    /// ## Returns
    /// `Result<Vec<GroupHook>, GitLabError>` — lista de webhooks.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list(
        &self,
        group_id: u64,
        filter: Option<&GroupHookFilter>,
    ) -> Result<Vec<GroupHook>, GitLabError> {
        let path = format!("groups/{}/hooks", group_id);
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "group_webhooks.list").await
    }

    /// Obtém um webhook de grupo por ID.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    /// - `hook_id`: ID do webhook no GitLab.
    ///
    /// ## Returns
    /// `Result<GroupHook, GitLabError>` — dados do webhook.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get(&self, group_id: u64, hook_id: u64) -> Result<GroupHook, GitLabError> {
        let path = format!("groups/{}/hooks/{}", group_id, hook_id);
        self.http.get(&path, &[], "group_webhooks.get").await
    }

    /// Cria um novo webhook de grupo.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    /// - `payload`: Dados do webhook a criar.
    ///
    /// ## Returns
    /// `Result<GroupHook, GitLabError>` — dados do webhook criado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn create(
        &self,
        group_id: u64,
        payload: &CreateGroupHookPayload,
    ) -> Result<GroupHook, GitLabError> {
        let path = format!("groups/{}/hooks", group_id);
        self.http.post(&path, payload, "group_webhooks.create").await
    }

    /// Atualiza um webhook de grupo existente.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    /// - `hook_id`: ID do webhook no GitLab.
    /// - `payload`: Dados do webhook a atualizar.
    ///
    /// ## Returns
    /// `Result<GroupHook, GitLabError>` — dados do webhook atualizado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn update(
        &self,
        group_id: u64,
        hook_id: u64,
        payload: &UpdateGroupHookPayload,
    ) -> Result<GroupHook, GitLabError> {
        let path = format!("groups/{}/hooks/{}", group_id, hook_id);
        self.http.put(&path, payload, "group_webhooks.update").await
    }

    /// Remove um webhook de grupo.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    /// - `hook_id`: ID do webhook no GitLab.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn delete(&self, group_id: u64, hook_id: u64) -> Result<(), GitLabError> {
        let path = format!("groups/{}/hooks/{}", group_id, hook_id);
        self.http.delete(&path, &[], "group_webhooks.delete").await
    }
}
