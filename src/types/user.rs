use serde::{Deserialize, Serialize};
use crate::types::base::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct User {
    pub id: GitLabId,
    pub username: String,
    pub name: String,
    pub state: Option<String>,
    pub avatar_url: Option<String>,
    pub web_url: Option<String>,
    pub email: Option<String>,
    pub created_at: Option<String>,
    pub bio: Option<String>,
    pub location: Option<String>,
    pub public_email: Option<String>,
    pub skype: Option<String>,
    pub linkedin: Option<String>,
    pub twitter: Option<String>,
    pub website_url: Option<String>,
    pub organization: Option<String>,
    pub last_sign_in_at: Option<String>,
    pub confirmed_at: Option<String>,
    pub last_activity_on: Option<String>,
    pub identities: Option<Vec<UserIdentity>>,
    pub is_admin: Option<bool>,
    pub is_bot: Option<bool>,
    pub is_external: Option<bool>,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserIdentity {
    pub provider: Option<String>,
    pub extern_uid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserStatus {
    pub emoji: Option<String>,
    pub message: Option<String>,
    pub message_html: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserPreferences {
    pub id: Option<GitLabId>,
    pub user_id: Option<GitLabId>,
    pub view_diffs_file_by_file: Option<bool>,
    pub show_whitespace_in_diffs: Option<bool>,
    pub use_legacy_web_ide: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateUserPayload {
    pub email: String,
    pub username: String,
    pub name: String,
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_confirmation: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateUserPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserFilter {
    pub username: Option<String>,
    pub search: Option<String>,
    pub active: Option<bool>,
    pub blocked: Option<bool>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}
