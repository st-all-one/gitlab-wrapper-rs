use std::ops::Deref;
use std::sync::Arc;

use crate::core::config::{GitLabConfig, ResolvedConfig};
use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;

mod resources;
use resources::ResourceGroup;

#[derive(Debug)]
pub struct GitLabClient {
    pub(crate) config: ResolvedConfig,
    inner: ResourceGroup,
}

impl Deref for GitLabClient {
    type Target = ResourceGroup;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl GitLabClient {
    pub fn config(&self) -> &ResolvedConfig {
        &self.config
    }

    pub fn new(config: GitLabConfig) -> Result<Self, GitLabError> {
        if config.base_url.is_empty() {
            return Err(GitLabError::Config("base_url is required".into()));
        }

        let resolved = ResolvedConfig::resolve(config);
        let config_ = resolved.clone();
        let http = Arc::new(HttpClient::new(resolved));

        log::info!(target: "gitlab_wrapper", "GitLabClient initialized");
        Ok(Self {
            config: config_,
            inner: ResourceGroup::new(http),
        })
    }
}
