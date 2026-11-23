use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::{MavenPackage, PackageTypeFilter};
use crate::utils::encoding::filter_to_query;
use std::sync::Arc;

/// Recurso de API para pacotes Maven no GitLab.
#[derive(Debug)]
pub struct PackageMavenResource {
    http: Arc<HttpClient>,
}

impl PackageMavenResource {
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
    ) -> Result<Vec<MavenPackage>, GitLabError> {
        let path = format!("projects/{project_id}/packages/maven");
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "package_maven.list").await
    }
}
