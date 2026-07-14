use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use std::sync::Arc;

/// Recurso de API para operações de pesquisa no GitLab.
#[derive(Debug)]
pub struct SearchResource {
    http: Arc<HttpClient>,
}

impl SearchResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Pesquisa globalmente em todo o GitLab.
    ///
    /// ## Params
    /// - `scope`: Escopo da pesquisa (ex: "projects", "issues", "merge_requests").
    /// - `search`: Termo de pesquisa.
    ///
    /// ## Returns
    /// `Result<Vec<SearchResultItem>, GitLabError>` — lista de resultados da pesquisa.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn global(
        &self,
        scope: &str,
        search: &str,
    ) -> Result<Vec<SearchResultItem>, GitLabError> {
        let query = vec![
            ("scope".to_string(), scope.to_string()),
            ("search".to_string(), search.to_string()),
        ];
        self.http.get("search", &query, "search.global").await
    }

    /// Pesquisa dentro de um grupo específico.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    /// - `scope`: Escopo da pesquisa (ex: "projects", "issues", "merge_requests").
    /// - `search`: Termo de pesquisa.
    ///
    /// ## Returns
    /// `Result<Vec<SearchResultItem>, GitLabError>` — lista de resultados da pesquisa.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn in_group(
        &self,
        group_id: u64,
        scope: &str,
        search: &str,
    ) -> Result<Vec<SearchResultItem>, GitLabError> {
        let path = format!("groups/{}/search", group_id);
        let query = vec![
            ("scope".to_string(), scope.to_string()),
            ("search".to_string(), search.to_string()),
        ];
        self.http.get(&path, &query, "search.in_group").await
    }

    /// Pesquisa dentro de um projeto específico.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `scope`: Escopo da pesquisa (ex: "issues", "merge_requests", "milestones").
    /// - `search`: Termo de pesquisa.
    ///
    /// ## Returns
    /// `Result<Vec<SearchResultItem>, GitLabError>` — lista de resultados da pesquisa.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn in_project(
        &self,
        project_id: u64,
        scope: &str,
        search: &str,
    ) -> Result<Vec<SearchResultItem>, GitLabError> {
        let path = format!("projects/{}/search", project_id);
        let query = vec![
            ("scope".to_string(), scope.to_string()),
            ("search".to_string(), search.to_string()),
        ];
        self.http.get(&path, &query, "search.in_project").await
    }
}
