use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use crate::utils::encoding::encode_query_param;
use std::sync::Arc;

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
    pub async fn list(&self, project_id: u64) -> Result<Vec<WikiPage>, GitLabError> {
        let path = format!("projects/{}/wikis", project_id);
        self.http.get(&path, &[], "wikis.list").await
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
    pub async fn get(&self, project_id: u64, slug: &str) -> Result<WikiPage, GitLabError> {
        let path = format!("projects/{}/wikis/{}", project_id, encode_query_param(slug));
        self.http.get(&path, &[], "wikis.get").await
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
    pub async fn create(
        &self,
        project_id: u64,
        payload: &CreateWikiPagePayload,
    ) -> Result<WikiPage, GitLabError> {
        let path = format!("projects/{}/wikis", project_id);
        self.http.post(&path, &payload, "wikis.create").await
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
    pub async fn update(
        &self,
        project_id: u64,
        slug: &str,
        payload: &UpdateWikiPagePayload,
    ) -> Result<WikiPage, GitLabError> {
        let path = format!("projects/{}/wikis/{}", project_id, encode_query_param(slug));
        self.http.put(&path, &payload, "wikis.update").await
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
    pub async fn delete(&self, project_id: u64, slug: &str) -> Result<(), GitLabError> {
        let path = format!("projects/{}/wikis/{}", project_id, encode_query_param(slug));
        self.http.delete(&path, &[], "wikis.delete").await
    }

    /// Faz upload de um anexo para a wiki.
    ///
    /// Envia um arquivo via multipart para `POST /projects/:id/wikis/attachments`.
    /// O anexo fica disponível para referência em páginas da wiki.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `file_name`: Nome do arquivo (ex.: "diagrama.png").
    /// - `data`: Conteúdo do arquivo em bytes.
    ///
    /// ## Returns
    /// `Result<WikiAttachmentResult, GitLabError>` — dados do anexo carregado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn upload_attachment(
        &self,
        project_id: u64,
        file_name: &str,
        data: Vec<u8>,
    ) -> Result<WikiAttachmentResult, GitLabError> {
        let path = format!("projects/{}/wikis/attachments", project_id);
        let part = reqwest::multipart::Part::bytes(data)
            .file_name(file_name.to_string());
        let form = reqwest::multipart::Form::new().part("file", part);
        self.http.post_multipart(&path, form, "wikis.upload_attachment").await
    }
}
