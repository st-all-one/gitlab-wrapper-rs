use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use std::sync::Arc;

/// Recurso de API para operações com configurações da aplicação GitLab.
#[derive(Debug)]
pub struct SettingsResource {
    http: Arc<HttpClient>,
}

impl SettingsResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Obtém as configurações atuais da aplicação GitLab.
    ///
    /// ## Returns
    /// `Result<serde_json::Value, GitLabError>` — configurações da aplicação.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get(&self) -> Result<serde_json::Value, GitLabError> {
        let path = "application/settings".to_string();
        self.http.get(&path, &[], "settings.get").await
    }

    /// Atualiza as configurações da aplicação GitLab.
    ///
    /// ## Params
    /// - `payload`: Configurações a serem atualizadas.
    ///
    /// ## Returns
    /// `Result<serde_json::Value, GitLabError>` — configurações atualizadas.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn update(
        &self,
        payload: &serde_json::Value,
    ) -> Result<serde_json::Value, GitLabError> {
        let path = "application/settings".to_string();
        self.http.put(&path, payload, "settings.update").await
    }
}
