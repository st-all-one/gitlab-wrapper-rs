use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use std::sync::Arc;
use crate::types::*;
use crate::utils::encoding::filter_to_query;

#[derive(Debug)]
pub struct TodosResource {
    http: Arc<HttpClient>,
}

impl TodosResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    pub fn list(&self, filter: Option<&TodoFilter>) -> Result<Vec<Todo>, GitLabError> {
        let query = filter_to_query(filter);
        self.http.get("todos", &query, "todos.list")
    }

    pub fn mark_done(&self, todo_id: u64) -> Result<Todo, GitLabError> {
        let path = format!("todos/{}/mark_as_done", todo_id);
        self.http.post(&path, &serde_json::Value::Null, "todos.mark_done")
    }

    pub fn mark_all_done(&self) -> Result<Vec<Todo>, GitLabError> {
        self.http.post("todos/mark_all_as_done", &serde_json::Value::Null, "todos.mark_all_done")
    }
}
