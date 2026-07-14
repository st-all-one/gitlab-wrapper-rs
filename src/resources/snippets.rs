use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use crate::utils::encoding::filter_to_query;
use std::sync::Arc;

/// Recurso de API para operações com snippets de projeto no GitLab.
#[derive(Debug)]
pub struct SnippetsResource {
    http: Arc<HttpClient>,
}

impl SnippetsResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista snippets de um projeto com filtros opcionais.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `filter`: Filtros opcionais para a consulta.
    ///
    /// ## Returns
    /// `Result<Vec<Snippet>, GitLabError>` — lista de snippets.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list(
        &self,
        project_id: u64,
        filter: Option<&SnippetFilter>,
    ) -> Result<Vec<Snippet>, GitLabError> {
        let path = format!("projects/{}/snippets", project_id);
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "snippets.list").await
    }

    /// Obtém um snippet de projeto pelo ID.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `snippet_id`: ID do snippet no GitLab.
    ///
    /// ## Returns
    /// `Result<Snippet, GitLabError>` — dados do snippet solicitado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get(&self, project_id: u64, snippet_id: u64) -> Result<Snippet, GitLabError> {
        let path = format!("projects/{}/snippets/{}", project_id, snippet_id);
        self.http.get(&path, &[], "snippets.get").await
    }

    /// Cria um novo snippet em um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `payload`: Dados para criar o snippet.
    ///
    /// ## Returns
    /// `Result<Snippet, GitLabError>` — dados do snippet criado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn create(
        &self,
        project_id: u64,
        payload: &CreateSnippetPayload,
    ) -> Result<Snippet, GitLabError> {
        let path = format!("projects/{}/snippets", project_id);
        self.http.post(&path, &payload, "snippets.create").await
    }

    /// Atualiza um snippet existente.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `snippet_id`: ID do snippet no GitLab.
    /// - `payload`: Dados para atualizar o snippet.
    ///
    /// ## Returns
    /// `Result<Snippet, GitLabError>` — dados do snippet atualizado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn update(
        &self,
        project_id: u64,
        snippet_id: u64,
        payload: &UpdateSnippetPayload,
    ) -> Result<Snippet, GitLabError> {
        let path = format!("projects/{}/snippets/{}", project_id, snippet_id);
        self.http.put(&path, &payload, "snippets.update").await
    }

    /// Remove um snippet.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `snippet_id`: ID do snippet no GitLab.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn delete(&self, project_id: u64, snippet_id: u64) -> Result<(), GitLabError> {
        let path = format!("projects/{}/snippets/{}", project_id, snippet_id);
        self.http.delete(&path, &[], "snippets.delete").await
    }

    /// Obtém o conteúdo bruto de um snippet.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `snippet_id`: ID do snippet no GitLab.
    ///
    /// ## Returns
    /// `Result<serde_json::Value, GitLabError>` — conteúdo bruto do snippet.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn raw(
        &self,
        project_id: u64,
        snippet_id: u64,
    ) -> Result<serde_json::Value, GitLabError> {
        let path = format!("projects/{}/snippets/{}/raw", project_id, snippet_id);
        self.http.get(&path, &[], "snippets.raw").await
    }
}
