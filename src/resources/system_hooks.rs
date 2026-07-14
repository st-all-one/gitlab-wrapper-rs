use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use std::sync::Arc;

/// Recurso de API para operações com system hooks (webhooks globais) no GitLab.
#[derive(Debug)]
pub struct SystemHooksResource {
    http: Arc<HttpClient>,
}

impl SystemHooksResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista todos os system hooks.
    ///
    /// ## Returns
    /// `Result<Vec<SystemHook>, GitLabError>` — lista de system hooks.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list(&self) -> Result<Vec<SystemHook>, GitLabError> {
        let path = "hooks".to_string();
        self.http.get(&path, &[], "system_hooks.list").await
    }

    /// Obtém um system hook pelo ID.
    ///
    /// ## Params
    /// - `hook_id`: ID do system hook.
    ///
    /// ## Returns
    /// `Result<SystemHook, GitLabError>` — dados do system hook.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get(&self, hook_id: u64) -> Result<SystemHook, GitLabError> {
        let path = format!("hooks/{}", hook_id);
        self.http.get(&path, &[], "system_hooks.get").await
    }

    /// Cria um novo system hook.
    ///
    /// ## Params
    /// - `payload`: Dados do system hook a criar.
    ///
    /// ## Returns
    /// `Result<SystemHook, GitLabError>` — dados do system hook criado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn create(
        &self,
        payload: &CreateSystemHookPayload,
    ) -> Result<SystemHook, GitLabError> {
        let path = "hooks".to_string();
        self.http.post(&path, payload, "system_hooks.create").await
    }

    /// Remove um system hook.
    ///
    /// ## Params
    /// - `hook_id`: ID do system hook a remover.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn delete(&self, hook_id: u64) -> Result<(), GitLabError> {
        let path = format!("hooks/{}", hook_id);
        self.http.delete(&path, &[], "system_hooks.delete").await
    }
}
