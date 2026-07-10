use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use std::sync::Arc;
use crate::types::*;
use crate::utils::encoding::filter_to_query;

#[derive(Debug)]
pub struct UsersResource {
    http: Arc<HttpClient>,
}

impl UsersResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    pub fn list(&self, filter: Option<&UserFilter>) -> Result<Vec<User>, GitLabError> {
        let query = filter_to_query(filter);
        self.http.get("users", &query, "users.list")
    }

    pub fn get(&self, user_id: u64) -> Result<User, GitLabError> {
        let path = format!("users/{}", user_id);
        self.http.get(&path, &[], "users.get")
    }

    pub fn get_current(&self) -> Result<User, GitLabError> {
        self.http.get("user", &[], "users.get_current")
    }

    pub fn create(&self, payload: &CreateUserPayload) -> Result<User, GitLabError> {
        self.http.post("users", &payload, "users.create")
    }

    pub fn update(&self, user_id: u64, payload: &UpdateUserPayload) -> Result<User, GitLabError> {
        let path = format!("users/{}", user_id);
        self.http.put(&path, &payload, "users.update")
    }

    pub fn delete(&self, user_id: u64) -> Result<(), GitLabError> {
        let path = format!("users/{}", user_id);
        self.http.delete(&path, &[], "users.delete")
    }

    pub fn status(&self, user_id: u64) -> Result<UserStatus, GitLabError> {
        let path = format!("users/{}/status", user_id);
        self.http.get(&path, &[], "users.status")
    }

    pub fn set_status(&self, emoji: Option<&str>, message: Option<&str>) -> Result<UserStatus, GitLabError> {
        let mut body = serde_json::json!({});
        if let Some(e) = emoji {
            body.as_object_mut().unwrap().insert("emoji".into(), serde_json::json!(e));
        }
        if let Some(m) = message {
            body.as_object_mut().unwrap().insert("message".into(), serde_json::json!(m));
        }
        self.http.put("user/status", &body, "users.set_status")
    }

    pub fn deactivate(&self, user_id: u64) -> Result<(), GitLabError> {
        let path = format!("users/{}/deactivate", user_id);
        self.http.post(&path, &serde_json::Value::Null, "users.deactivate")
    }

    pub fn activate(&self, user_id: u64) -> Result<(), GitLabError> {
        let path = format!("users/{}/activate", user_id);
        self.http.post(&path, &serde_json::Value::Null, "users.activate")
    }

    pub fn ban(&self, user_id: u64) -> Result<(), GitLabError> {
        let path = format!("users/{}/ban", user_id);
        self.http.post(&path, &serde_json::Value::Null, "users.ban")
    }

    pub fn unban(&self, user_id: u64) -> Result<(), GitLabError> {
        let path = format!("users/{}/unban", user_id);
        self.http.post(&path, &serde_json::Value::Null, "users.unban")
    }
}
