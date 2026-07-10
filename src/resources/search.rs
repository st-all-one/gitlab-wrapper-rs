use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use std::sync::Arc;
use crate::types::*;

#[derive(Debug)]
pub struct SearchResource {
    http: Arc<HttpClient>,
}

impl SearchResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    pub fn global(&self, scope: &str, search: &str) -> Result<Vec<SearchResultItem>, GitLabError> {
        let query = vec![("scope".to_string(), scope.to_string()), ("search".to_string(), search.to_string())];
        self.http.get("search", &query, "search.global")
    }

    pub fn in_group(&self, group_id: u64, scope: &str, search: &str) -> Result<Vec<SearchResultItem>, GitLabError> {
        let path = format!("groups/{}/search", group_id);
        let query = vec![("scope".to_string(), scope.to_string()), ("search".to_string(), search.to_string())];
        self.http.get(&path, &query, "search.in_group")
    }

    pub fn in_project(&self, project_id: u64, scope: &str, search: &str) -> Result<Vec<SearchResultItem>, GitLabError> {
        let path = format!("projects/{}/search", project_id);
        let query = vec![("scope".to_string(), scope.to_string()), ("search".to_string(), search.to_string())];
        self.http.get(&path, &query, "search.in_project")
    }
}
