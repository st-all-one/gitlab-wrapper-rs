use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use crate::utils::encoding::filter_to_query;
use std::sync::Arc;

/// Recurso de API para pacotes Composer no GitLab.
#[derive(Debug)]
pub struct PackageComposerResource {
    http: Arc<HttpClient>,
}

impl PackageComposerResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Executa a operação .
    /// Executa a operacao `list_versions`.
    pub async fn list_versions(
        &self,
        project_id: u64,
        filter: Option<&PackageTypeFilter>,
    ) -> Result<Vec<ComposerPackage>, GitLabError> {
        let path = format!("projects/{}/packages/composer", project_id);
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "package_composer.list").await
    }
}
