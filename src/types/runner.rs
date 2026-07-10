use serde::{Deserialize, Serialize};
use crate::types::base::GitLabId;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Runner {
    pub id: GitLabId,
    pub description: Option<String>,
    pub ip_address: Option<String>,
    pub active: Option<bool>,
    pub paused: Option<bool>,
    pub is_shared: Option<bool>,
    pub runner_type: Option<String>,
    pub status: Option<String>,
    pub online: Option<bool>,
    pub architecture: Option<String>,
    pub platform: Option<String>,
    pub locked: Option<bool>,
    pub access_level: Option<String>,
    pub version: Option<String>,
    pub revision: Option<String>,
    pub tag_list: Option<Vec<String>>,
    pub run_untagged: Option<bool>,
    pub maximum_timeout: Option<u32>,
    pub projects: Option<Vec<RunnerProject>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RunnerProject {
    pub id: GitLabId,
    pub name: Option<String>,
    pub full_path: Option<String>,
    pub web_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateRunnerPayload {
    pub runner_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_untagged: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_timeout: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateRunnerPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_untagged: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_timeout: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}
