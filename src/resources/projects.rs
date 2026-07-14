use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use crate::utils::encoding::encode_query_param;
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
        let encoded = encode_query_param(path);
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
        self.http.post(&path, &serde_json::json!({}), "projects.archive").await
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
        self.http.post(&path, &serde_json::json!({}), "projects.unarchive").await
    }

    /// Cria um fork de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `namespace_path`: Caminho do namespace de destino (ex.: "grupo/subgrupo").
    ///   Se `None`, o fork é criado no namespace do usuário autenticado.
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
        namespace_path: Option<&str>,
    ) -> Result<Project, GitLabError> {
        let path = format!("projects/{}/fork", project_id);
        let body = if let Some(ns) = namespace_path {
            serde_json::json!({ "namespace": ns })
        } else {
            serde_json::json!({})
        };
        self.http.post(&path, &body, "projects.fork").await
    }

    /// Faz upload genérico de arquivo para um projeto.
    ///
    /// Envia um arquivo via multipart para `POST /projects/:id/uploads`.
    /// Útil para anexar arquivos a issues, MRs, etc. O retorno inclui
    /// URL relativa e código markdown para referência.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `file_name`: Nome do arquivo (ex.: "relatorio.pdf").
    /// - `data`: Conteúdo do arquivo em bytes.
    ///
    /// ## Returns
    /// `Result<UploadResult, GitLabError>` — dados do upload.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn upload_file(
        &self,
        project_id: u64,
        file_name: &str,
        data: Vec<u8>,
    ) -> Result<UploadResult, GitLabError> {
        let path = format!("projects/{}/uploads", project_id);
        let part = reqwest::multipart::Part::bytes(data).file_name(file_name.to_string());
        let form = reqwest::multipart::Form::new().part("file", part);
        self.http.post_multipart(&path, form, "projects.upload_file").await
    }

    /// Faz upload de avatar para um projeto.
    ///
    /// Envia uma imagem via multipart para `PUT /projects/:id`, definindo
    /// o avatar do projeto. Formatos aceitos: PNG, JPEG, GIF.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `file_name`: Nome do arquivo (ex.: "logo.png").
    /// - `data`: Conteúdo da imagem em bytes.
    ///
    /// ## Returns
    /// `Result<Project, GitLabError>` — dados do projeto atualizado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn upload_avatar(
        &self,
        project_id: u64,
        file_name: &str,
        data: Vec<u8>,
    ) -> Result<Project, GitLabError> {
        let path = format!("projects/{}", project_id);
        let part = reqwest::multipart::Part::bytes(data).file_name(file_name.to_string());
        let form = reqwest::multipart::Form::new().part("avatar", part);
        self.http.put_multipart(&path, form, "projects.upload_avatar").await
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

    /// Marca um projeto como favorito (star).
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    ///
    /// ## Returns
    /// `Result<Project, GitLabError>` — dados do projeto favoritado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn star(&self, project_id: u64) -> Result<Project, GitLabError> {
        let path = format!("projects/{}/star", project_id);
        self.http.post(&path, &serde_json::json!({}), "projects.star").await
    }

    /// Remove o favorito (unstar) de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    ///
    /// ## Returns
    /// `Result<Project, GitLabError>` — dados do projeto desfavoritado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn unstar(&self, project_id: u64) -> Result<Project, GitLabError> {
        let path = format!("projects/{}/unstar", project_id);
        self.http.post(&path, &serde_json::json!({}), "projects.unstar").await
    }

    /// Lista forks de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `filter`: Filtros opcionais para a consulta.
    ///
    /// ## Returns
    /// `Result<Vec<Project>, GitLabError>` — lista de forks do projeto.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list_forks(
        &self,
        project_id: u64,
        filter: Option<&ProjectFilter>,
    ) -> Result<Vec<Project>, GitLabError> {
        let path = format!("projects/{}/forks", project_id);
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "projects.list_forks").await
    }

    /// Obtém as linguagens de programação de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    ///
    /// ## Returns
    /// `Result<serde_json::Value, GitLabError>` — mapa de linguagens e seus percentuais.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn languages(&self, project_id: u64) -> Result<serde_json::Value, GitLabError> {
        let path = format!("projects/{}/languages", project_id);
        self.http.get(&path, &[], "projects.languages").await
    }

    /// Compartilha um projeto com um grupo.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `group_id`: ID do grupo de destino.
    /// - `group_access`: Nível de acesso concedido ao grupo.
    ///
    /// ## Returns
    /// `Result<Project, GitLabError>` — dados do projeto compartilhado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn share(
        &self,
        project_id: u64,
        group_id: u64,
        group_access: u32,
    ) -> Result<Project, GitLabError> {
        let path = format!("projects/{}/share", project_id);
        let body = serde_json::json!({ "group_id": group_id, "group_access": group_access });
        self.http.post(&path, &body, "projects.share").await
    }

    /// Remove o compartilhamento de um projeto com um grupo.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `group_id`: ID do grupo.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn unshare(&self, project_id: u64, group_id: u64) -> Result<(), GitLabError> {
        let path = format!("projects/{}/share/{}", project_id, group_id);
        self.http.delete(&path, &[], "projects.unshare").await
    }
}
