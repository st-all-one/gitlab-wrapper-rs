use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use std::sync::Arc;

/// Recurso de API para operações com tokens de acesso de projetos e grupos no GitLab.
#[derive(Debug)]
pub struct AccessTokensResource {
    http: Arc<HttpClient>,
}

impl AccessTokensResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista todos os tokens de acesso de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<AccessToken>, GitLabError>` — lista de tokens de acesso do projeto.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list_project_tokens(
        &self,
        project_id: u64,
    ) -> Result<Vec<AccessToken>, GitLabError> {
        let path = format!("projects/{}/access_tokens", project_id);
        self.http.get(&path, &[], "access_tokens.list_project").await
    }

    /// Obtém um token de acesso específico de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `token_id`: ID do token de acesso.
    ///
    /// ## Returns
    /// `Result<AccessToken, GitLabError>` — dados do token de acesso solicitado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get_project_token(
        &self,
        project_id: u64,
        token_id: u64,
    ) -> Result<AccessToken, GitLabError> {
        let path = format!("projects/{}/access_tokens/{}", project_id, token_id);
        self.http.get(&path, &[], "access_tokens.get_project").await
    }

    /// Cria um novo token de acesso em um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `payload`: Dados para criar o token de acesso.
    ///
    /// ## Returns
    /// `Result<AccessToken, GitLabError>` — dados do token de acesso criado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn create_project_token(
        &self,
        project_id: u64,
        payload: &CreateAccessTokenPayload,
    ) -> Result<AccessToken, GitLabError> {
        let path = format!("projects/{}/access_tokens", project_id);
        self.http.post(&path, &payload, "access_tokens.create_project").await
    }

    /// Revoga um token de acesso de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `token_id`: ID do token de acesso a ser revogado.
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
        let path = format!("projects/{}/access_tokens/{}", project_id, token_id);
        self.http.delete(&path, &[], "access_tokens.revoke_project").await
    }

    /// Lista todos os tokens de acesso de um grupo.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<AccessToken>, GitLabError>` — lista de tokens de acesso do grupo.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list_group_tokens(&self, group_id: u64) -> Result<Vec<AccessToken>, GitLabError> {
        let path = format!("groups/{}/access_tokens", group_id);
        self.http.get(&path, &[], "access_tokens.list_group").await
    }

    /// Obtém um token de acesso específico de um grupo.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    /// - `token_id`: ID do token de acesso.
    ///
    /// ## Returns
    /// `Result<AccessToken, GitLabError>` — dados do token de acesso solicitado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get_group_token(
        &self,
        group_id: u64,
        token_id: u64,
    ) -> Result<AccessToken, GitLabError> {
        let path = format!("groups/{}/access_tokens/{}", group_id, token_id);
        self.http.get(&path, &[], "access_tokens.get_group").await
    }

    /// Cria um novo token de acesso em um grupo.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    /// - `payload`: Dados para criar o token de acesso.
    ///
    /// ## Returns
    /// `Result<AccessToken, GitLabError>` — dados do token de acesso criado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn create_group_token(
        &self,
        group_id: u64,
        payload: &CreateAccessTokenPayload,
    ) -> Result<AccessToken, GitLabError> {
        let path = format!("groups/{}/access_tokens", group_id);
        self.http.post(&path, &payload, "access_tokens.create_group").await
    }

    /// Revoga um token de acesso de um grupo.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    /// - `token_id`: ID do token de acesso a ser revogado.
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
        let path = format!("groups/{}/access_tokens/{}", group_id, token_id);
        self.http.delete(&path, &[], "access_tokens.revoke_group").await
    }
}
