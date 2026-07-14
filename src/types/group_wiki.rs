use serde::{Deserialize, Serialize};

/// Filtros para listagem de páginas de wiki de grupo.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WikiFilter {
    /// Incluir o conteúdo da página na resposta.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_content: Option<bool>,
    /// Ordenação das páginas (`title` ou `created_at`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
}
