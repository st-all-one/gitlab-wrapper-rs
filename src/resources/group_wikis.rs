use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use crate::utils::encoding::{encode_query_param, filter_to_query};
use std::sync::Arc;

/// Recurso de API para operações com wikis de grupo no GitLab.
#[derive(Debug)]
pub struct GroupWikisResource {
    http: Arc<HttpClient>,
}

impl GroupWikisResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista páginas de wiki de um grupo.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    /// - `filter`: Filtros opcionais para a consulta.
    ///
    /// ## Returns
    /// `Result<Vec<WikiPage>, GitLabError>` — lista de páginas da wiki.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list(
        &self,
        group_id: u64,
        filter: Option<&WikiFilter>,
    ) -> Result<Vec<WikiPage>, GitLabError> {
        let path = format!("groups/{}/wikis", group_id);
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "group_wikis.list").await
    }

    /// Obtém uma página de wiki pelo slug.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    /// - `slug`: Slug da página da wiki.
    ///
    /// ## Returns
    /// `Result<WikiPage, GitLabError>` — dados da página da wiki.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get(&self, group_id: u64, slug: &str) -> Result<WikiPage, GitLabError> {
        let path = format!("groups/{}/wikis/{}", group_id, encode_query_param(slug));
        self.http.get(&path, &[], "group_wikis.get").await
    }

    /// Cria uma nova página de wiki.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
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
        group_id: u64,
        payload: &CreateWikiPagePayload,
    ) -> Result<WikiPage, GitLabError> {
        let path = format!("groups/{}/wikis", group_id);
        self.http.post(&path, &payload, "group_wikis.create").await
    }

    /// Atualiza uma página de wiki existente.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
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
        group_id: u64,
        slug: &str,
        payload: &UpdateWikiPagePayload,
    ) -> Result<WikiPage, GitLabError> {
        let path = format!("groups/{}/wikis/{}", group_id, encode_query_param(slug));
        self.http.put(&path, &payload, "group_wikis.update").await
    }

    /// Remove uma página de wiki.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    /// - `slug`: Slug da página a ser removida.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn delete(&self, group_id: u64, slug: &str) -> Result<(), GitLabError> {
        let path = format!("groups/{}/wikis/{}", group_id, encode_query_param(slug));
        self.http.delete(&path, &[], "group_wikis.delete").await
    }

    /// Faz upload de um anexo para a wiki do grupo.
    ///
    /// Envia um arquivo via multipart para `POST /groups/:id/wikis/attachments`.
    /// O anexo fica disponível para referência em páginas da wiki.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
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
        group_id: u64,
        file_name: &str,
        data: Vec<u8>,
    ) -> Result<WikiAttachmentResult, GitLabError> {
        let path = format!("groups/{}/wikis/attachments", group_id);
        let part = reqwest::multipart::Part::bytes(data).file_name(file_name.to_string());
        let form = reqwest::multipart::Form::new().part("file", part);
        self.http.post_multipart(&path, form, "group_wikis.upload_attachment").await
    }
}
