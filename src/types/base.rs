use serde::{Deserialize, Serialize};

pub type GitLabId = u64;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorInfo {
    pub id: GitLabId,
    pub username: String,
    pub name: String,
    pub state: Option<String>,
    pub avatar_url: Option<String>,
    pub web_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Links {
    pub self_: Option<String>,
    pub notes: Option<String>,
    pub award_emoji: Option<String>,
    pub project: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeStats {
    pub time_estimate: Option<i64>,
    pub total_time_spent: Option<i64>,
    pub human_time_estimate: Option<String>,
    pub human_total_time_spent: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskCompletionStatus {
    pub count: Option<u32>,
    pub completed_count: Option<u32>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaginationParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub pagination: Option<String>,
    pub order_by: Option<String>,
    pub sort: Option<String>,
}
