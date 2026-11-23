use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::{AccessRequest, AccessRequestFilter};
use crate::utils::encoding::filter_to_query;
use std::sync::Arc;

/// Recurso de API para operações com solicitações de acesso a projetos no GitLab.
#[derive(Debug)]
pub struct AccessRequestsResource {
    http: Arc<HttpClient>,
}

impl AccessRequestsResource {
    /// Cria uma nova instância do recurso.
    pub(crate) const fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista todas as solicitações de acesso de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `filter`: Filtros opcionais para a consulta.
    ///
    /// ## Returns
    /// `Result<Vec<AccessRequest>, GitLabError>` — lista de solicitações de acesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list(
        &self,
        project_id: u64,
        filter: Option<&AccessRequestFilter>,
    ) -> Result<Vec<AccessRequest>, GitLabError> {
        let path = format!("projects/{project_id}/access_requests");
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "access_requests.list").await
    }

    /// Solicita acesso a um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    ///
    /// ## Returns
    /// `Result<AccessRequest, GitLabError>` — dados da solicitação de acesso criada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn request(&self, project_id: u64) -> Result<AccessRequest, GitLabError> {
        let path = format!("projects/{project_id}/access_requests");
        self.http
            .post(&path, &serde_json::json!({}), "access_requests.request")
            .await
    }

    /// Aprova uma solicitação de acesso a um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `user_id`: ID do usuário a ser aprovado.
    ///
    /// ## Returns
    /// `Result<AccessRequest, GitLabError>` — dados da solicitação de acesso aprovada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn approve(
        &self,
        project_id: u64,
        user_id: u64,
    ) -> Result<AccessRequest, GitLabError> {
        let path = format!("projects/{project_id}/access_requests/{user_id}/approve");
        self.http
            .put(&path, &serde_json::json!({}), "access_requests.approve")
            .await
    }

    /// Nega uma solicitação de acesso a um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `user_id`: ID do usuário a ser negado.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn deny(&self, project_id: u64, user_id: u64) -> Result<(), GitLabError> {
        let path = format!("projects/{project_id}/access_requests/{user_id}");
        self.http.delete(&path, &[], "access_requests.deny").await
    }
}
