//! Módulo de utilitários OAuth 2.0 para GitLab.
//!
//! Fornece suporte aos fluxos de autorização OAuth: código de autorização
//! (com suporte a PKCE), Device Grant e renovação/revogação de tokens.
//!
//! # Limitação conhecida
//!
//! As funções deste módulo criam seu próprio cliente HTTP via `LazyLock<Client>`
//! e **não passam pelo rate limiter** do `HttpClient`. Na prática isso é aceitável
//! porque fluxos OAuth são chamados esporadicamente. Para rate limiting estrito,
//! faça as requisições HTTP manualmente através do `HttpClient`.

mod auth_code;
mod device_grant;
mod pkce;
mod refresh;

pub use auth_code::{
    AuthCodeUrlOptions, ExchangeCodeOptions, authorization_code_url, exchange_authorization_code,
};
pub use device_grant::{
    DeviceAuthOptions, GetTokenOptions, PollTokenOptions, get_token, poll_for_token,
    request_device_authorization,
};
pub use pkce::{generate_code_challenge, generate_code_verifier};
pub use refresh::{RefreshTokenOptions, RevokeTokenOptions, refresh_token, revoke_token};

pub use crate::types::{DeviceAuthResponse, OAuthErrorResponse, OAuthTokenResponse};
