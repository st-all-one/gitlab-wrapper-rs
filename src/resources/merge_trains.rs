use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use crate::utils::encoding::filter_to_query;
use std::sync::Arc;

/// Recurso de API para merge trains no GitLab.
#[derive(Debug)]
pub struct MergeTrainsResource {
    http: Arc<HttpClient>,
}

impl MergeTrainsResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Executa a operação .
    /// Executa a operacao `list`.
    pub async fn list(
        &self,
        project_id: u64,
        filter: Option<&MergeTrainFilter>,
    ) -> Result<Vec<MergeTrain>, GitLabError> {
        let path = format!("projects/{}/merge_trains", project_id);
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "merge_trains.list").await
    }

    /// Executa a operação .
    /// Executa a operacao `get`.
    pub async fn get(&self, project_id: u64, mr_iid: u32) -> Result<MergeTrain, GitLabError> {
        let path = format!("projects/{}/merge_trains/{}", project_id, mr_iid);
        self.http.get(&path, &[], "merge_trains.get").await
    }
}
