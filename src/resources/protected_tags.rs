use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use crate::utils::encoding::{encode_query_param, filter_to_query};
use std::sync::Arc;

/// Recurso de API para operações com tags protegidas no GitLab.
#[derive(Debug)]
pub struct ProtectedTagsResource {
    http: Arc<HttpClient>,
}

impl ProtectedTagsResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista todas as tags protegidas de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `filter`: Filtro opcional (paginação, busca).
    ///
    /// ## Returns
    /// `Result<Vec<ProtectedTag>, GitLabError>` — lista de tags protegidas.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list(
        &self,
        project_id: u64,
        filter: Option<&ProtectedTagFilter>,
    ) -> Result<Vec<ProtectedTag>, GitLabError> {
        let path = format!("projects/{}/protected_tags", project_id);
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "protected_tags.list").await
    }

    /// Obtém uma tag protegida pelo nome.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `name`: Nome ou padrão da tag protegida.
    ///
    /// ## Returns
    /// `Result<ProtectedTag, GitLabError>` — dados da tag protegida.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get(&self, project_id: u64, name: &str) -> Result<ProtectedTag, GitLabError> {
        let path = format!("projects/{}/protected_tags/{}", project_id, encode_query_param(name));
        self.http.get(&path, &[], "protected_tags.get").await
    }

    /// Protege uma tag (cria regra de proteção).
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `payload`: Dados da tag a proteger.
    ///
    /// ## Returns
    /// `Result<ProtectedTag, GitLabError>` — dados da tag protegida.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn protect(
        &self,
        project_id: u64,
        payload: &ProtectTagPayload,
    ) -> Result<ProtectedTag, GitLabError> {
        let path = format!("projects/{}/protected_tags", project_id);
        self.http.post(&path, payload, "protected_tags.protect").await
    }

    /// Remove a proteção de uma tag.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `name`: Nome ou padrão da tag a desproteger.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn unprotect(&self, project_id: u64, name: &str) -> Result<(), GitLabError> {
        let path = format!("projects/{}/protected_tags/{}", project_id, encode_query_param(name));
        self.http.delete(&path, &[], "protected_tags.unprotect").await
    }
}
