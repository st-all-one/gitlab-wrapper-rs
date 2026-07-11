use serde::{Deserialize, Serialize};
use crate::types::base::*;

/// Resposta da API GitLab representando um membro de projeto/grupo.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Member {
    /// ID do membro no GitLab.
    pub id: GitLabId,
    /// Nome de usuário do membro.
    pub username: String,
    /// Nome completo do membro.
    pub name: String,
    /// Estado do membro (ex: "active", "blocked").
    pub state: Option<String>,
    /// URL do avatar do membro.
    pub avatar_url: Option<String>,
    /// URL do perfil do membro no GitLab.
    pub web_url: Option<String>,
    /// Nível de acesso do membro no projeto/grupo.
    pub access_level: Option<u32>,
    /// Data de expiração do acesso (formato ISO 8601).
    pub expires_at: Option<String>,
    /// Data de criação do membro (formato ISO 8601).
    pub created_at: Option<String>,
    /// Informações do usuário que adicionou o membro.
    pub created_by: Option<AuthorInfo>,
}

/// Payload para adicionar um membro a um projeto/grupo na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AddMemberPayload {
    /// ID do usuário a ser adicionado como membro.
    pub user_id: GitLabId,
    /// Nível de acesso do membro (ex: 10 para Guest, 20 para Reporter, etc.).
    pub access_level: u32,
    /// Data de expiração do acesso (formato ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

/// Payload para atualizar um membro de projeto/grupo na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateMemberPayload {
    /// Novo nível de acesso do membro (ex: 10 para Guest, 20 para Reporter, etc.).
    pub access_level: u32,
    /// Nova data de expiração do acesso (formato ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}
