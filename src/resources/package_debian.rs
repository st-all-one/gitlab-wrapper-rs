use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::{DebianPackage, PackageTypeFilter};
use crate::utils::encoding::filter_to_query;
use std::sync::Arc;

/// Recurso de API para pacotes Debian no GitLab.
#[derive(Debug)]
pub struct PackageDebianResource {
    http: Arc<HttpClient>,
}

impl PackageDebianResource {
    pub(crate) const fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Executa a operação .
    /// Executa a operacao `list_versions`.
    ///
    /// ## Errors
    /// Retorna `GitLabError::Api` se o filtro for inválido ou o projeto não existir,
    /// ou `GitLabError::Http` em erro de transporte da listagem.
    pub async fn list_versions(
        &self,
        project_id: u64,
        filter: Option<&PackageTypeFilter>,
    ) -> Result<Vec<DebianPackage>, GitLabError> {
        let path = format!("projects/{project_id}/packages/debian");
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "package_debian.list").await
    }
}
