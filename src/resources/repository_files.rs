use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use std::sync::Arc;
use crate::types::*;
use crate::utils::encoding::encode_query_param;

/// Recurso de API para operações com arquivos de repositório no GitLab.
#[derive(Debug)]
pub struct RepositoryFilesResource {
    http: Arc<HttpClient>,
}

impl RepositoryFilesResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Obtém um arquivo do repositório.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `file_path`: Caminho do arquivo no repositório.
    /// - `ref_`: Nome da branch, tag ou SHA do commit.
    ///
    /// ## Returns
    /// `Result<RepositoryFile, GitLabError>` — dados do arquivo.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn get(&self, project_id: u64, file_path: &str, ref_: &str) -> Result<RepositoryFile, GitLabError> {
        let path = format!("projects/{}/repository/files/{}", project_id, encode_query_param(file_path));
        let query = vec![("ref".to_string(), ref_.to_string())];
        self.http.get(&path, &query, "repository_files.get")
    }

    /// Obtém o conteúdo bruto (raw) de um arquivo.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `file_path`: Caminho do arquivo no repositório.
    /// - `ref_`: Nome da branch, tag ou SHA do commit.
    ///
    /// ## Returns
    /// `Result<String, GitLabError>` — conteúdo bruto do arquivo.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn raw(&self, project_id: u64, file_path: &str, ref_: &str) -> Result<String, GitLabError> {
        let path = format!("projects/{}/repository/files/{}/raw", project_id, encode_query_param(file_path));
        let query = vec![("ref".to_string(), ref_.to_string())];
        self.http.get_raw_text(&path, &query, "repository_files.raw")
    }

    /// Obtém informações de blame de um arquivo.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `file_path`: Caminho do arquivo no repositório.
    /// - `ref_`: Nome da branch, tag ou SHA do commit.
    ///
    /// ## Returns
    /// `Result<serde_json::Value, GitLabError>` — dados de blame do arquivo.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn blame(&self, project_id: u64, file_path: &str, ref_: &str) -> Result<serde_json::Value, GitLabError> {
        let path = format!("projects/{}/repository/files/{}/blame", project_id, encode_query_param(file_path));
        let query = vec![("ref".to_string(), ref_.to_string())];
        self.http.get(&path, &query, "repository_files.blame")
    }

    /// Cria um novo arquivo no repositório.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `file_path`: Caminho do arquivo a ser criado.
    /// - `payload`: Dados para criar o arquivo.
    ///
    /// ## Returns
    /// `Result<RepositoryFile, GitLabError>` — dados do arquivo criado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn create(&self, project_id: u64, file_path: &str, payload: &CreateFilePayload) -> Result<RepositoryFile, GitLabError> {
        let path = format!("projects/{}/repository/files/{}", project_id, encode_query_param(file_path));
        self.http.post(&path, &payload, "repository_files.create")
    }

    /// Atualiza um arquivo existente no repositório.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `file_path`: Caminho do arquivo a ser atualizado.
    /// - `payload`: Dados para atualizar o arquivo.
    ///
    /// ## Returns
    /// `Result<RepositoryFile, GitLabError>` — dados do arquivo atualizado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn update(&self, project_id: u64, file_path: &str, payload: &UpdateFilePayload) -> Result<RepositoryFile, GitLabError> {
        let path = format!("projects/{}/repository/files/{}", project_id, encode_query_param(file_path));
        self.http.put(&path, &payload, "repository_files.update")
    }

    /// Remove um arquivo do repositório.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `file_path`: Caminho do arquivo a ser removido.
    /// - `branch`: Nome da branch.
    /// - `commit_message`: Mensagem do commit de remoção.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn delete(&self, project_id: u64, file_path: &str, branch: &str, commit_message: &str) -> Result<(), GitLabError> {
        let path = format!("projects/{}/repository/files/{}", project_id, encode_query_param(file_path));
        let body = serde_json::json!({ "branch": branch, "commit_message": commit_message });
        self.http.delete_with_body(&path, &body, "repository_files.delete")
    }
}
