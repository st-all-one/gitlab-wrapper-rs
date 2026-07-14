use crate::types::base::AuthorInfo;
use serde::{Deserialize, Serialize};

/// Resposta da API GitLab representando uma página do wiki.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WikiPage {
    /// Conteúdo da página do wiki.
    pub content: Option<String>,
    /// Codificação do conteúdo ("base64" ou "text").
    pub encoding: Option<String>,
    /// Formato da página ("markdown", "rdoc", "asciidoc", etc.).
    pub format: Option<String>,
    /// Slug (identificador na URL) da página.
    pub slug: Option<String>,
    /// Título da página do wiki.
    pub title: Option<String>,
    /// Versão atual da página.
    pub version: Option<WikiPageVersion>,
    /// Ordem da página no wiki.
    pub page_order: Option<u32>,
    /// Data da última atualização no formato ISO 8601.
    pub last_updated_at: Option<String>,
    /// Último usuário que atualizou a página.
    pub last_updated_by: Option<AuthorInfo>,
}

/// Resposta da API GitLab representando a versão de uma página do wiki.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WikiPageVersion {
    /// ID da versão.
    pub id: Option<String>,
    /// SHA da versão.
    pub sha: Option<String>,
    /// Data de autoria da versão no formato ISO 8601.
    pub authored_date: Option<String>,
    /// Autor da versão.
    pub author: Option<AuthorInfo>,
    /// Commit associado à versão.
    pub commit: Option<WikiPageCommit>,
}

/// Resposta da API GitLab representando o commit de uma versão do wiki.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WikiPageCommit {
    /// ID do commit.
    pub id: Option<String>,
    /// Mensagem do commit.
    pub message: Option<String>,
}

/// Payload para criar uma página do wiki na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateWikiPagePayload {
    /// Título da página do wiki.
    pub title: String,
    /// Conteúdo da página do wiki.
    pub content: String,
    /// Formato da página ("markdown", "rdoc", "asciidoc").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

/// Payload para atualizar uma página do wiki na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateWikiPagePayload {
    /// Novo título da página do wiki.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Novo conteúdo da página do wiki.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Novo formato da página ("markdown", "rdoc", "asciidoc").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}
