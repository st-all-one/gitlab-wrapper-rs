use crate::types::base::*;
use serde::{Deserialize, Serialize};

/// Resposta da API GitLab representando uma nota em rascunho (draft note).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DraftNote {
    /// ID único da nota em rascunho.
    pub id: GitLabId,
    /// Autor da nota em rascunho.
    pub author: Option<AuthorInfo>,
    /// ID do merge request ao qual a nota pertence.
    pub merge_request_id: Option<GitLabId>,
    /// Indica se a nota deve resolver a discussão.
    pub resolve_discussion: Option<bool>,
    /// Corpo da nota em rascunho.
    pub note: Option<String>,
    /// Posição da nota (para notas posicionadas em código).
    pub position: Option<serde_json::Value>,
    /// Data de criação no formato ISO 8601.
    pub created_at: Option<String>,
    /// Data da última atualização no formato ISO 8601.
    pub updated_at: Option<String>,
}

/// Payload para criar uma nota em rascunho na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateDraftNotePayload {
    /// Corpo da nota em rascunho (obrigatório).
    pub note: String,
    /// Indica se a nota deve resolver a discussão.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolve_discussion: Option<bool>,
    /// Posição da nota (para notas posicionadas em código).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<serde_json::Value>,
}

/// Payload para atualizar uma nota em rascunho na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateDraftNotePayload {
    /// Novo corpo da nota em rascunho.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// Indica se a nota deve resolver a discussão.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolve_discussion: Option<bool>,
    /// Posição da nota (para notas posicionadas em código).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<serde_json::Value>,
}
