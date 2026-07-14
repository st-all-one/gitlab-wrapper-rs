use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use crate::utils::encoding::{encode_query_param, filter_to_query};
use std::sync::Arc;

/// Recurso de API para templates de licença open source no GitLab.
#[derive(Debug)]
pub struct LicenseTemplatesResource {
    http: Arc<HttpClient>,
}

impl LicenseTemplatesResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista todas as licenças template disponíveis.
    ///
    /// ## Params
    /// - `filter`: Filtro opcional (paginação).
    ///
    /// ## Returns
    /// `Result<Vec<Template>, GitLabError>` — lista de templates de licença.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    /// Executa a operação .
    /// Executa a operacao `list`.
    pub async fn list(
        &self,
        filter: Option<&TemplateFilter>,
    ) -> Result<Vec<Template>, GitLabError> {
        let path = "templates/licenses".to_string();
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "license_templates.list").await
    }

    /// Obtém uma licença template pela chave (nome).
    ///
    /// ## Params
    /// - `key`: Nome da licença (ex.: "mit").
    ///
    /// ## Returns
    /// `Result<Template, GitLabError>` — dados da licença.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    /// Executa a operação .
    /// Executa a operacao `get`.
    pub async fn get(&self, key: &str) -> Result<Template, GitLabError> {
        let path = format!("templates/licenses/{}", encode_query_param(key));
        self.http.get(&path, &[], "license_templates.get").await
    }
}
