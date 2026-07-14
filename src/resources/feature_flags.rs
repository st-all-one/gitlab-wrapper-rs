use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use crate::utils::encoding::{encode_query_param, filter_to_query};
use std::sync::Arc;

/// Recurso de API para operações com feature flags no GitLab.
#[derive(Debug)]
pub struct FeatureFlagsResource {
    http: Arc<HttpClient>,
}

impl FeatureFlagsResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista todas as feature flags de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `filter`: Filtro opcional (paginação, escopo).
    ///
    /// ## Returns
    /// `Result<Vec<FeatureFlag>, GitLabError>` — lista de feature flags.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list(
        &self,
        project_id: u64,
        filter: Option<&FeatureFlagFilter>,
    ) -> Result<Vec<FeatureFlag>, GitLabError> {
        let path = format!("projects/{}/feature_flags", project_id);
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "feature_flags.list").await
    }

    /// Obtém uma feature flag pela chave (nome).
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `key`: Chave (nome) da feature flag.
    ///
    /// ## Returns
    /// `Result<FeatureFlag, GitLabError>` — dados da feature flag.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get(&self, project_id: u64, key: &str) -> Result<FeatureFlag, GitLabError> {
        let path = format!("projects/{}/feature_flags/{}", project_id, encode_query_param(key));
        self.http.get(&path, &[], "feature_flags.get").await
    }

    /// Cria uma nova feature flag.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `payload`: Dados da feature flag a criar.
    ///
    /// ## Returns
    /// `Result<FeatureFlag, GitLabError>` — dados da feature flag criada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn create(
        &self,
        project_id: u64,
        payload: &CreateFeatureFlagPayload,
    ) -> Result<FeatureFlag, GitLabError> {
        let path = format!("projects/{}/feature_flags", project_id);
        self.http.post(&path, payload, "feature_flags.create").await
    }

    /// Atualiza uma feature flag existente.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `key`: Chave (nome) da feature flag.
    /// - `payload`: Dados da feature flag a atualizar.
    ///
    /// ## Returns
    /// `Result<FeatureFlag, GitLabError>` — dados da feature flag atualizada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn update(
        &self,
        project_id: u64,
        key: &str,
        payload: &UpdateFeatureFlagPayload,
    ) -> Result<FeatureFlag, GitLabError> {
        let path = format!("projects/{}/feature_flags/{}", project_id, encode_query_param(key));
        self.http.put(&path, payload, "feature_flags.update").await
    }

    /// Remove uma feature flag.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `key`: Chave (nome) da feature flag.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn delete(&self, project_id: u64, key: &str) -> Result<(), GitLabError> {
        let path = format!("projects/{}/feature_flags/{}", project_id, encode_query_param(key));
        self.http.delete(&path, &[], "feature_flags.delete").await
    }
}
