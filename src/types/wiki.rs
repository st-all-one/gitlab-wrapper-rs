use serde::{Deserialize, Serialize};
use crate::types::base::AuthorInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WikiPage {
    pub content: Option<String>,
    pub encoding: Option<String>,
    pub format: Option<String>,
    pub slug: Option<String>,
    pub title: Option<String>,
    pub version: Option<WikiPageVersion>,
    pub page_order: Option<u32>,
    pub last_updated_at: Option<String>,
    pub last_updated_by: Option<AuthorInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WikiPageVersion {
    pub id: Option<String>,
    pub sha: Option<String>,
    pub authored_date: Option<String>,
    pub author: Option<AuthorInfo>,
    pub commit: Option<WikiPageCommit>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WikiPageCommit {
    pub id: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateWikiPagePayload {
    pub title: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateWikiPagePayload {
    pub title: Option<String>,
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}
