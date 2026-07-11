use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use std::sync::Arc;
use crate::types::*;
use crate::utils::encoding::encode_query_param;

/// Recurso de API para operações com releases no GitLab.
#[derive(Debug)]
pub struct ReleasesResource {
    http: Arc<HttpClient>,
}

impl ReleasesResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista todas as releases de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<Release>, GitLabError>` — lista de releases.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn list(&self, project_id: u64) -> Result<Vec<Release>, GitLabError> {
        let path = format!("projects/{}/releases", project_id);
        self.http.get(&path, &[], "releases.list")
    }

    /// Obtém uma release pelo nome da tag.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `tag_name`: Nome da tag associada à release.
    ///
    /// ## Returns
    /// `Result<Release, GitLabError>` — dados da release solicitada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn get(&self, project_id: u64, tag_name: &str) -> Result<Release, GitLabError> {
        let path = format!("projects/{}/releases/{}", project_id, encode_query_param(tag_name));
        self.http.get(&path, &[], "releases.get")
    }

    /// Cria uma nova release.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `payload`: Dados para criar a release.
    ///
    /// ## Returns
    /// `Result<Release, GitLabError>` — dados da release criada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn create(&self, project_id: u64, payload: &CreateReleasePayload) -> Result<Release, GitLabError> {
        let path = format!("projects/{}/releases", project_id);
        self.http.post(&path, &payload, "releases.create")
    }

    /// Atualiza uma release existente.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `tag_name`: Nome da tag associada à release.
    /// - `payload`: Dados para atualizar a release.
    ///
    /// ## Returns
    /// `Result<Release, GitLabError>` — dados da release atualizada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn update(&self, project_id: u64, tag_name: &str, payload: &UpdateReleasePayload) -> Result<Release, GitLabError> {
        let path = format!("projects/{}/releases/{}", project_id, encode_query_param(tag_name));
        self.http.put(&path, &payload, "releases.update")
    }

    /// Remove uma release.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `tag_name`: Nome da tag associada à release.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn delete(&self, project_id: u64, tag_name: &str) -> Result<(), GitLabError> {
        let path = format!("projects/{}/releases/{}", project_id, encode_query_param(tag_name));
        self.http.delete(&path, &[], "releases.delete")
    }

    /// Cria um link de asset em uma release.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `tag_name`: Nome da tag associada à release.
    /// - `payload`: Dados para criar o link.
    ///
    /// ## Returns
    /// `Result<ReleaseLinkItem, GitLabError>` — dados do link criado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn create_link(&self, project_id: u64, tag_name: &str, payload: &CreateReleaseLinkPayload) -> Result<ReleaseLinkItem, GitLabError> {
        let path = format!("projects/{}/releases/{}/assets/links", project_id, encode_query_param(tag_name));
        self.http.post(&path, &payload, "releases.create_link")
    }

    /// Remove um link de asset de uma release.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `tag_name`: Nome da tag associada à release.
    /// - `link_id`: ID do link no GitLab.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn delete_link(&self, project_id: u64, tag_name: &str, link_id: u64) -> Result<(), GitLabError> {
        let path = format!("projects/{}/releases/{}/assets/links/{}", project_id, encode_query_param(tag_name), link_id);
        self.http.delete(&path, &[], "releases.delete_link")
    }
}
