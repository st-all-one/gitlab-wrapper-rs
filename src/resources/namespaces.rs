use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::{Namespace, NamespaceFilter};
use crate::utils::encoding::filter_to_query;
use std::sync::Arc;

/// Recurso de API para namespaces no GitLab.
#[derive(Debug)]
pub struct NamespacesResource {
    http: Arc<HttpClient>,
}

impl NamespacesResource {
    pub(crate) const fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Executa a operação .
    /// Executa a operacao `list`.
    ///
    /// ## Errors
    /// Retorna `GitLabError::Api` se o filtro for inválido ou `GitLabError::Http`
    /// em erro de transporte da listagem.
    pub async fn list(
        &self,
        filter: Option<&NamespaceFilter>,
    ) -> Result<Vec<Namespace>, GitLabError> {
        let query = filter_to_query(filter);
        self.http.get("namespaces", &query, "namespaces.list").await
    }

    /// Executa a operação .
    /// Executa a operacao `get`.
    ///
    /// ## Errors
    /// Retorna `GitLabError::Api` se o namespace não existir ou `GitLabError::Http`
    /// em erro de transporte da requisição.
    pub async fn get(&self, namespace_id: u64) -> Result<Namespace, GitLabError> {
        let path = format!("namespaces/{namespace_id}");
        self.http.get(&path, &[], "namespaces.get").await
    }
}
