use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use std::sync::Arc;

/// Recurso de API para renderizar Markdown.
#[derive(Debug)]
pub struct MarkdownResource {
    http: Arc<HttpClient>,
}

impl MarkdownResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Executa a operação .
    /// Executa a operacao `render`.
    pub async fn render(&self, payload: &MarkdownPayload) -> Result<MarkdownResult, GitLabError> {
        self.http.post("markdown", payload, "markdown.render").await
    }
}
