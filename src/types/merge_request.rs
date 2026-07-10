use serde::{Deserialize, Serialize};
use crate::types::base::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MergeRequest {
    pub id: GitLabId,
    pub iid: u32,
    pub project_id: GitLabId,
    pub title: String,
    pub description: Option<String>,
    pub state: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub merged_at: Option<String>,
    pub closed_at: Option<String>,
    pub source_branch: Option<String>,
    pub target_branch: Option<String>,
    pub source_project_id: Option<GitLabId>,
    pub target_project_id: Option<GitLabId>,
    pub author: Option<AuthorInfo>,
    pub assignees: Option<Vec<AuthorInfo>>,
    pub reviewers: Option<Vec<AuthorInfo>>,
    pub web_url: Option<String>,
    pub merge_status: Option<String>,
    pub merge_when_pipeline_succeeds: Option<bool>,
    pub draft: Option<bool>,
    pub work_in_progress: Option<bool>,
    pub labels: Option<Vec<String>>,
    pub milestone: Option<serde_json::Value>,
    pub time_stats: Option<TimeStats>,
    pub task_completion_status: Option<TaskCompletionStatus>,
    pub upvotes: Option<u32>,
    pub downvotes: Option<u32>,
    pub user_notes_count: Option<u32>,
    pub changes_count: Option<String>,
    pub squash: Option<bool>,
    pub pipeline: Option<MergeRequestPipeline>,
    pub diff_refs: Option<MergeRequestDiffRefs>,
    pub force_remove_source_branch: Option<bool>,
    pub merge_commit_sha: Option<String>,
    pub squash_commit_sha: Option<String>,
    pub should_remove_source_branch: Option<bool>,
    pub merge_user: Option<AuthorInfo>,
    pub _links: Option<Links>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MergeRequestPipeline {
    pub id: GitLabId,
    pub ref_: Option<String>,
    pub sha: Option<String>,
    pub status: Option<String>,
    pub web_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MergeRequestDiffRefs {
    pub base_sha: Option<String>,
    pub head_sha: Option<String>,
    pub start_sha: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateMergeRequestPayload {
    pub source_branch: String,
    pub target_branch: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee_ids: Option<Vec<GitLabId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviewer_ids: Option<Vec<GitLabId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milestone_id: Option<GitLabId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_source_branch: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub squash: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateMergeRequestPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_event: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee_ids: Option<Vec<GitLabId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviewer_ids: Option<Vec<GitLabId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milestone_id: Option<GitLabId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_source_branch: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub squash: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discussion_locked: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MergeRequestFilter {
    pub state: Option<String>,
    pub labels: Option<String>,
    pub milestone: Option<String>,
    pub assignee_id: Option<GitLabId>,
    pub author_id: Option<GitLabId>,
    pub reviewer_id: Option<GitLabId>,
    pub source_branch: Option<String>,
    pub target_branch: Option<String>,
    pub search: Option<String>,
    pub draft: Option<bool>,
    pub scope: Option<String>,
    pub order_by: Option<String>,
    pub sort: Option<String>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MergePayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_commit_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub squash_commit_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub should_remove_source_branch: Option<bool>,
}
