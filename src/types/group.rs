use serde::{Deserialize, Serialize};
use crate::types::base::*;

/// Resposta da API GitLab representando um grupo.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Group {
    /// ID único do grupo.
    pub id: GitLabId,
    /// Nome do grupo.
    pub name: String,
    /// Caminho do grupo (usado na URL).
    pub path: String,
    /// Descrição do grupo.
    pub description: Option<String>,
    /// Nível de visibilidade do grupo ("public", "internal", "private").
    pub visibility: Option<String>,
    /// URL do avatar do grupo.
    pub avatar_url: Option<String>,
    /// URL da página do grupo no GitLab.
    pub web_url: Option<String>,
    /// Nome completo do grupo (incluindo ancestrais).
    pub full_name: Option<String>,
    /// Caminho completo do grupo (incluindo ancestrais).
    pub full_path: Option<String>,
    /// ID do grupo pai.
    pub parent_id: Option<GitLabId>,
    /// Número de projetos no grupo.
    pub projects_count: Option<u32>,
    /// Número de subgrupos no grupo.
    pub subgroup_count: Option<u32>,
    /// Data de criação no formato ISO 8601.
    pub created_at: Option<String>,
    /// Data da última atualização no formato ISO 8601.
    pub updated_at: Option<String>,
    /// Grupos com os quais este grupo foi compartilhado.
    pub shared_with_groups: Option<Vec<serde_json::Value>>,
}

/// Payload para criar um grupo na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateGroupPayload {
    /// Nome do grupo.
    pub name: String,
    /// Caminho do grupo (usado na URL).
    pub path: String,
    /// Descrição do grupo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Nível de visibilidade do grupo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    /// ID do grupo pai (para criar subgrupo).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<GitLabId>,
}

/// Payload para atualizar um grupo na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateGroupPayload {
    /// Novo nome do grupo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Nova descrição do grupo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Novo nível de visibilidade do grupo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

/// Filtros disponíveis para listar grupos. Use `..Default::default()` para valores padrão.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GroupFilter {
    /// Termo de busca para filtrar grupos.
    pub search: Option<String>,
    /// Filtrar apenas grupos de nível superior (sem pai).
    pub top_level_only: Option<bool>,
    /// Número da página para recuperação.
    pub page: Option<u32>,
    /// Número de itens por página.
    pub per_page: Option<u32>,
}
