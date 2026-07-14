use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use std::sync::Arc;

/// Recurso de API para operações com regras de push de grupo no GitLab.
#[derive(Debug)]
pub struct GroupPushRulesResource {
    http: Arc<HttpClient>,
}

impl GroupPushRulesResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Obtém a regra de push de um grupo.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    ///
    /// ## Returns
    /// `Result<GroupPushRule, GitLabError>` — dados da regra de push.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get(&self, group_id: u64) -> Result<GroupPushRule, GitLabError> {
        let path = format!("groups/{}/push_rule", group_id);
        self.http.get(&path, &[], "group_push_rules.get").await
    }

    /// Cria uma regra de push para um grupo.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    /// - `payload`: Dados para criar a regra de push.
    ///
    /// ## Returns
    /// `Result<GroupPushRule, GitLabError>` — dados da regra de push criada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn create(
        &self,
        group_id: u64,
        payload: &CreateGroupPushRulePayload,
    ) -> Result<GroupPushRule, GitLabError> {
        let path = format!("groups/{}/push_rule", group_id);
        self.http.post(&path, payload, "group_push_rules.create").await
    }

    /// Atualiza a regra de push de um grupo.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    /// - `payload`: Dados para atualizar a regra de push.
    ///
    /// ## Returns
    /// `Result<GroupPushRule, GitLabError>` — dados da regra de push atualizada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn update(
        &self,
        group_id: u64,
        payload: &UpdateGroupPushRulePayload,
    ) -> Result<GroupPushRule, GitLabError> {
        let path = format!("groups/{}/push_rule", group_id);
        self.http.put(&path, payload, "group_push_rules.update").await
    }

    /// Remove a regra de push de um grupo.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn delete(&self, group_id: u64) -> Result<(), GitLabError> {
        let path = format!("groups/{}/push_rule", group_id);
        self.http.delete(&path, &[], "group_push_rules.delete").await
    }
}
