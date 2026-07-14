use crate::ErrorCategory;
use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use crate::utils::encoding::filter_to_query;
use std::sync::Arc;

/// Recurso de API para operações com usuários no GitLab.
#[derive(Debug)]
pub struct UsersResource {
    http: Arc<HttpClient>,
}

impl UsersResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista usuários com filtros opcionais.
    ///
    /// ## Params
    /// - `filter`: Filtros opcionais para a consulta.
    ///
    /// ## Returns
    /// `Result<Vec<User>, GitLabError>` — lista de usuários.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list(&self, filter: Option<&UserFilter>) -> Result<Vec<User>, GitLabError> {
        let query = filter_to_query(filter);
        self.http.get("users", &query, "users.list").await
    }

    /// Obtém um usuário pelo ID.
    ///
    /// ## Params
    /// - `user_id`: ID do usuário no GitLab.
    ///
    /// ## Returns
    /// `Result<User, GitLabError>` — dados do usuário solicitado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get(&self, user_id: u64) -> Result<User, GitLabError> {
        let path = format!("users/{}", user_id);
        self.http.get(&path, &[], "users.get").await
    }

    /// Obtém os dados do usuário autenticado.
    ///
    /// ## Returns
    /// `Result<User, GitLabError>` — dados do usuário atual.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get_current(&self) -> Result<User, GitLabError> {
        self.http.get("user", &[], "users.get_current").await
    }

    /// Cria um novo usuário.
    ///
    /// ## Params
    /// - `payload`: Dados para criar o usuário.
    ///
    /// ## Returns
    /// `Result<User, GitLabError>` — dados do usuário criado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn create(&self, payload: &CreateUserPayload) -> Result<User, GitLabError> {
        self.http.post("users", &payload, "users.create").await
    }

    /// Atualiza um usuário existente.
    ///
    /// ## Params
    /// - `user_id`: ID do usuário no GitLab.
    /// - `payload`: Dados para atualizar o usuário.
    ///
    /// ## Returns
    /// `Result<User, GitLabError>` — dados do usuário atualizado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn update(
        &self,
        user_id: u64,
        payload: &UpdateUserPayload,
    ) -> Result<User, GitLabError> {
        let path = format!("users/{}", user_id);
        self.http.put(&path, &payload, "users.update").await
    }

    /// Remove um usuário.
    ///
    /// ## Params
    /// - `user_id`: ID do usuário no GitLab.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn delete(&self, user_id: u64) -> Result<(), GitLabError> {
        let path = format!("users/{}", user_id);
        self.http.delete(&path, &[], "users.delete").await
    }

    /// Obtém o status de um usuário.
    ///
    /// ## Params
    /// - `user_id`: ID do usuário no GitLab.
    ///
    /// ## Returns
    /// `Result<UserStatus, GitLabError>` — status do usuário.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn status(&self, user_id: u64) -> Result<UserStatus, GitLabError> {
        let path = format!("users/{}/status", user_id);
        self.http.get(&path, &[], "users.status").await
    }

    /// Define o status do usuário autenticado.
    ///
    /// ## Params
    /// - `emoji`: Emoji opcional para o status.
    /// - `message`: Mensagem opcional para o status.
    ///
    /// ## Returns
    /// `Result<UserStatus, GitLabError>` — status atualizado do usuário.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn set_status(
        &self,
        emoji: Option<&str>,
        message: Option<&str>,
    ) -> Result<UserStatus, GitLabError> {
        let mut body = serde_json::Map::new();
        if let Some(e) = emoji {
            body.insert("emoji".into(), serde_json::json!(e));
        }
        if let Some(m) = message {
            body.insert("message".into(), serde_json::json!(m));
        }
        // Some GitLab versions return the full UserStatus object,
        // others return a minimal response — handle both.
        match self
            .http
            .put::<UserStatus, _>(
                "user/status",
                &serde_json::Value::Object(body),
                "users.set_status",
            )
            .await
        {
            Ok(status) => Ok(status),
            Err(e) => {
                // If the request succeeded but parsing failed, return a best-effort status
                if let GitLabError::Api { category: ErrorCategory::ParseError, .. } = &e {
                    Ok(UserStatus {
                        emoji: emoji.map(|e| e.to_string()),
                        message: message.map(|m| m.to_string()),
                        message_html: None,
                    })
                } else {
                    Err(e)
                }
            }
        }
    }

    /// Desativa um usuário.
    ///
    /// ## Params
    /// - `user_id`: ID do usuário no GitLab.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn deactivate(&self, user_id: u64) -> Result<(), GitLabError> {
        let path = format!("users/{}/deactivate", user_id);
        self.http.post(&path, &serde_json::json!({}), "users.deactivate").await
    }

    /// Reativa um usuário desativado.
    ///
    /// ## Params
    /// - `user_id`: ID do usuário no GitLab.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn activate(&self, user_id: u64) -> Result<(), GitLabError> {
        let path = format!("users/{}/activate", user_id);
        self.http.post(&path, &serde_json::json!({}), "users.activate").await
    }

    /// Bane um usuário.
    ///
    /// ## Params
    /// - `user_id`: ID do usuário no GitLab.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn ban(&self, user_id: u64) -> Result<(), GitLabError> {
        let path = format!("users/{}/ban", user_id);
        self.http.post(&path, &serde_json::json!({}), "users.ban").await
    }

    /// Remove o banimento de um usuário.
    ///
    /// ## Params
    /// - `user_id`: ID do usuário no GitLab.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn unban(&self, user_id: u64) -> Result<(), GitLabError> {
        let path = format!("users/{}/unban", user_id);
        self.http.post(&path, &serde_json::json!({}), "users.unban").await
    }

    /// Obtém as preferências do usuário autenticado.
    ///
    /// ## Returns
    /// `Result<UserPreferences, GitLabError>` — preferências do usuário.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn preferences(&self) -> Result<UserPreferences, GitLabError> {
        self.http.get("user/preferences", &[], "users.preferences").await
    }

    /// Define as preferências do usuário autenticado.
    ///
    /// ## Params
    /// - `prefs`: Dados JSON com as preferências a serem definidas.
    ///
    /// ## Returns
    /// `Result<UserPreferences, GitLabError>` — preferências atualizadas.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn set_preferences(
        &self,
        prefs: &serde_json::Value,
    ) -> Result<UserPreferences, GitLabError> {
        self.http.put("user/preferences", &prefs, "users.set_preferences").await
    }
}
