use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use crate::utils::encoding::{encode_query_param, filter_to_query};
use std::sync::Arc;

/// Recurso de API para templates Dockerfile no GitLab.
#[derive(Debug)]
pub struct DockerfileTemplatesResource {
    http: Arc<HttpClient>,
}

impl DockerfileTemplatesResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista todos os templates Dockerfile disponíveis.
    ///
    /// ## Params
    /// - `filter`: Filtro opcional (paginação).
    ///
    /// ## Returns
    /// `Result<Vec<Template>, GitLabError>` — lista de templates.
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
        let path = "templates/dockerfiles".to_string();
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "dockerfile_templates.list").await
    }

    /// Obtém um template Dockerfile pela chave (nome).
    ///
    /// ## Params
    /// - `key`: Nome do template (ex.: "Node").
    ///
    /// ## Returns
    /// `Result<Template, GitLabError>` — dados do template.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    /// Executa a operação .
    /// Executa a operacao `get`.
    pub async fn get(&self, key: &str) -> Result<Template, GitLabError> {
        let path = format!("templates/dockerfiles/{}", encode_query_param(key));
        self.http.get(&path, &[], "dockerfile_templates.get").await
    }
}
