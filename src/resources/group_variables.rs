use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use crate::utils::encoding::{encode_query_param, filter_to_query};
use std::sync::Arc;

/// Recurso de API para operações com variáveis de CI/CD em nível de grupo no GitLab.
#[derive(Debug)]
pub struct GroupVariablesResource {
    http: Arc<HttpClient>,
}

impl GroupVariablesResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista todas as variáveis de CI/CD de um grupo.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    /// - `filter`: Filtro opcional (paginação).
    ///
    /// ## Returns
    /// `Result<Vec<GroupVariable>, GitLabError>` — lista de variáveis.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list(
        &self,
        group_id: u64,
        filter: Option<&GroupVariableFilter>,
    ) -> Result<Vec<GroupVariable>, GitLabError> {
        let path = format!("groups/{}/variables", group_id);
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "group_variables.list").await
    }

    /// Obtém uma variável de grupo pela chave.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    /// - `key`: Chave (nome) da variável.
    ///
    /// ## Returns
    /// `Result<GroupVariable, GitLabError>` — dados da variável.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get(&self, group_id: u64, key: &str) -> Result<GroupVariable, GitLabError> {
        let path = format!("groups/{}/variables/{}", group_id, encode_query_param(key));
        self.http.get(&path, &[], "group_variables.get").await
    }

    /// Cria uma nova variável de grupo.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    /// - `payload`: Dados da variável a criar.
    ///
    /// ## Returns
    /// `Result<GroupVariable, GitLabError>` — dados da variável criada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn create(
        &self,
        group_id: u64,
        payload: &CreateGroupVariablePayload,
    ) -> Result<GroupVariable, GitLabError> {
        let path = format!("groups/{}/variables", group_id);
        self.http.post(&path, payload, "group_variables.create").await
    }

    /// Atualiza uma variável de grupo existente.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    /// - `key`: Chave (nome) da variável.
    /// - `payload`: Dados da variável a atualizar.
    ///
    /// ## Returns
    /// `Result<GroupVariable, GitLabError>` — dados da variável atualizada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn update(
        &self,
        group_id: u64,
        key: &str,
        payload: &UpdateGroupVariablePayload,
    ) -> Result<GroupVariable, GitLabError> {
        let path = format!("groups/{}/variables/{}", group_id, encode_query_param(key));
        self.http.put(&path, payload, "group_variables.update").await
    }

    /// Remove uma variável de grupo.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    /// - `key`: Chave (nome) da variável.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn delete(&self, group_id: u64, key: &str) -> Result<(), GitLabError> {
        let path = format!("groups/{}/variables/{}", group_id, encode_query_param(key));
        self.http.delete(&path, &[], "group_variables.delete").await
    }
}
