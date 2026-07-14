use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use std::sync::Arc;

/// Recurso de API para operações com configurações de notificação no GitLab.
#[derive(Debug)]
pub struct NotificationSettingsResource {
    http: Arc<HttpClient>,
}

impl NotificationSettingsResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Obtém as configurações globais de notificação do usuário atual.
    ///
    /// ## Returns
    /// `Result<serde_json::Value, GitLabError>` — configurações de notificação globais.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get_global(&self) -> Result<serde_json::Value, GitLabError> {
        self.http.get("notification_settings", &[], "notification_settings.get_global").await
    }

    /// Atualiza as configurações globais de notificação do usuário atual.
    ///
    /// ## Params
    /// - `payload`: Dados para atualizar as configurações.
    ///
    /// ## Returns
    /// `Result<serde_json::Value, GitLabError>` — configurações atualizadas.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn update_global(
        &self,
        payload: &UpdateNotificationPayload,
    ) -> Result<serde_json::Value, GitLabError> {
        self.http.put("notification_settings", payload, "notification_settings.update_global").await
    }

    /// Obtém as configurações de notificação de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    ///
    /// ## Returns
    /// `Result<serde_json::Value, GitLabError>` — configurações de notificação do projeto.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get_project(&self, project_id: u64) -> Result<serde_json::Value, GitLabError> {
        let path = format!("projects/{}/notification_settings", project_id);
        self.http.get(&path, &[], "notification_settings.get_project").await
    }

    /// Atualiza as configurações de notificação de um projeto.
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
    pub async fn update_project(
        &self,
        project_id: u64,
        payload: &UpdateNotificationPayload,
    ) -> Result<serde_json::Value, GitLabError> {
        let path = format!("projects/{}/notification_settings", project_id);
        self.http.put(&path, payload, "notification_settings.update_project").await
    }

    /// Obtém as configurações de notificação de um grupo.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    ///
    /// ## Returns
    /// `Result<serde_json::Value, GitLabError>` — configurações de notificação do grupo.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get_group(&self, group_id: u64) -> Result<serde_json::Value, GitLabError> {
        let path = format!("groups/{}/notification_settings", group_id);
        self.http.get(&path, &[], "notification_settings.get_group").await
    }

    /// Atualiza as configurações de notificação de um grupo.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    /// - `payload`: Dados para atualizar as configurações.
    ///
    /// ## Returns
    /// `Result<serde_json::Value, GitLabError>` — configurações atualizadas.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn update_group(
        &self,
        group_id: u64,
        payload: &UpdateNotificationPayload,
    ) -> Result<serde_json::Value, GitLabError> {
        let path = format!("groups/{}/notification_settings", group_id);
        self.http.put(&path, payload, "notification_settings.update_group").await
    }
}
