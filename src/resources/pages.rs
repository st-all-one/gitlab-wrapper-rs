use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use std::sync::Arc;

/// Recurso de API para operações com páginas do GitLab Pages.
#[derive(Debug)]
pub struct PagesResource {
    http: Arc<HttpClient>,
}

impl PagesResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Obtém as configurações de GitLab Pages de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    ///
    /// ## Returns
    /// `Result<serde_json::Value, GitLabError>` — configurações de pages.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get_settings(&self, project_id: u64) -> Result<serde_json::Value, GitLabError> {
        let path = format!("projects/{}/pages", project_id);
        self.http.get(&path, &[], "pages.get_settings").await
    }

    /// Atualiza as configurações de GitLab Pages de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `payload`: Dados para atualizar as configurações.
    ///
    /// ## Returns
    /// `Result<serde_json::Value, GitLabError>` — configurações atualizadas.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn update_settings(
        &self,
        project_id: u64,
        payload: &PageSettings,
    ) -> Result<serde_json::Value, GitLabError> {
        let path = format!("projects/{}/pages", project_id);
        self.http.put(&path, payload, "pages.update_settings").await
    }
}
