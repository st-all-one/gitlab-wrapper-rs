use serde::{Deserialize, Serialize};
use crate::types::base::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Issue {
    pub id: GitLabId,
    pub iid: u32,
    pub project_id: GitLabId,
    pub title: String,
    pub description: Option<String>,
    pub state: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub closed_at: Option<String>,
    pub labels: Option<Vec<String>>,
    pub milestone: Option<serde_json::Value>,
    pub assignees: Option<Vec<AuthorInfo>>,
    pub author: Option<AuthorInfo>,
    pub web_url: Option<String>,
    pub confidential: Option<bool>,
    pub discussion_locked: Option<bool>,
    pub issue_type: Option<String>,
    pub severity: Option<String>,
    pub time_stats: Option<TimeStats>,
    pub task_completion_status: Option<TaskCompletionStatus>,
    pub references: Option<IssueReferences>,
    pub moved_to_id: Option<GitLabId>,
    pub duplicated_to_id: Option<GitLabId>,
    pub updated_by_id: Option<GitLabId>,
    pub last_edited_at: Option<String>,
    pub last_edited_by: Option<AuthorInfo>,
    pub user_notes_count: Option<u32>,
    pub upvotes: Option<u32>,
    pub downvotes: Option<u32>,
    pub merge_requests_count: Option<u32>,
    pub due_date: Option<String>,
    pub weight: Option<i32>,
    pub _links: Option<Links>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IssueReferences {
    pub short: Option<String>,
    pub relative: Option<String>,
    pub full: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateIssuePayload {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidential: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee_ids: Option<Vec<GitLabId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milestone_id: Option<GitLabId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateIssuePayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidential: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_event: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee_ids: Option<Vec<GitLabId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milestone_id: Option<GitLabId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IssueFilter {
    pub state: Option<String>,
    pub labels: Option<String>,
    pub assignee_id: Option<GitLabId>,
    pub milestone: Option<String>,
    pub search: Option<String>,
    pub scope: Option<String>,
    pub confidential: Option<bool>,
    pub order_by: Option<String>,
    pub sort: Option<String>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}
