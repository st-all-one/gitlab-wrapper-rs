use crate::types::base::GitLabId;
use serde::{Deserialize, Serialize};

/// Tag protegida no GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ProtectedTag {
    /// ID da regra de proteção.
    pub id: GitLabId,
    /// Nome ou padrão da tag (ex.: `"v*"`, `"release-*"`).
    pub name: String,
    /// Níveis de acesso permitidos para criar esta tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_access_levels: Option<Vec<ProtectedTagAccessLevel>>,
}

/// Nível de acesso para criação de tag protegida.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ProtectedTagAccessLevel {
    /// Valor numérico do nível de acesso.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_level: Option<u32>,
    /// Descrição textual do nível de acesso.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_level_description: Option<String>,
    /// ID do grupo com acesso.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<GitLabId>,
    /// ID do usuário com acesso.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<GitLabId>,
}

/// Payload para proteger uma tag.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ProtectTagPayload {
    /// Nome ou padrão da tag a proteger.
    pub name: String,
    /// Nível de acesso mínimo para criar (`0` = ninguém, `30` = developer, `40` = maintainer).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_access_level: Option<u32>,
    /// ID do usuário com permissão.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<GitLabId>,
    /// ID do grupo com permissão.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<GitLabId>,
}

/// Filtros para listar tags protegidas.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ProtectedTagFilter {
    /// Número da página.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Quantidade de itens por página.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>,
    /// Texto para buscar tags protegidas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
}
