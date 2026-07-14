use serde::{Deserialize, Serialize};

/// Convite no GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Invitation {
    /// Campo `pub email`.
    pub email: String,
    /// Campo `pub access_level`.
    pub access_level: u32,
    /// Campo `pub created_at`.
    pub created_at: Option<String>,
    /// Campo `pub expires_at`.
    pub expires_at: Option<String>,
    /// Campo `pub invite_email`.
    pub invite_email: Option<String>,
}

/// Payload para criar convite.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateInvitationPayload {
    /// Campo `pub email`.
    pub email: String,
    /// Campo `pub access_level`.
    pub access_level: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Campo `pub expires_at`.
    pub expires_at: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
/// Tipo `InvitationFilter`.
pub struct InvitationFilter {
    /// Campo `pub page`.
    pub page: Option<u32>,
    /// Campo `pub per_page`.
    pub per_page: Option<u32>,
}
