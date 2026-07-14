use serde::{Deserialize, Serialize};

/// Template da API GitLab (Dockerfile, .gitignore, CI YAML, licença).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Template {
    /// Nome do template.
    /// Campo `pub name`.
    pub name: String,
    /// Conteúdo do template.
    /// Campo `pub content`.
    pub content: Option<String>,
}

/// Filtro para listar templates.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TemplateFilter {
    /// Número da página.
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Campo `pub page`.
    pub page: Option<u32>,
    /// Quantidade de itens por página.
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Campo `pub per_page`.
    pub per_page: Option<u32>,
}
