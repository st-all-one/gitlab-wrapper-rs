use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use std::sync::Arc;
use crate::types::*;
use crate::utils::encoding::filter_to_query;

#[derive(Debug)]
pub struct EventsResource {
    http: Arc<HttpClient>,
}

impl EventsResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    pub fn list(&self, filter: Option<&EventFilter>) -> Result<Vec<Event>, GitLabError> {
        let query = filter_to_query(filter);
        self.http.get("events", &query, "events.list")
    }

    pub fn list_user_events(&self, user_id: u64, filter: Option<&EventFilter>) -> Result<Vec<Event>, GitLabError> {
        let path = format!("users/{}/events", user_id);
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "events.list_user")
    }

    pub fn list_project_events(&self, project_id: u64, filter: Option<&EventFilter>) -> Result<Vec<Event>, GitLabError> {
        let path = format!("projects/{}/events", project_id);
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "events.list_project")
    }
}
