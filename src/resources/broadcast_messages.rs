use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use std::sync::Arc;

/// Recurso de API para operações com mensagens broadcast no GitLab.
#[derive(Debug)]
pub struct BroadcastMessagesResource {
    http: Arc<HttpClient>,
}

impl BroadcastMessagesResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista todas as mensagens broadcast.
    ///
    /// ## Returns
    /// `Result<Vec<BroadcastMessage>, GitLabError>` — lista de mensagens broadcast.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list(&self) -> Result<Vec<BroadcastMessage>, GitLabError> {
        let path = "broadcast_messages".to_string();
        self.http.get(&path, &[], "broadcast_messages.list").await
    }

    /// Obtém uma mensagem broadcast pelo ID.
    ///
    /// ## Params
    /// - `message_id`: ID da mensagem broadcast.
    ///
    /// ## Returns
    /// `Result<BroadcastMessage, GitLabError>` — dados da mensagem.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get(&self, message_id: u64) -> Result<BroadcastMessage, GitLabError> {
        let path = format!("broadcast_messages/{}", message_id);
        self.http.get(&path, &[], "broadcast_messages.get").await
    }

    /// Cria uma nova mensagem broadcast.
    ///
    /// ## Params
    /// - `payload`: Dados da mensagem a criar.
    ///
    /// ## Returns
    /// `Result<BroadcastMessage, GitLabError>` — dados da mensagem criada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn create(
        &self,
        payload: &CreateBroadcastMessagePayload,
    ) -> Result<BroadcastMessage, GitLabError> {
        let path = "broadcast_messages".to_string();
        self.http.post(&path, payload, "broadcast_messages.create").await
    }

    /// Atualiza uma mensagem broadcast existente.
    ///
    /// ## Params
    /// - `message_id`: ID da mensagem broadcast.
    /// - `payload`: Dados da mensagem a atualizar.
    ///
    /// ## Returns
    /// `Result<BroadcastMessage, GitLabError>` — dados da mensagem atualizada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn update(
        &self,
        message_id: u64,
        payload: &UpdateBroadcastMessagePayload,
    ) -> Result<BroadcastMessage, GitLabError> {
        let path = format!("broadcast_messages/{}", message_id);
        self.http.put(&path, payload, "broadcast_messages.update").await
    }

    /// Remove uma mensagem broadcast.
    ///
    /// ## Params
    /// - `message_id`: ID da mensagem broadcast a remover.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn delete(&self, message_id: u64) -> Result<(), GitLabError> {
        let path = format!("broadcast_messages/{}", message_id);
        self.http.delete(&path, &[], "broadcast_messages.delete").await
    }
}
