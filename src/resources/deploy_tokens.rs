use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use std::sync::Arc;

/// Recurso de API para operações com tokens de deploy no GitLab.
#[derive(Debug)]
pub struct DeployTokensResource {
    http: Arc<HttpClient>,
}

impl DeployTokensResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista todos os tokens de deploy de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<DeployToken>, GitLabError>` — lista de tokens de deploy.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list_project_tokens(
        &self,
        project_id: u64,
    ) -> Result<Vec<DeployToken>, GitLabError> {
        let path = format!("projects/{}/deploy_tokens", project_id);
        self.http.get(&path, &[], "deploy_tokens.list_project").await
    }

    /// Lista todos os tokens de deploy de um grupo.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<DeployToken>, GitLabError>` — lista de tokens de deploy.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list_group_tokens(&self, group_id: u64) -> Result<Vec<DeployToken>, GitLabError> {
        let path = format!("groups/{}/deploy_tokens", group_id);
        self.http.get(&path, &[], "deploy_tokens.list_group").await
    }

    /// Cria um novo token de deploy em um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `payload`: Dados para criar o token de deploy.
    ///
    /// ## Returns
    /// `Result<DeployToken, GitLabError>` — dados do token de deploy criado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn create_project_token(
        &self,
        project_id: u64,
        payload: &CreateDeployTokenPayload,
    ) -> Result<DeployToken, GitLabError> {
        let path = format!("projects/{}/deploy_tokens", project_id);
        self.http.post(&path, &payload, "deploy_tokens.create_project").await
    }

    /// Cria um novo token de deploy em um grupo.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    /// - `payload`: Dados para criar o token de deploy.
    ///
    /// ## Returns
    /// `Result<DeployToken, GitLabError>` — dados do token de deploy criado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn create_group_token(
        &self,
        group_id: u64,
        payload: &CreateDeployTokenPayload,
    ) -> Result<DeployToken, GitLabError> {
        let path = format!("groups/{}/deploy_tokens", group_id);
        self.http.post(&path, &payload, "deploy_tokens.create_group").await
    }

    /// Revoga um token de deploy de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `token_id`: ID do token de deploy a ser revogado.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn revoke_project_token(
        &self,
        project_id: u64,
        token_id: u64,
    ) -> Result<(), GitLabError> {
        let path = format!("projects/{}/deploy_tokens/{}", project_id, token_id);
        self.http.delete(&path, &[], "deploy_tokens.revoke_project").await
    }

    /// Revoga um token de deploy de um grupo.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    /// - `token_id`: ID do token de deploy a ser revogado.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn revoke_group_token(
        &self,
        group_id: u64,
        token_id: u64,
    ) -> Result<(), GitLabError> {
        let path = format!("groups/{}/deploy_tokens/{}", group_id, token_id);
        self.http.delete(&path, &[], "deploy_tokens.revoke_group").await
    }

    /// Lista todos os tokens de deploy disponíveis.
    ///
    /// ## Returns
    /// `Result<Vec<DeployToken>, GitLabError>` — lista de todos os tokens de deploy.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list_all(&self) -> Result<Vec<DeployToken>, GitLabError> {
        let path = "deploy_tokens".to_string();
        self.http.get(&path, &[], "deploy_tokens.list_all").await
    }
}
