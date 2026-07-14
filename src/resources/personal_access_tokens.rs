use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use crate::utils::encoding::filter_to_query;
use std::sync::Arc;

/// Recurso de API para operações com tokens de acesso pessoal no GitLab.
#[derive(Debug)]
pub struct PersonalAccessTokensResource {
    http: Arc<HttpClient>,
}

impl PersonalAccessTokensResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista todos os tokens de acesso pessoal com filtros opcionais.
    ///
    /// ## Params
    /// - `filter`: Filtros opcionais para a consulta.
    ///
    /// ## Returns
    /// `Result<Vec<PersonalAccessToken>, GitLabError>` — lista de tokens de acesso pessoal.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list(
        &self,
        filter: Option<&PersonalAccessTokenFilter>,
    ) -> Result<Vec<PersonalAccessToken>, GitLabError> {
        let path = "personal_access_tokens".to_string();
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "personal_access_tokens.list").await
    }

    /// Obtém um token de acesso pessoal pelo ID.
    ///
    /// ## Params
    /// - `token_id`: ID do token de acesso pessoal.
    ///
    /// ## Returns
    /// `Result<PersonalAccessToken, GitLabError>` — dados do token solicitado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get(&self, token_id: u64) -> Result<PersonalAccessToken, GitLabError> {
        let path = format!("personal_access_tokens/{}", token_id);
        self.http.get(&path, &[], "personal_access_tokens.get").await
    }

    /// Revoga um token de acesso pessoal.
    ///
    /// ## Params
    /// - `token_id`: ID do token de acesso pessoal a ser revogado.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn revoke(&self, token_id: u64) -> Result<(), GitLabError> {
        let path = format!("personal_access_tokens/{}", token_id);
        self.http.delete(&path, &[], "personal_access_tokens.revoke").await
    }
}
