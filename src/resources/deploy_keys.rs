use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use std::sync::Arc;

/// Recurso de API para operações com chaves de deploy no GitLab.
#[derive(Debug)]
pub struct DeployKeysResource {
    http: Arc<HttpClient>,
}

impl DeployKeysResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista todas as chaves de deploy de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<DeployKey>, GitLabError>` — lista de chaves de deploy.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list(&self, project_id: u64) -> Result<Vec<DeployKey>, GitLabError> {
        let path = format!("projects/{}/deploy_keys", project_id);
        self.http.get(&path, &[], "deploy_keys.list").await
    }

    /// Obtém uma chave de deploy pelo ID.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `key_id`: ID da chave de deploy no GitLab.
    ///
    /// ## Returns
    /// `Result<DeployKey, GitLabError>` — dados da chave de deploy solicitada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get(&self, project_id: u64, key_id: u64) -> Result<DeployKey, GitLabError> {
        let path = format!("projects/{}/deploy_keys/{}", project_id, key_id);
        self.http.get(&path, &[], "deploy_keys.get").await
    }

    /// Cria uma nova chave de deploy em um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `payload`: Dados para criar a chave de deploy.
    ///
    /// ## Returns
    /// `Result<DeployKey, GitLabError>` — dados da chave de deploy criada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn create(
        &self,
        project_id: u64,
        payload: &CreateDeployKeyPayload,
    ) -> Result<DeployKey, GitLabError> {
        let path = format!("projects/{}/deploy_keys", project_id);
        self.http.post(&path, &payload, "deploy_keys.create").await
    }

    /// Remove uma chave de deploy de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `key_id`: ID da chave de deploy no GitLab.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn delete(&self, project_id: u64, key_id: u64) -> Result<(), GitLabError> {
        let path = format!("projects/{}/deploy_keys/{}", project_id, key_id);
        self.http.delete(&path, &[], "deploy_keys.delete").await
    }

    /// Atualiza uma chave de deploy existente.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `key_id`: ID da chave de deploy no GitLab.
    /// - `payload`: Dados para atualizar a chave de deploy.
    ///
    /// ## Returns
    /// `Result<DeployKey, GitLabError>` — dados da chave de deploy atualizada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn update(
        &self,
        project_id: u64,
        key_id: u64,
        payload: &UpdateDeployKeyPayload,
    ) -> Result<DeployKey, GitLabError> {
        let path = format!("projects/{}/deploy_keys/{}", project_id, key_id);
        self.http.put(&path, &payload, "deploy_keys.update").await
    }

    /// Ativa uma chave de deploy em um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `key_id`: ID da chave de deploy no GitLab.
    ///
    /// ## Returns
    /// `Result<DeployKey, GitLabError>` — dados da chave de deploy ativada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn enable(&self, project_id: u64, key_id: u64) -> Result<DeployKey, GitLabError> {
        let path = format!("projects/{}/deploy_keys/{}/enable", project_id, key_id);
        self.http.post(&path, &serde_json::Value::Null, "deploy_keys.enable").await
    }
}
