use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use std::sync::Arc;
use crate::types::*;
use crate::utils::encoding::encode_query_param;

/// Recurso de API para operações com wikis no GitLab.
#[derive(Debug)]
pub struct WikisResource {
    http: Arc<HttpClient>,
}

impl WikisResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista páginas de wiki de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<WikiPage>, GitLabError>` — lista de páginas da wiki.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn list(&self, project_id: u64) -> Result<Vec<WikiPage>, GitLabError> {
        let path = format!("projects/{}/wikis", project_id);
        self.http.get(&path, &[], "wikis.list")
    }

    /// Obtém uma página de wiki pelo slug.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `slug`: Slug da página da wiki.
    ///
    /// ## Returns
    /// `Result<WikiPage, GitLabError>` — dados da página da wiki.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn get(&self, project_id: u64, slug: &str) -> Result<WikiPage, GitLabError> {
        let path = format!("projects/{}/wikis/{}", project_id, encode_query_param(slug));
        self.http.get(&path, &[], "wikis.get")
    }

    /// Cria uma nova página de wiki.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `payload`: Dados para criar a página da wiki.
    ///
    /// ## Returns
    /// `Result<WikiPage, GitLabError>` — dados da página criada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn create(&self, project_id: u64, payload: &CreateWikiPagePayload) -> Result<WikiPage, GitLabError> {
        let path = format!("projects/{}/wikis", project_id);
        self.http.post(&path, &payload, "wikis.create")
    }

    /// Atualiza uma página de wiki existente.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `slug`: Slug da página da wiki.
    /// - `payload`: Dados para atualizar a página.
    ///
    /// ## Returns
    /// `Result<WikiPage, GitLabError>` — dados da página atualizada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn update(&self, project_id: u64, slug: &str, payload: &UpdateWikiPagePayload) -> Result<WikiPage, GitLabError> {
        let path = format!("projects/{}/wikis/{}", project_id, encode_query_param(slug));
        self.http.put(&path, &payload, "wikis.update")
    }

    /// Remove uma página de wiki.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `slug`: Slug da página a ser removida.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn delete(&self, project_id: u64, slug: &str) -> Result<(), GitLabError> {
        let path = format!("projects/{}/wikis/{}", project_id, encode_query_param(slug));
        self.http.delete(&path, &[], "wikis.delete")
    }

    /// Faz upload de um anexo para a wiki.
    ///
    /// ## Params
    /// - `_project_id`: ID do projeto no GitLab.
    /// - `_file_path`: Caminho do arquivo a ser enviado.
    ///
    /// ## Returns
    /// `Result<serde_json::Value, GitLabError>` — erro, pois upload de anexo requer multipart.
    ///
    /// ## Errors
    /// Retorna `GitLabError::Config` informando que multipart não é suportado.
    pub fn upload_attachment(&self, _project_id: u64, _file_path: &str) -> Result<serde_json::Value, GitLabError> {
        Err(GitLabError::Config("Wiki attachment upload requires multipart - not supported via blocking HTTP client".into()))
    }
}
