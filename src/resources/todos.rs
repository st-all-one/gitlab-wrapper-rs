use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use crate::utils::encoding::filter_to_query;
use std::sync::Arc;

/// Recurso de API para operações com tarefas (todos) no GitLab.
#[derive(Debug)]
pub struct TodosResource {
    http: Arc<HttpClient>,
}

impl TodosResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista tarefas com filtros opcionais.
    ///
    /// ## Params
    /// - `filter`: Filtros opcionais para a consulta.
    ///
    /// ## Returns
    /// `Result<Vec<Todo>, GitLabError>` — lista de tarefas.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list(&self, filter: Option<&TodoFilter>) -> Result<Vec<Todo>, GitLabError> {
        let query = filter_to_query(filter);
        self.http.get("todos", &query, "todos.list").await
    }

    /// Marca uma tarefa como concluída.
    ///
    /// ## Params
    /// - `todo_id`: ID da tarefa no GitLab.
    ///
    /// ## Returns
    /// `Result<Todo, GitLabError>` — dados da tarefa atualizada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn mark_done(&self, todo_id: u64) -> Result<Todo, GitLabError> {
        let path = format!("todos/{}/mark_as_done", todo_id);
        self.http.post(&path, &serde_json::json!({}), "todos.mark_done").await
    }

    /// Marca todas as tarefas como concluídas.
    ///
    /// ## Params
    /// Nenhum.
    ///
    /// ## Returns
    /// `Result<Vec<Todo>, GitLabError>` — lista de tarefas atualizadas.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn mark_all_done(&self) -> Result<Vec<Todo>, GitLabError> {
        self.http
            .post("todos/mark_all_as_done", &serde_json::json!({}), "todos.mark_all_done")
            .await
    }
}
