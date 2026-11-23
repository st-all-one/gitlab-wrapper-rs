use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::{CustomAttribute, SetCustomAttributePayload};
use crate::utils::encoding::encode_query_param;
use std::sync::Arc;

/// Recurso de API para atributos customizados (project, group, user).
#[derive(Debug)]
pub struct CustomAttributesResource {
    http: Arc<HttpClient>,
}

impl CustomAttributesResource {
    pub(crate) const fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    fn project_path(project_id: u64) -> String {
        format!("projects/{project_id}/custom_attributes")
    }
    fn group_path(group_id: u64) -> String {
        format!("groups/{group_id}/custom_attributes")
    }
    fn user_path(user_id: u64) -> String {
        format!("users/{user_id}/custom_attributes")
    }

    /// Executa a operação .
    /// Executa a operacao `list_project`.
    ///
    /// ## Errors
    /// Retorna `GitLabError::Api` se o projeto não existir ou `GitLabError::Http`
    /// em erro de transporte da listagem.
    pub async fn list_project(&self, project_id: u64) -> Result<Vec<CustomAttribute>, GitLabError> {
        self.http
            .get(
                &Self::project_path(project_id),
                &[],
                "custom_attributes.list_project",
            )
            .await
    }
    /// Executa a operação .
    /// Executa a operacao `get_project`.
    ///
    /// ## Errors
    /// Retorna `GitLabError::Api` se o atributo não existir ou `GitLabError::Http`
    /// em erro de transporte da requisição.
    pub async fn get_project(
        &self,
        project_id: u64,
        key: &str,
    ) -> Result<CustomAttribute, GitLabError> {
        self.http
            .get(
                &format!(
                    "{}/{}",
                    Self::project_path(project_id),
                    encode_query_param(key)
                ),
                &[],
                "custom_attributes.get_project",
            )
            .await
    }
    /// Executa a operação .
    /// Executa a operacao `set_project`.
    ///
    /// ## Errors
    /// Retorna `GitLabError::Api` se o payload for inválido ou `GitLabError::Http`
    /// em erro de transporte da requisição.
    pub async fn set_project(
        &self,
        project_id: u64,
        key: &str,
        payload: &SetCustomAttributePayload,
    ) -> Result<CustomAttribute, GitLabError> {
        self.http
            .put(
                &format!(
                    "{}/{}",
                    Self::project_path(project_id),
                    encode_query_param(key)
                ),
                payload,
                "custom_attributes.set_project",
            )
            .await
    }
    /// Executa a operação .
    /// Executa a operacao `delete_project`.
    ///
    /// ## Errors
    /// Retorna `GitLabError::Api` se o atributo não existir ou `GitLabError::Http`
    /// em erro de transporte da exclusão.
    pub async fn delete_project(&self, project_id: u64, key: &str) -> Result<(), GitLabError> {
        self.http
            .delete(
                &format!(
                    "{}/{}",
                    Self::project_path(project_id),
                    encode_query_param(key)
                ),
                &[],
                "custom_attributes.delete_project",
            )
            .await
    }
    /// Executa a operação .
    /// Executa a operacao `list_group`.
    ///
    /// ## Errors
    /// Retorna `GitLabError::Api` se o grupo não existir ou `GitLabError::Http`
    /// em erro de transporte da listagem.
    pub async fn list_group(&self, group_id: u64) -> Result<Vec<CustomAttribute>, GitLabError> {
        self.http
            .get(
                &Self::group_path(group_id),
                &[],
                "custom_attributes.list_group",
            )
            .await
    }
    /// Executa a operação .
    /// Executa a operacao `get_group`.
    ///
    /// ## Errors
    /// Retorna `GitLabError::Api` se o atributo não existir ou `GitLabError::Http`
    /// em erro de transporte da requisição.
    pub async fn get_group(
        &self,
        group_id: u64,
        key: &str,
    ) -> Result<CustomAttribute, GitLabError> {
        self.http
            .get(
                &format!("{}/{}", Self::group_path(group_id), encode_query_param(key)),
                &[],
                "custom_attributes.get_group",
            )
            .await
    }
    /// Executa a operação .
    /// Executa a operacao `set_group`.
    ///
    /// ## Errors
    /// Retorna `GitLabError::Api` se o payload for inválido ou `GitLabError::Http`
    /// em erro de transporte da requisição.
    pub async fn set_group(
        &self,
        group_id: u64,
        key: &str,
        payload: &SetCustomAttributePayload,
    ) -> Result<CustomAttribute, GitLabError> {
        self.http
            .put(
                &format!("{}/{}", Self::group_path(group_id), encode_query_param(key)),
                payload,
                "custom_attributes.set_group",
            )
            .await
    }
    /// Executa a operação .
    /// Executa a operacao `delete_group`.
    ///
    /// ## Errors
    /// Retorna `GitLabError::Api` se o atributo não existir ou `GitLabError::Http`
    /// em erro de transporte da exclusão.
    pub async fn delete_group(&self, group_id: u64, key: &str) -> Result<(), GitLabError> {
        self.http
            .delete(
                &format!("{}/{}", Self::group_path(group_id), encode_query_param(key)),
                &[],
                "custom_attributes.delete_group",
            )
            .await
    }
    /// Executa a operação .
    /// Executa a operacao `list_user`.
    ///
    /// ## Errors
    /// Retorna `GitLabError::Api` se o usuário não existir ou `GitLabError::Http`
    /// em erro de transporte da listagem.
    pub async fn list_user(&self, user_id: u64) -> Result<Vec<CustomAttribute>, GitLabError> {
        self.http
            .get(
                &Self::user_path(user_id),
                &[],
                "custom_attributes.list_user",
            )
            .await
    }
    /// Executa a operação .
    /// Executa a operacao `get_user`.
    ///
    /// ## Errors
    /// Retorna `GitLabError::Api` se o atributo não existir ou `GitLabError::Http`
    /// em erro de transporte da requisição.
    pub async fn get_user(&self, user_id: u64, key: &str) -> Result<CustomAttribute, GitLabError> {
        self.http
            .get(
                &format!("{}/{}", Self::user_path(user_id), encode_query_param(key)),
                &[],
                "custom_attributes.get_user",
            )
            .await
    }
    /// Executa a operação .
    /// Executa a operacao `set_user`.
    ///
    /// ## Errors
    /// Retorna `GitLabError::Api` se o payload for inválido ou `GitLabError::Http`
    /// em erro de transporte da requisição.
    pub async fn set_user(
        &self,
        user_id: u64,
        key: &str,
        payload: &SetCustomAttributePayload,
    ) -> Result<CustomAttribute, GitLabError> {
        self.http
            .put(
                &format!("{}/{}", Self::user_path(user_id), encode_query_param(key)),
                payload,
                "custom_attributes.set_user",
            )
            .await
    }
    /// Executa a operação .
    /// Executa a operacao `delete_user`.
    ///
    /// ## Errors
    /// Retorna `GitLabError::Api` se o atributo não existir ou `GitLabError::Http`
    /// em erro de transporte da exclusão.
    pub async fn delete_user(&self, user_id: u64, key: &str) -> Result<(), GitLabError> {
        self.http
            .delete(
                &format!("{}/{}", Self::user_path(user_id), encode_query_param(key)),
                &[],
                "custom_attributes.delete_user",
            )
            .await
    }
}
