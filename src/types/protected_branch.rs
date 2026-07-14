use serde::{Deserialize, Serialize};

use crate::types::base::GitLabId;

/// Branch protegido no GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ProtectedBranch {
    /// ID da regra de proteção.
    pub id: GitLabId,
    /// Nome ou padrão do branch (ex.: `"main"`, `"release-*"`).
    pub name: String,
    /// Níveis de acesso permitidos para fazer push.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_access_levels: Option<Vec<AccessLevelInfo>>,
    /// Níveis de acesso permitidos para fazer merge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_access_levels: Option<Vec<AccessLevelInfo>>,
    /// Se força de push é permitida.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_force_push: Option<bool>,
    /// Se aprovação de code owner é obrigatória.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_owner_approval_required: Option<bool>,
}

/// Informações de nível de acesso para uma regra de proteção.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AccessLevelInfo {
    /// Valor numérico do nível de acesso.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_level: Option<u32>,
    /// Descrição textual do nível de acesso.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_level_description: Option<String>,
    /// ID do usuário com acesso especial.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<GitLabId>,
    /// ID do grupo com acesso especial.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<GitLabId>,
}

/// Payload para proteger um branch.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ProtectBranchPayload {
    /// Nome ou padrão do branch a proteger.
    pub name: String,
    /// Nível de acesso mínimo para push (`0` = ninguém, `30` = developer, `40` = maintainer).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_access_level: Option<u32>,
    /// Nível de acesso mínimo para merge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_access_level: Option<u32>,
    /// Se força de push é permitida.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_force_push: Option<bool>,
    /// Se code owner approval é obrigatório.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_owner_approval_required: Option<bool>,
    /// ID do usuário com permissão.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<GitLabId>,
    /// ID do grupo com permissão.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<GitLabId>,
}

/// Filtros para listar branches protegidos.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ProtectedBranchFilter {
    /// Número da página.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Quantidade de itens por página.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>,
    /// Texto para buscar branches protegidos.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
}
