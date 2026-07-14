use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use crate::utils::encoding::encode_query_param;
use std::sync::Arc;

/// Recurso de API para operações com o Container Registry no GitLab.
#[derive(Debug)]
pub struct ContainerRegistryResource {
    http: Arc<HttpClient>,
}

impl ContainerRegistryResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista todos os repositórios do Container Registry de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<ContainerRepository>, GitLabError>` — lista de repositórios.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list_repositories(
        &self,
        project_id: u64,
    ) -> Result<Vec<ContainerRepository>, GitLabError> {
        let path = format!("projects/{}/registry/repositories", project_id);
        self.http.get(&path, &[], "container_registry.list_repositories").await
    }

    /// Remove um repositório do Container Registry.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `repo_id`: ID do repositório no Container Registry.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn delete_repository(
        &self,
        project_id: u64,
        repo_id: u64,
    ) -> Result<(), GitLabError> {
        let path = format!("projects/{}/registry/repositories/{}", project_id, repo_id);
        self.http.delete(&path, &[], "container_registry.delete_repository").await
    }

    /// Lista todas as tags de um repositório do Container Registry.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `repo_id`: ID do repositório no Container Registry.
    ///
    /// ## Returns
    /// `Result<Vec<ContainerTag>, GitLabError>` — lista de tags.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list_tags(
        &self,
        project_id: u64,
        repo_id: u64,
    ) -> Result<Vec<ContainerTag>, GitLabError> {
        let path = format!("projects/{}/registry/repositories/{}/tags", project_id, repo_id);
        self.http.get(&path, &[], "container_registry.list_tags").await
    }

    /// Obtém uma tag específica de um repositório do Container Registry.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `repo_id`: ID do repositório no Container Registry.
    /// - `tag_name`: Nome da tag (pode conter slashes, deve ser URL-encoded).
    ///
    /// ## Returns
    /// `Result<ContainerTag, GitLabError>` — dados da tag.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get_tag(
        &self,
        project_id: u64,
        repo_id: u64,
        tag_name: &str,
    ) -> Result<ContainerTag, GitLabError> {
        let path = format!(
            "projects/{}/registry/repositories/{}/tags/{}",
            project_id,
            repo_id,
            encode_query_param(tag_name)
        );
        self.http.get(&path, &[], "container_registry.get_tag").await
    }

    /// Remove uma tag de um repositório do Container Registry.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `repo_id`: ID do repositório no Container Registry.
    /// - `tag_name`: Nome da tag a ser removida.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn delete_tag(
        &self,
        project_id: u64,
        repo_id: u64,
        tag_name: &str,
    ) -> Result<(), GitLabError> {
        let path = format!(
            "projects/{}/registry/repositories/{}/tags/{}",
            project_id,
            repo_id,
            encode_query_param(tag_name)
        );
        self.http.delete(&path, &[], "container_registry.delete_tag").await
    }
}
