use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use crate::utils::encoding::{encode_query_param, filter_to_query};
use std::sync::Arc;

/// Recurso de API para operações com variáveis de CI/CD em nível de projeto no GitLab.
#[derive(Debug)]
pub struct VariablesResource {
    http: Arc<HttpClient>,
}

impl VariablesResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista todas as variáveis de CI/CD de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `filter`: Filtro opcional (paginação).
    ///
    /// ## Returns
    /// `Result<Vec<CiVariable>, GitLabError>` — lista de variáveis.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list(
        &self,
        project_id: u64,
        filter: Option<&VariableFilter>,
    ) -> Result<Vec<CiVariable>, GitLabError> {
        let path = format!("projects/{}/variables", project_id);
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "variables.list").await
    }

    /// Obtém uma variável de CI/CD pela chave.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `key`: Chave (nome) da variável.
    ///
    /// ## Returns
    /// `Result<CiVariable, GitLabError>` — dados da variável.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get(&self, project_id: u64, key: &str) -> Result<CiVariable, GitLabError> {
        let path = format!("projects/{}/variables/{}", project_id, encode_query_param(key));
        self.http.get(&path, &[], "variables.get").await
    }

    /// Cria uma nova variável de CI/CD.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `payload`: Dados da variável a criar.
    ///
    /// ## Returns
    /// `Result<CiVariable, GitLabError>` — dados da variável criada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn create(
        &self,
        project_id: u64,
        payload: &CreateCiVariablePayload,
    ) -> Result<CiVariable, GitLabError> {
        let path = format!("projects/{}/variables", project_id);
        self.http.post(&path, payload, "variables.create").await
    }

    /// Atualiza uma variável de CI/CD existente.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `key`: Chave (nome) da variável.
    /// - `payload`: Dados da variável a atualizar.
    ///
    /// ## Returns
    /// `Result<CiVariable, GitLabError>` — dados da variável atualizada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn update(
        &self,
        project_id: u64,
        key: &str,
        payload: &UpdateCiVariablePayload,
    ) -> Result<CiVariable, GitLabError> {
        let path = format!("projects/{}/variables/{}", project_id, encode_query_param(key));
        self.http.put(&path, payload, "variables.update").await
    }

    /// Remove uma variável de CI/CD.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `key`: Chave (nome) da variável.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn delete(&self, project_id: u64, key: &str) -> Result<(), GitLabError> {
        let path = format!("projects/{}/variables/{}", project_id, encode_query_param(key));
        self.http.delete(&path, &[], "variables.delete").await
    }
}
