use serde::{Deserialize, Serialize};
use crate::types::base::*;
use crate::types::branch::BranchCommit;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Release {
    pub tag_name: Option<String>,
    pub tag_path: Option<String>,
    pub description: Option<String>,
    pub description_html: Option<String>,
    pub name: Option<String>,
    pub created_at: Option<String>,
    pub released_at: Option<String>,
    pub author: Option<AuthorInfo>,
    pub commit: Option<BranchCommit>,
    pub assets: Option<ReleaseAssets>,
    pub evidences: Option<Vec<ReleaseEvidence>>,
    pub _links: Option<ReleaseLinks>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReleaseAssets {
    pub count: Option<u32>,
    pub sources: Option<Vec<ReleaseSource>>,
    pub links: Option<Vec<ReleaseLinkItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReleaseSource {
    pub format: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReleaseLinkItem {
    pub id: Option<GitLabId>,
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReleaseEvidence {
    pub sha: Option<String>,
    pub filepath: Option<String>,
    pub collected_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReleaseLinks {
    pub self_: Option<String>,
    pub edit_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateReleasePayload {
    pub tag_name: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub released_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milestones: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateReleasePayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub released_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milestones: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateReleaseLinkPayload {
    pub name: String,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filepath: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_type: Option<String>,
}
