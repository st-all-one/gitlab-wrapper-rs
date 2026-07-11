use serde::{Deserialize, Serialize};
use crate::types::base::*;

/// Resposta da API GitLab representando um usuário.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct User {
    /// ID único do usuário.
    pub id: GitLabId,
    /// Nome de usuário.
    pub username: String,
    /// Nome completo do usuário.
    pub name: String,
    /// Estado da conta do usuário ("active", "blocked", etc.).
    pub state: Option<String>,
    /// URL do avatar do usuário.
    pub avatar_url: Option<String>,
    /// URL do perfil do usuário no GitLab.
    pub web_url: Option<String>,
    /// E-mail do usuário (pode estar oculto).
    pub email: Option<String>,
    /// Data de criação da conta no formato ISO 8601.
    pub created_at: Option<String>,
    /// Biografia do usuário.
    pub bio: Option<String>,
    /// Localização do usuário.
    pub location: Option<String>,
    /// E-mail público do usuário.
    pub public_email: Option<String>,
    /// Nome de usuário no Skype.
    pub skype: Option<String>,
    /// Nome de usuário no LinkedIn.
    pub linkedin: Option<String>,
    /// Nome de usuário no Twitter.
    pub twitter: Option<String>,
    /// URL do site pessoal do usuário.
    pub website_url: Option<String>,
    /// Organização à qual o usuário pertence.
    pub organization: Option<String>,
    /// Data do último login no formato ISO 8601.
    pub last_sign_in_at: Option<String>,
    /// Data de confirmação da conta no formato ISO 8601.
    pub confirmed_at: Option<String>,
    /// Data da última atividade no formato ISO 8601.
    pub last_activity_on: Option<String>,
    /// Identidades associadas ao usuário (provedores externos).
    pub identities: Option<Vec<UserIdentity>>,
    /// Indica se o usuário é administrador.
    pub is_admin: Option<bool>,
    /// Indica se o usuário é um bot.
    pub is_bot: Option<bool>,
    /// Indica se o usuário é externo.
    pub is_external: Option<bool>,
    /// Observação interna sobre o usuário.
    pub note: Option<String>,
}

/// Resposta da API GitLab representando uma identidade de provedor externo
/// associada a um usuário.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserIdentity {
    /// Nome do provedor de identidade (ex.: "ldap", "saml").
    pub provider: Option<String>,
    /// Identificador externo do usuário no provedor.
    pub extern_uid: Option<String>,
}

/// Resposta da API GitLab representando o status de um usuário
/// (emoji e mensagem de status).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserStatus {
    /// Emoji do status (ex.: "smiley", "coffee").
    pub emoji: Option<String>,
    /// Mensagem de status.
    pub message: Option<String>,
    /// Mensagem de status em HTML.
    pub message_html: Option<String>,
}

/// Resposta da API GitLab representando as preferências de um usuário.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserPreferences {
    /// ID das preferências.
    pub id: Option<GitLabId>,
    /// ID do usuário ao qual as preferências pertencem.
    pub user_id: Option<GitLabId>,
    /// Indica se os diffs são exibidos arquivo por arquivo.
    pub view_diffs_file_by_file: Option<bool>,
    /// Indica se espaços em branco são exibidos nos diffs.
    pub show_whitespace_in_diffs: Option<bool>,
    /// Indica se o Web IDE legado deve ser usado.
    pub use_legacy_web_ide: Option<bool>,
}

/// Payload para criar um usuário na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateUserPayload {
    /// E-mail do usuário.
    pub email: String,
    /// Nome de usuário.
    pub username: String,
    /// Nome completo do usuário.
    pub name: String,
    /// Senha do usuário.
    pub password: String,
    /// Indica se a confirmação por e-mail deve ser pulada.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_confirmation: Option<bool>,
}

/// Payload para atualizar um usuário na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateUserPayload {
    /// Novo e-mail do usuário.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Novo nome de usuário.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Novo nome completo do usuário.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Nova senha do usuário.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

/// Filtros disponíveis para listar usuários. Use `..Default::default()` para valores padrão.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UserFilter {
    /// Nome de usuário exato para filtrar.
    pub username: Option<String>,
    /// Termo de busca para filtrar usuários.
    pub search: Option<String>,
    /// Filtrar apenas usuários ativos.
    pub active: Option<bool>,
    /// Filtrar apenas usuários bloqueados.
    pub blocked: Option<bool>,
    /// Número da página para recuperação.
    pub page: Option<u32>,
    /// Número de itens por página.
    pub per_page: Option<u32>,
}
