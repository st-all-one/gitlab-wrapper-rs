use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use crate::utils::encoding::filter_to_query;
use std::sync::Arc;

/// Recurso de API para pacotes Debian no GitLab.
#[derive(Debug)]
pub struct PackageDebianResource {
    http: Arc<HttpClient>,
}

impl PackageDebianResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Executa a operação .
    /// Executa a operacao `list_versions`.
    pub async fn list_versions(
        &self,
        project_id: u64,
        filter: Option<&PackageTypeFilter>,
    ) -> Result<Vec<DebianPackage>, GitLabError> {
        let path = format!("projects/{}/packages/debian", project_id);
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "package_debian.list").await
    }
}
