use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use crate::utils::encoding::encode_query_param;
use std::sync::Arc;

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
    pub async fn list(&self, project_id: u64) -> Result<Vec<Release>, GitLabError> {
        let path = format!("projects/{}/releases", project_id);
        self.http.get(&path, &[], "releases.list").await
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
    pub async fn get(&self, project_id: u64, tag_name: &str) -> Result<Release, GitLabError> {
        let path = format!("projects/{}/releases/{}", project_id, encode_query_param(tag_name));
        self.http.get(&path, &[], "releases.get").await
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
    pub async fn create(
        &self,
        project_id: u64,
        payload: &CreateReleasePayload,
    ) -> Result<Release, GitLabError> {
        let path = format!("projects/{}/releases", project_id);
        self.http.post(&path, &payload, "releases.create").await
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
    pub async fn update(
        &self,
        project_id: u64,
        tag_name: &str,
        payload: &UpdateReleasePayload,
    ) -> Result<Release, GitLabError> {
        let path = format!("projects/{}/releases/{}", project_id, encode_query_param(tag_name));
        self.http.put(&path, &payload, "releases.update").await
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
    pub async fn delete(&self, project_id: u64, tag_name: &str) -> Result<(), GitLabError> {
        let path = format!("projects/{}/releases/{}", project_id, encode_query_param(tag_name));
        self.http.delete(&path, &[], "releases.delete").await
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
    pub async fn create_link(
        &self,
        project_id: u64,
        tag_name: &str,
        payload: &CreateReleaseLinkPayload,
    ) -> Result<ReleaseLinkItem, GitLabError> {
        let path = format!(
            "projects/{}/releases/{}/assets/links",
            project_id,
            encode_query_param(tag_name)
        );
        self.http.post(&path, &payload, "releases.create_link").await
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
    pub async fn delete_link(
        &self,
        project_id: u64,
        tag_name: &str,
        link_id: u64,
    ) -> Result<(), GitLabError> {
        let path = format!(
            "projects/{}/releases/{}/assets/links/{}",
            project_id,
            encode_query_param(tag_name),
            link_id
        );
        self.http.delete(&path, &[], "releases.delete_link").await
    }

    /// Lista todos os links de assets de uma release.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `tag_name`: Nome da tag associada à release.
    ///
    /// ## Returns
    /// `Result<Vec<ReleaseLinkItem>, GitLabError>` — lista de links da release.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list_links(
        &self,
        project_id: u64,
        tag_name: &str,
    ) -> Result<Vec<ReleaseLinkItem>, GitLabError> {
        let path = format!(
            "projects/{}/releases/{}/assets/links",
            project_id,
            encode_query_param(tag_name)
        );
        self.http.get(&path, &[], "releases.list_links").await
    }

    /// Obtém um link específico de uma release.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `tag_name`: Nome da tag associada à release.
    /// - `link_id`: ID do link no GitLab.
    ///
    /// ## Returns
    /// `Result<ReleaseLinkItem, GitLabError>` — dados do link solicitado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get_link(
        &self,
        project_id: u64,
        tag_name: &str,
        link_id: u64,
    ) -> Result<ReleaseLinkItem, GitLabError> {
        let path = format!(
            "projects/{}/releases/{}/assets/links/{}",
            project_id,
            encode_query_param(tag_name),
            link_id
        );
        self.http.get(&path, &[], "releases.get_link").await
    }

    /// Atualiza um link de asset de uma release.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `tag_name`: Nome da tag associada à release.
    /// - `link_id`: ID do link no GitLab.
    /// - `name`: Novo nome do link (opcional).
    /// - `url`: Nova URL do link (opcional).
    ///
    /// ## Returns
    /// `Result<ReleaseLinkItem, GitLabError>` — dados do link atualizado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn update_link(
        &self,
        project_id: u64,
        tag_name: &str,
        link_id: u64,
        name: Option<&str>,
        url: Option<&str>,
    ) -> Result<ReleaseLinkItem, GitLabError> {
        let path = format!(
            "projects/{}/releases/{}/assets/links/{}",
            project_id,
            encode_query_param(tag_name),
            link_id
        );
        let mut body = serde_json::json!({});
        if let Some(n) = name {
            body["name"] = serde_json::json!(n);
        }
        if let Some(u) = url {
            body["url"] = serde_json::json!(u);
        }
        self.http.put(&path, &body, "releases.update_link").await
    }

    /// Obtém a release mais recente de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    ///
    /// ## Returns
    /// `Result<Release, GitLabError>` — dados da release mais recente.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get_latest(&self, project_id: u64) -> Result<Release, GitLabError> {
        let path = format!("projects/{}/releases/permalink/latest", project_id);
        self.http.get(&path, &[], "releases.get_latest").await
    }

    /// Faz download de um asset de uma release.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `tag_name`: Nome da tag associada à release.
    /// - `asset_path`: Caminho do asset dentro da release.
    ///
    /// ## Returns
    /// `Result<serde_json::Value, GitLabError>` — resposta do endpoint de download.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn download_asset(
        &self,
        project_id: u64,
        tag_name: &str,
        asset_path: &str,
    ) -> Result<serde_json::Value, GitLabError> {
        let path = format!(
            "projects/{}/releases/{}/downloads/{}",
            project_id,
            encode_query_param(tag_name),
            asset_path
        );
        self.http.get(&path, &[], "releases.download_asset").await
    }
}
