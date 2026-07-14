use crate::types::base::GitLabId;
use serde::{Deserialize, Serialize};

/// Resposta da API GitLab representando um item de resultado de busca.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SearchResultItem {
    /// ID do item encontrado.
    pub id: Option<GitLabId>,
    /// Título do item.
    pub title: Option<String>,
    /// Descrição do item.
    pub description: Option<String>,
    /// Tipo do item (project, issue, merge_request, etc.).
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// URL do item.
    pub url: Option<String>,
    /// ID do projeto ao qual o item pertence.
    pub project_id: Option<GitLabId>,
    /// Nome de usuário associado ao item.
    pub username: Option<String>,
    /// Nome do arquivo (para resultados de código).
    pub filename: Option<String>,
    /// Nome base do arquivo (sem extensão).
    pub basename: Option<String>,
    /// Dados adicionais do resultado.
    pub data: Option<String>,
    /// Caminho do arquivo (para resultados de código).
    pub path: Option<String>,
    /// Referência (branch/tag) — renomeado de `ref` para `ref_` devido a palavra reservada.
    #[serde(rename = "ref")]
    pub ref_: Option<String>,
    /// Linha inicial do resultado (para código).
    pub startline: Option<u32>,
    /// Linguagem de programação (para código).
    pub language: Option<String>,
    /// Conteúdo da linha (para código).
    pub content: Option<String>,
}
