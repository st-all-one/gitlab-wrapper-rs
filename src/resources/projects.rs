use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use crate::utils::encoding::filter_to_query;
use std::sync::Arc;

/// Recurso de API para operações com projetos no GitLab.
#[derive(Debug)]
pub struct ProjectsResource {
    http: Arc<HttpClient>,
}

impl ProjectsResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista projetos com filtros opcionais.
    ///
    /// ## Params
    /// - `filter`: Filtros opcionais para a consulta.
    ///
    /// ## Returns
    /// `Result<Vec<Project>, GitLabError>` — lista de projetos.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list(&self, filter: Option<&ProjectFilter>) -> Result<Vec<Project>, GitLabError> {
        let query = filter_to_query(filter);
        self.http.get("projects", &query, "projects.list").await
    }

    /// Lista todos os projetos (paginação automática).
    ///
    /// ## Params
    /// - `filter`: Filtros opcionais para a consulta.
    ///
    /// ## Returns
    /// `Result<Vec<Project>, GitLabError>` — lista completa de projetos.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list_all(
        &self,
        filter: Option<&ProjectFilter>,
    ) -> Result<Vec<Project>, GitLabError> {
        let query = filter_to_query(filter);
        self.http.paginate_all("projects", &query, "projects.list_all").await
    }

    /// Obtém um projeto pelo ID.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    ///
    /// ## Returns
    /// `Result<Project, GitLabError>` — dados do projeto solicitado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get(&self, project_id: u64) -> Result<Project, GitLabError> {
        let path = format!("projects/{}", project_id);
        self.http.get(&path, &[], "projects.get").await
    }

    /// Obtém um projeto pelo caminho URL-encoded.
    ///
    /// ## Params
    /// - `path`: Caminho do projeto (ex: "namespace/project").
    ///
    /// ## Returns
    /// `Result<Project, GitLabError>` — dados do projeto solicitado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get_by_path(&self, path: &str) -> Result<Project, GitLabError> {
        let encoded = crate::utils::encoding::encode_query_param(path);
        let url = format!("projects/{}", encoded);
        self.http.get(&url, &[], "projects.get_by_path").await
    }

    /// Cria um novo projeto.
    ///
    /// ## Params
    /// - `payload`: Dados para criar o projeto.
    ///
    /// ## Returns
    /// `Result<Project, GitLabError>` — dados do projeto criado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn create(&self, payload: &CreateProjectPayload) -> Result<Project, GitLabError> {
        self.http.post("projects", &payload, "projects.create").await
    }

    /// Atualiza um projeto existente.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `payload`: Dados para atualizar o projeto.
    ///
    /// ## Returns
    /// `Result<Project, GitLabError>` — dados do projeto atualizado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn update(
        &self,
        project_id: u64,
        payload: &UpdateProjectPayload,
    ) -> Result<Project, GitLabError> {
        let path = format!("projects/{}", project_id);
        self.http.put(&path, &payload, "projects.update").await
    }

    /// Remove um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn delete(&self, project_id: u64) -> Result<(), GitLabError> {
        let path = format!("projects/{}", project_id);
        self.http.delete(&path, &[], "projects.delete").await
    }

    /// Arquiva um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    ///
    /// ## Returns
    /// `Result<Project, GitLabError>` — dados do projeto arquivado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn archive(&self, project_id: u64) -> Result<Project, GitLabError> {
        let path = format!("projects/{}/archive", project_id);
        self.http.post(&path, &serde_json::Value::Null, "projects.archive").await
    }

    /// Desarquiva um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    ///
    /// ## Returns
    /// `Result<Project, GitLabError>` — dados do projeto desarquivado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn unarchive(&self, project_id: u64) -> Result<Project, GitLabError> {
        let path = format!("projects/{}/unarchive", project_id);
        self.http.post(&path, &serde_json::Value::Null, "projects.unarchive").await
    }

    /// Cria um fork de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `_namespace`: Namespace opcional para o fork.
    ///
    /// ## Returns
    /// `Result<Project, GitLabError>` — dados do projeto forkado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn fork(
        &self,
        project_id: u64,
        _namespace: Option<&str>,
    ) -> Result<Project, GitLabError> {
        let path = format!("projects/{}/fork", project_id);
        self.http.post(&path, &serde_json::Value::Null, "projects.fork").await
    }

    /// Faz upload de avatar para um projeto.
    ///
    /// ## Params
    /// - `_project_id`: ID do projeto no GitLab.
    /// - `_file_path`: Caminho do arquivo de avatar.
    ///
    /// ## Returns
    /// `Result<Project, GitLabError>` — erro, pois upload de avatar requer multipart.
    ///
    /// ## Errors
    /// Retorna `GitLabError::Config` informando que multipart não é suportado.
    pub async fn upload_avatar(
        &self,
        _project_id: u64,
        _file_path: &str,
    ) -> Result<Project, GitLabError> {
        Err(GitLabError::Config(
            "Avatar upload requires multipart - not supported via the HTTP client".into(),
        ))
    }

    /// Transfere um projeto para outro namespace.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `namespace_id`: ID do namespace de destino.
    ///
    /// ## Returns
    /// `Result<Project, GitLabError>` — dados do projeto transferido.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn transfer(
        &self,
        project_id: u64,
        namespace_id: u64,
    ) -> Result<Project, GitLabError> {
        let path = format!("projects/{}/transfer", project_id);
        let body = serde_json::json!({ "namespace_id": namespace_id });
        self.http.put(&path, &body, "projects.transfer").await
    }
}
