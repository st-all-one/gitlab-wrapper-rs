use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use std::sync::Arc;
use crate::types::*;
use crate::utils::encoding::encode_query_param;

/// Recurso de API para operações com branches no GitLab.
#[derive(Debug)]
pub struct BranchesResource {
    http: Arc<HttpClient>,
}

impl BranchesResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista branches de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<Branch>, GitLabError>` — lista de branches do projeto.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn list(&self, project_id: u64) -> Result<Vec<Branch>, GitLabError> {
        let path = format!("projects/{}/repository/branches", project_id);
        self.http.get(&path, &[], "branches.list")
    }

    /// Obtém um branch pelo nome.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `branch`: Nome do branch.
    ///
    /// ## Returns
    /// `Result<Branch, GitLabError>` — dados do branch solicitado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn get(&self, project_id: u64, branch: &str) -> Result<Branch, GitLabError> {
        let path = format!("projects/{}/repository/branches/{}", project_id, encode_query_param(branch));
        self.http.get(&path, &[], "branches.get")
    }

    /// Cria um novo branch em um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `payload`: Dados para criar o branch.
    ///
    /// ## Returns
    /// `Result<Branch, GitLabError>` — dados do branch criado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn create(&self, project_id: u64, payload: &CreateBranchPayload) -> Result<Branch, GitLabError> {
        let path = format!("projects/{}/repository/branches", project_id);
        self.http.post(&path, &payload, "branches.create")
    }

    /// Remove um branch de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `branch`: Nome do branch a ser removido.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn delete(&self, project_id: u64, branch: &str) -> Result<(), GitLabError> {
        let path = format!("projects/{}/repository/branches/{}", project_id, encode_query_param(branch));
        self.http.delete(&path, &[], "branches.delete")
    }

    /// Remove todos os branches mesclados de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn delete_merged(&self, project_id: u64) -> Result<(), GitLabError> {
        let path = format!("projects/{}/repository/merged_branches", project_id);
        self.http.delete(&path, &[], "branches.delete_merged")
    }
}
