use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use std::sync::Arc;

/// Recurso de API para operações de importação e exportação de projetos no GitLab.
#[derive(Debug)]
pub struct ImportExportResource {
    http: Arc<HttpClient>,
}

impl ImportExportResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Agenda a exportação de um projeto.
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
    pub async fn schedule_export(&self, project_id: u64) -> Result<(), GitLabError> {
        let path = format!("projects/{}/export", project_id);
        self.http.post(&path, &serde_json::json!({}), "import_export.schedule_export").await
    }

    /// Consulta o status da exportação de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    ///
    /// ## Returns
    /// `Result<serde_json::Value, GitLabError>` — status da exportação.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn export_status(&self, project_id: u64) -> Result<serde_json::Value, GitLabError> {
        let path = format!("projects/{}/export", project_id);
        self.http.get(&path, &[], "import_export.export_status").await
    }

    /// Faz o download do arquivo de exportação de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    ///
    /// ## Returns
    /// `Result<serde_json::Value, GitLabError>` — dados do arquivo de exportação.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn download_export(&self, project_id: u64) -> Result<serde_json::Value, GitLabError> {
        let path = format!("projects/{}/export/download", project_id);
        self.http.get(&path, &[], "import_export.download_export").await
    }

    /// Importa um projeto a partir de um arquivo de exportação.
    ///
    /// ## Params
    /// - `payload`: Dados da importação.
    ///
    /// ## Returns
    /// `Result<Project, GitLabError>` — dados do projeto importado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn import(&self, payload: &ImportPayload) -> Result<Project, GitLabError> {
        let path = "projects/import".to_string();
        self.http.post(&path, payload, "import_export.import").await
    }

    /// Consulta o status da importação de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    ///
    /// ## Returns
    /// `Result<serde_json::Value, GitLabError>` — status da importação.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn import_status(&self, project_id: u64) -> Result<serde_json::Value, GitLabError> {
        let path = format!("projects/{}/import", project_id);
        self.http.get(&path, &[], "import_export.import_status").await
    }
}
