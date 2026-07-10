use std::fmt;
use std::time::Duration;

use super::constants::{DEFAULT_TIMEOUT, DEFAULT_MAX_RPS};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AuthMethod {
    Header,
    #[default]
    Bearer,
}

#[derive(Debug, Clone, Default)]
pub struct GitLabConfig {
    pub base_url: String,
    pub token: Option<String>,
    pub auth_method: Option<AuthMethod>,
    pub sudo: Option<String>,
    pub timeout: Option<Duration>,
    pub max_rps: Option<u32>,
}

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

#[derive(Clone)]
pub struct ResolvedConfig {
    pub base_url: String,
    pub token: Option<String>,
    pub auth_method: AuthMethod,
    pub sudo: Option<String>,
    pub timeout: Duration,
    pub max_rps: u32,
}

impl ResolvedConfig {
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
