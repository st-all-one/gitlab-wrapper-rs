use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use crate::utils::encoding::encode_query_param;
use std::sync::Arc;

/// Recurso de API para atributos customizados (project, group, user).
#[derive(Debug)]
pub struct CustomAttributesResource {
    http: Arc<HttpClient>,
}

impl CustomAttributesResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    fn project_path(project_id: u64) -> String {
        format!("projects/{}/custom_attributes", project_id)
    }
    fn group_path(group_id: u64) -> String {
        format!("groups/{}/custom_attributes", group_id)
    }
    fn user_path(user_id: u64) -> String {
        format!("users/{}/custom_attributes", user_id)
    }

    /// Executa a operação .
    /// Executa a operacao `list_project`.
    pub async fn list_project(&self, project_id: u64) -> Result<Vec<CustomAttribute>, GitLabError> {
        self.http.get(&Self::project_path(project_id), &[], "custom_attributes.list_project").await
    }
    /// Executa a operação .
    /// Executa a operacao `get_project`.
    pub async fn get_project(
        &self,
        project_id: u64,
        key: &str,
    ) -> Result<CustomAttribute, GitLabError> {
        self.http
            .get(
                &format!("{}/{}", Self::project_path(project_id), encode_query_param(key)),
                &[],
                "custom_attributes.get_project",
            )
            .await
    }
    /// Executa a operação .
    /// Executa a operacao `set_project`.
    pub async fn set_project(
        &self,
        project_id: u64,
        key: &str,
        payload: &SetCustomAttributePayload,
    ) -> Result<CustomAttribute, GitLabError> {
        self.http
            .put(
                &format!("{}/{}", Self::project_path(project_id), encode_query_param(key)),
                payload,
                "custom_attributes.set_project",
            )
            .await
    }
    /// Executa a operação .
    /// Executa a operacao `delete_project`.
    pub async fn delete_project(&self, project_id: u64, key: &str) -> Result<(), GitLabError> {
        self.http
            .delete(
                &format!("{}/{}", Self::project_path(project_id), encode_query_param(key)),
                &[],
                "custom_attributes.delete_project",
            )
            .await
    }
    /// Executa a operação .
    /// Executa a operacao `list_group`.
    pub async fn list_group(&self, group_id: u64) -> Result<Vec<CustomAttribute>, GitLabError> {
        self.http.get(&Self::group_path(group_id), &[], "custom_attributes.list_group").await
    }
    /// Executa a operação .
    /// Executa a operacao `get_group`.
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
    pub async fn list_user(&self, user_id: u64) -> Result<Vec<CustomAttribute>, GitLabError> {
        self.http.get(&Self::user_path(user_id), &[], "custom_attributes.list_user").await
    }
    /// Executa a operação .
    /// Executa a operacao `get_user`.
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
