use serde::{Deserialize, Serialize};
use crate::types::base::*;
use crate::types::note::Note;

/// Resposta da API GitLab representando uma discussão (thread de notas).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Discussion {
    /// ID único da discussão.
    pub id: String,
    /// Notas pertencentes à discussão.
    pub notes: Option<Vec<Note>>,
    /// Indica se a discussão é uma nota individual (não uma thread).
    pub individual_note: Option<bool>,
    /// Indica se a discussão é resolvível.
    pub resolvable: Option<bool>,
    /// Indica se a discussão foi resolvida.
    pub resolved: Option<bool>,
    /// Usuário que resolveu a discussão.
    pub resolved_by: Option<AuthorInfo>,
    /// Data em que a discussão foi resolvida.
    pub resolved_at: Option<String>,
}

/// Payload para criar uma discussão na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateDiscussionPayload {
    /// Corpo da nota inicial da discussão (obrigatório).
    pub body: String,
}
