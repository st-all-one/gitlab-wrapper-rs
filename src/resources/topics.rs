use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use crate::utils::encoding::filter_to_query;
use std::sync::Arc;

/// Recurso de API para operações com tópicos no GitLab.
#[derive(Debug)]
pub struct TopicsResource {
    http: Arc<HttpClient>,
}

impl TopicsResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista todos os tópicos com filtros opcionais.
    ///
    /// ## Params
    /// - `filter`: Filtros opcionais para a consulta.
    ///
    /// ## Returns
    /// `Result<Vec<Topic>, GitLabError>` — lista de tópicos.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list(&self, filter: Option<&TopicFilter>) -> Result<Vec<Topic>, GitLabError> {
        let path = "topics".to_string();
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "topics.list").await
    }

    /// Obtém um tópico pelo ID.
    ///
    /// ## Params
    /// - `topic_id`: ID do tópico no GitLab.
    ///
    /// ## Returns
    /// `Result<Topic, GitLabError>` — dados do tópico solicitado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get(&self, topic_id: u64) -> Result<Topic, GitLabError> {
        let path = format!("topics/{}", topic_id);
        self.http.get(&path, &[], "topics.get").await
    }

    /// Cria um novo tópico.
    ///
    /// ## Params
    /// - `payload`: Dados para criar o tópico.
    ///
    /// ## Returns
    /// `Result<Topic, GitLabError>` — dados do tópico criado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn create(&self, payload: &CreateTopicPayload) -> Result<Topic, GitLabError> {
        let path = "topics".to_string();
        self.http.post(&path, &payload, "topics.create").await
    }

    /// Atualiza um tópico existente.
    ///
    /// ## Params
    /// - `topic_id`: ID do tópico a ser atualizado.
    /// - `payload`: Dados para atualizar o tópico.
    ///
    /// ## Returns
    /// `Result<Topic, GitLabError>` — dados do tópico atualizado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn update(
        &self,
        topic_id: u64,
        payload: &UpdateTopicPayload,
    ) -> Result<Topic, GitLabError> {
        let path = format!("topics/{}", topic_id);
        self.http.put(&path, &payload, "topics.update").await
    }

    /// Remove um tópico.
    ///
    /// ## Params
    /// - `topic_id`: ID do tópico a ser removido.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn delete(&self, topic_id: u64) -> Result<(), GitLabError> {
        let path = format!("topics/{}", topic_id);
        self.http.delete(&path, &[], "topics.delete").await
    }
}
