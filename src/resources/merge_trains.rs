use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::{MergeTrain, MergeTrainFilter};
use crate::utils::encoding::filter_to_query;
use std::sync::Arc;

/// Recurso de API para merge trains no GitLab.
#[derive(Debug)]
pub struct MergeTrainsResource {
    http: Arc<HttpClient>,
}

impl MergeTrainsResource {
    pub(crate) const fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Executa a operação .
    /// Executa a operacao `list`.
    ///
    /// ## Errors
    /// Retorna `GitLabError::Api` se o filtro for inválido ou o projeto não existir,
    /// ou `GitLabError::Http` em erro de transporte.
    pub async fn list(
        &self,
        project_id: u64,
        filter: Option<&MergeTrainFilter>,
    ) -> Result<Vec<MergeTrain>, GitLabError> {
        let path = format!("projects/{project_id}/merge_trains");
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "merge_trains.list").await
    }

    /// Executa a operação .
    /// Executa a operacao `get`.
    ///
    /// ## Errors
    /// Retorna `GitLabError::Api` se o merge train não existir ou `GitLabError::Http`
    /// em erro de transporte da requisição.
    pub async fn get(&self, project_id: u64, mr_iid: u32) -> Result<MergeTrain, GitLabError> {
        let path = format!("projects/{project_id}/merge_trains/{mr_iid}");
        self.http.get(&path, &[], "merge_trains.get").await
    }
}
