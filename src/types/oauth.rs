use serde::{Deserialize, Serialize};

/// Resposta da API GitLab representando um token de acesso OAuth 2.0.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthTokenResponse {
    /// Token de acesso.
    pub access_token: String,
    /// Tipo do token (ex: Bearer).
    pub token_type: String,
    /// Token de atualização (refresh).
    pub refresh_token: Option<String>,
    /// Escopo do token.
    pub scope: String,
    /// Timestamp de criação do token.
    pub created_at: u64,
    /// Tempo de expiração em segundos.
    pub expires_in: u64,
}

/// Resposta da API GitLab representando uma autorização de dispositivo (device flow).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceAuthResponse {
    /// Código de dispositivo para verificação.
    pub device_code: String,
    /// Código de usuário para verificação.
    pub user_code: String,
    /// URI de verificação.
    pub verification_uri: String,
    /// URI de verificação completa com o código.
    pub verification_uri_complete: Option<String>,
    /// Tempo de expiração em segundos.
    pub expires_in: u64,
    /// Intervalo mínimo entre polling em segundos.
    pub interval: u64,
}

/// Resposta da API GitLab representando um erro de OAuth.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthErrorResponse {
    /// Código do erro.
    pub error: String,
    /// Descrição detalhada do erro.
    pub error_description: Option<String>,
}
