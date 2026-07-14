use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use crate::utils::encoding::encode_query_param;
use std::sync::Arc;

/// Recurso de API para templates de projeto no GitLab.
#[derive(Debug)]
pub struct ProjectTemplatesResource {
    http: Arc<HttpClient>,
}

impl ProjectTemplatesResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Executa a operação .
    /// Executa a operacao `list`.
    pub async fn list(&self, project_id: u64, type_: &str) -> Result<Vec<Template>, GitLabError> {
        let path = format!("projects/{}/templates/{}", project_id, encode_query_param(type_));
        self.http.get(&path, &[], "project_templates.list").await
    }

    /// Executa a operação .
    /// Executa a operacao `get`.
    pub async fn get(
        &self,
        project_id: u64,
        type_: &str,
        key: &str,
    ) -> Result<Template, GitLabError> {
        let path = format!(
            "projects/{}/templates/{}/{}",
            project_id,
            encode_query_param(type_),
            encode_query_param(key)
        );
        self.http.get(&path, &[], "project_templates.get").await
    }
}
