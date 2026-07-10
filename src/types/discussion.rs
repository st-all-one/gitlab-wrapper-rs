use serde::{Deserialize, Serialize};
use crate::types::base::*;
use crate::types::note::Note;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Discussion {
    pub id: String,
    pub notes: Option<Vec<Note>>,
    pub individual_note: Option<bool>,
    pub resolvable: Option<bool>,
    pub resolved: Option<bool>,
    pub resolved_by: Option<AuthorInfo>,
    pub resolved_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateDiscussionPayload {
    pub body: String,
}
