use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use std::sync::Arc;

/// Recurso de API para operações com pacotes no GitLab.
#[derive(Debug)]
pub struct PackagesResource {
    http: Arc<HttpClient>,
}

impl PackagesResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista todos os pacotes de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<Package>, GitLabError>` — lista de pacotes.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list(&self, project_id: u64) -> Result<Vec<Package>, GitLabError> {
        let path = format!("projects/{}/packages", project_id);
        self.http.get(&path, &[], "packages.list").await
    }

    /// Obtém um pacote pelo ID.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `package_id`: ID do pacote no GitLab.
    ///
    /// ## Returns
    /// `Result<Package, GitLabError>` — dados do pacote solicitado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get(&self, project_id: u64, package_id: u64) -> Result<Package, GitLabError> {
        let path = format!("projects/{}/packages/{}", project_id, package_id);
        self.http.get(&path, &[], "packages.get").await
    }

    /// Remove um pacote.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `package_id`: ID do pacote no GitLab.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn delete(&self, project_id: u64, package_id: u64) -> Result<(), GitLabError> {
        let path = format!("projects/{}/packages/{}", project_id, package_id);
        self.http.delete(&path, &[], "packages.delete").await
    }

    /// Lista os arquivos de um pacote.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `package_id`: ID do pacote no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<serde_json::Value>, GitLabError>` — lista de arquivos do pacote.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list_files(
        &self,
        project_id: u64,
        package_id: u64,
    ) -> Result<Vec<serde_json::Value>, GitLabError> {
        let path = format!("projects/{}/packages/{}/package_files", project_id, package_id);
        self.http.get(&path, &[], "packages.list_files").await
    }
}
