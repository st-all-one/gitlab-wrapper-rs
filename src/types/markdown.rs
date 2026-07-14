use serde::{Deserialize, Serialize};

/// Resultado da renderização de Markdown.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MarkdownResult {
    /// Campo `pub html`.
    pub html: String,
}

/// Payload para renderizar Markdown.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MarkdownPayload {
    /// Campo `pub text`.
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Campo `pub gfm`.
    pub gfm: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Campo `pub project`.
    pub project: Option<String>,
}
