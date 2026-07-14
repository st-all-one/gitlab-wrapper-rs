//! Configuração do cliente GitLab.
//!
//! Define [`AuthMethod`] (método de autenticação), [`GitLabConfig`] (configuração
//! fornecida pelo usuário) e [`ResolvedConfig`] (configuração com valores padrão aplicados).

use std::fmt;
use std::time::Duration;

use super::constants::{DEFAULT_MAX_RPS, DEFAULT_TIMEOUT};

/// Método de autenticação usado nas requisições à API do GitLab.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AuthMethod {
    /// Envia o token no cabeçalho `Private-Token`.
    Header,
    /// Envia o token no cabeçalho `Authorization: Bearer <token>` (padrão).
    #[default]
    Bearer,
}

/// Configuração fornecida pelo usuário para inicializar o [`crate::client::GitLabClient`].
///
/// Campos opcionais recebem valores padrão durante a resolução via
/// [`ResolvedConfig::resolve`].
#[derive(Clone, Default)]
pub struct GitLabConfig {
    /// URL base da instância GitLab (ex.: `https://gitlab.com`).
    pub base_url: String,
    /// Token de acesso pessoal ou de aplicação.
    pub token: Option<String>,
    /// Método de autenticação (`Header` ou `Bearer`). Usa `Bearer` como padrão.
    pub auth_method: Option<AuthMethod>,
    /// Modo sudo — usuário em nome do qual as ações serão executadas.
    pub sudo: Option<String>,
    /// Timeout máximo para requisições HTTP.
    pub timeout: Option<Duration>,
    /// Máximo de requisições por segundo (rate limiting).
    pub max_rps: Option<u32>,
}

/// Implementação de `Debug` que oculta o token (exibe `"REDACTED"`).
impl fmt::Debug for GitLabConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GitLabConfig")
            .field("base_url", &self.base_url)
            .field("token", &"REDACTED")
            .field("auth_method", &self.auth_method)
            .field("sudo", &self.sudo)
            .field("timeout", &self.timeout)
            .field("max_rps", &self.max_rps)
            .finish()
    }
}

/// Implementação de `Debug` que oculta o token (exibe `"REDACTED"`).
impl fmt::Debug for ResolvedConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ResolvedConfig")
            .field("base_url", &self.base_url)
            .field("token", &"REDACTED")
            .field("auth_method", &self.auth_method)
            .field("sudo", &self.sudo)
            .field("timeout", &self.timeout)
            .field("max_rps", &self.max_rps)
            .finish()
    }
}

/// Configuração resolvida com valores padrão aplicados.
///
/// Produzida por [`ResolvedConfig::resolve`] a partir de [`GitLabConfig`].
/// Todos os campos opcionais da configuração de entrada são preenchidos com
/// seus valores padrão definidos em [`crate::core::constants`].
#[derive(Clone)]
pub struct ResolvedConfig {
    /// URL base da instância GitLab (sem barra final).
    pub base_url: String,
    /// Token de acesso pessoal ou de aplicação.
    pub token: Option<String>,
    /// Método de autenticação resolvido (padrão: `Bearer`).
    pub auth_method: AuthMethod,
    /// Modo sudo opcional.
    pub sudo: Option<String>,
    /// Timeout máximo para requisições (padrão: 30 segundos).
    pub timeout: Duration,
    /// Máximo de requisições por segundo (padrão: 10 RPS).
    pub max_rps: u32,
}

impl ResolvedConfig {
    /// Resolve uma [`GitLabConfig`] aplicando valores padrão aos campos não especificados.
    ///
    /// A URL base é normalizada (remove a barra final). Campos com valor `None` recebem
    /// os valores padrão definidos em [`crate::core::constants`].
    ///
    /// ## Params
    /// - `config`: Configuração fornecida pelo usuário.
    ///
    /// ## Returns
    /// `ResolvedConfig` — configuração com todos os campos preenchidos.
    pub fn resolve(config: GitLabConfig) -> Self {
        let base_url = config.base_url.trim_end_matches('/').to_string();
        Self {
            base_url,
            token: config.token,
            auth_method: config.auth_method.unwrap_or_default(),
            sudo: config.sudo,
            timeout: config.timeout.unwrap_or(DEFAULT_TIMEOUT),
            max_rps: config.max_rps.unwrap_or(DEFAULT_MAX_RPS),
        }
    }
}
