use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use crate::utils::encoding::encode_query_param;
use std::sync::Arc;

/// Recurso de API para operações com tags no GitLab.
#[derive(Debug)]
pub struct TagsResource {
    http: Arc<HttpClient>,
}

impl TagsResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista tags de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<Tag>, GitLabError>` — lista de tags do projeto.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list(&self, project_id: u64) -> Result<Vec<Tag>, GitLabError> {
        let path = format!("projects/{}/repository/tags", project_id);
        self.http.get(&path, &[], "tags.list").await
    }

    /// Obtém uma tag pelo nome.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `tag`: Nome da tag.
    ///
    /// ## Returns
    /// `Result<Tag, GitLabError>` — dados da tag solicitada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get(&self, project_id: u64, tag: &str) -> Result<Tag, GitLabError> {
        let path = format!("projects/{}/repository/tags/{}", project_id, encode_query_param(tag));
        self.http.get(&path, &[], "tags.get").await
    }

    /// Cria uma nova tag em um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `payload`: Dados para criar a tag.
    ///
    /// ## Returns
    /// `Result<Tag, GitLabError>` — dados da tag criada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn create(
        &self,
        project_id: u64,
        payload: &CreateTagPayload,
    ) -> Result<Tag, GitLabError> {
        let path = format!("projects/{}/repository/tags", project_id);
        self.http.post(&path, &payload, "tags.create").await
    }

    /// Remove uma tag de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `tag`: Nome da tag a ser removida.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn delete(&self, project_id: u64, tag: &str) -> Result<(), GitLabError> {
        let path = format!("projects/{}/repository/tags/{}", project_id, encode_query_param(tag));
        self.http.delete(&path, &[], "tags.delete").await
    }
}
