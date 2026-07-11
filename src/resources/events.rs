use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use std::sync::Arc;
use crate::types::*;
use crate::utils::encoding::filter_to_query;

/// Recurso de API para operações com eventos no GitLab.
#[derive(Debug)]
pub struct EventsResource {
    http: Arc<HttpClient>,
}

impl EventsResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista eventos com filtros opcionais.
    ///
    /// ## Params
    /// - `filter`: Filtros opcionais para a consulta.
    ///
    /// ## Returns
    /// `Result<Vec<Event>, GitLabError>` — lista de eventos.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn list(&self, filter: Option<&EventFilter>) -> Result<Vec<Event>, GitLabError> {
        let query = filter_to_query(filter);
        self.http.get("events", &query, "events.list")
    }

    /// Lista eventos de um usuário específico.
    ///
    /// ## Params
    /// - `user_id`: ID do usuário no GitLab.
    /// - `filter`: Filtros opcionais para a consulta.
    ///
    /// ## Returns
    /// `Result<Vec<Event>, GitLabError>` — lista de eventos do usuário.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn list_user_events(&self, user_id: u64, filter: Option<&EventFilter>) -> Result<Vec<Event>, GitLabError> {
        let path = format!("users/{}/events", user_id);
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "events.list_user")
    }

    /// Lista eventos de um projeto específico.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `filter`: Filtros opcionais para a consulta.
    ///
    /// ## Returns
    /// `Result<Vec<Event>, GitLabError>` — lista de eventos do projeto.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn list_project_events(&self, project_id: u64, filter: Option<&EventFilter>) -> Result<Vec<Event>, GitLabError> {
        let path = format!("projects/{}/events", project_id);
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "events.list_project")
    }
}
