use serde::{Deserialize, Serialize};

/// Item da árvore de repositório retornado pela API do GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RepositoryTreeItem {
    /// Hash ID da entrada na árvore.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Nome do arquivo ou diretório.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Tipo da entrada ("tree", "blob" ou "commit").
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// Caminho completo da entrada no repositório.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Modo do arquivo (ex.: "100644", "040000").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

/// Filtros para listar a árvore de repositório.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TreeFilter {
    /// Caminho para listar de forma aninhada.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Nome da branch, tag ou SHA do commit.
    #[serde(rename = "ref", skip_serializing_if = "Option::is_none")]
    pub ref_: Option<String>,
    /// Se verdadeiro, lista recursivamente.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recursive: Option<bool>,
    /// Número de itens por página.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>,
    /// Número da página.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
}
