use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use crate::utils::encoding::{encode_query_param, filter_to_query};
use std::sync::Arc;

/// Recurso de API para operações com branches protegidos no GitLab.
#[derive(Debug)]
pub struct ProtectedBranchesResource {
    http: Arc<HttpClient>,
}

impl ProtectedBranchesResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista todos os branches protegidos de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `filter`: Filtro opcional (paginação, busca).
    ///
    /// ## Returns
    /// `Result<Vec<ProtectedBranch>, GitLabError>` — lista de branches protegidos.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list(
        &self,
        project_id: u64,
        filter: Option<&ProtectedBranchFilter>,
    ) -> Result<Vec<ProtectedBranch>, GitLabError> {
        let path = format!("projects/{}/protected_branches", project_id);
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "protected_branches.list").await
    }

    /// Obtém um branch protegido pelo nome.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `name`: Nome do branch protegido.
    ///
    /// ## Returns
    /// `Result<ProtectedBranch, GitLabError>` — dados do branch protegido.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get(&self, project_id: u64, name: &str) -> Result<ProtectedBranch, GitLabError> {
        let path =
            format!("projects/{}/protected_branches/{}", project_id, encode_query_param(name));
        self.http.get(&path, &[], "protected_branches.get").await
    }

    /// Protege um branch (cria regra de proteção).
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `payload`: Dados do branch a proteger.
    ///
    /// ## Returns
    /// `Result<ProtectedBranch, GitLabError>` — dados do branch protegido.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn protect(
        &self,
        project_id: u64,
        payload: &ProtectBranchPayload,
    ) -> Result<ProtectedBranch, GitLabError> {
        let path = format!("projects/{}/protected_branches", project_id);
        self.http.post(&path, payload, "protected_branches.protect").await
    }

    /// Remove a proteção de um branch.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `name`: Nome do branch a desproteger.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn unprotect(&self, project_id: u64, name: &str) -> Result<(), GitLabError> {
        let path =
            format!("projects/{}/protected_branches/{}", project_id, encode_query_param(name));
        self.http.delete(&path, &[], "protected_branches.unprotect").await
    }
}
