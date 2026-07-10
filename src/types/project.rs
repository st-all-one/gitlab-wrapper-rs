use serde::{Deserialize, Serialize};
use crate::types::base::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Project {
    pub id: GitLabId,
    pub name: String,
    pub path: String,
    pub description: Option<String>,
    pub visibility: Option<String>,
    pub namespace: Option<ProjectNamespace>,
    pub default_branch: Option<String>,
    pub ssh_url_to_repo: Option<String>,
    pub http_url_to_repo: Option<String>,
    pub web_url: Option<String>,
    pub avatar_url: Option<String>,
    pub star_count: Option<u32>,
    pub forks_count: Option<u32>,
    pub last_activity_at: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub archived: Option<bool>,
    pub empty_repo: Option<bool>,
    pub owner: Option<AuthorInfo>,
    pub permissions: Option<ProjectPermissions>,
    pub statistics: Option<ProjectStatistics>,
    pub topics: Option<Vec<String>>,
    pub tag_list: Option<Vec<String>>,
    pub issues_enabled: Option<bool>,
    pub merge_requests_enabled: Option<bool>,
    pub wiki_enabled: Option<bool>,
    pub jobs_enabled: Option<bool>,
    pub snippets_enabled: Option<bool>,
    pub container_registry_enabled: Option<bool>,
    pub shared_runners_enabled: Option<bool>,
    pub public_jobs: Option<bool>,
    pub open_issues_count: Option<u32>,
    pub ci_default_git_depth: Option<u32>,
    pub ci_forward_deployment_enabled: Option<bool>,
    pub request_access_enabled: Option<bool>,
    pub only_allow_merge_if_pipeline_succeeds: Option<bool>,
    pub only_allow_merge_if_all_discussions_are_resolved: Option<bool>,
    pub remove_source_branch_after_merge: Option<bool>,
    pub printing_merge_request_link_enabled: Option<bool>,
    pub merge_method: Option<String>,
    pub squash_option: Option<String>,
    pub auto_devops_enabled: Option<bool>,
    pub auto_devops_deploy_strategy: Option<String>,
    pub autoclose_referenced_issues: Option<bool>,
    pub repository_storage: Option<String>,
    pub shared_with_groups: Option<Vec<serde_json::Value>>,
    pub _links: Option<Links>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ProjectNamespace {
    pub id: GitLabId,
    pub name: String,
    pub path: String,
    pub kind: Option<String>,
    pub full_path: Option<String>,
    pub parent_id: Option<GitLabId>,
    pub avatar_url: Option<String>,
    pub web_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ProjectPermissions {
    pub project_access: Option<ProjectAccessLevel>,
    pub group_access: Option<ProjectAccessLevel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ProjectAccessLevel {
    pub access_level: Option<u32>,
    pub notification_level: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ProjectStatistics {
    pub commit_count: Option<u64>,
    pub storage_size: Option<u64>,
    pub repository_size: Option<u64>,
    pub wiki_size: Option<u64>,
    pub lfs_objects_size: Option<u64>,
    pub job_artifacts_size: Option<u64>,
    pub pipeline_artifacts_size: Option<u64>,
    pub packages_size: Option<u64>,
    pub snippets_size: Option<u64>,
    pub uploads_size: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateProjectPayload {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initialize_with_readme: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_id: Option<GitLabId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateProjectPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_branch: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ProjectFilter {
    pub search: Option<String>,
    pub visibility: Option<String>,
    pub membership: Option<bool>,
    pub owned: Option<bool>,
    pub starred: Option<bool>,
    pub archived: Option<bool>,
    pub topic: Option<String>,
    pub order_by: Option<String>,
    pub sort: Option<String>,
    pub statistics: Option<bool>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}
