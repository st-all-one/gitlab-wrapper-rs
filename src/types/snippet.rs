use crate::types::base::*;
use serde::{Deserialize, Serialize};

/// Resposta da API GitLab representando um snippet de projeto.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Snippet {
    /// ID único do snippet (global).
    pub id: GitLabId,
    /// Título do snippet.
    pub title: Option<String>,
    /// Descrição do snippet.
    pub description: Option<String>,
    /// Conteúdo do snippet.
    pub content: Option<String>,
    /// Nome do arquivo do snippet.
    pub file_name: Option<String>,
    /// Visibilidade do snippet ("public", "internal", "private").
    pub visibility: Option<String>,
    /// Autor do snippet.
    pub author: Option<AuthorInfo>,
    /// Data da última atualização no formato ISO 8601.
    pub updated_at: Option<String>,
    /// Data de criação no formato ISO 8601.
    pub created_at: Option<String>,
    /// URL do snippet no GitLab.
    pub web_url: Option<String>,
    /// URL do conteúdo bruto do snippet.
    pub raw_url: Option<String>,
    /// ID do projeto ao qual o snippet pertence.
    pub project_id: Option<GitLabId>,
}

/// Payload para criar um snippet de projeto na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateSnippetPayload {
    /// Título do snippet (obrigatório).
    pub title: String,
    /// Nome do arquivo do snippet (obrigatório).
    pub file_name: String,
    /// Conteúdo do snippet (obrigatório).
    pub content: String,
    /// Visibilidade do snippet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    /// Descrição do snippet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Payload para atualizar um snippet de projeto na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateSnippetPayload {
    /// Novo título do snippet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Novo nome do arquivo do snippet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// Novo conteúdo do snippet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Nova visibilidade do snippet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    /// Nova descrição do snippet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Filtros para listar snippets de projeto. Use `..Default::default()` para valores padrão.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SnippetFilter {
    /// Número da página para paginação.
    pub page: Option<u32>,
    /// Número de itens por página.
    pub per_page: Option<u32>,
}
