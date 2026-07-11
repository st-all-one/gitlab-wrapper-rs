//! Módulo do cliente GitLab — define o [`GitLabClient`] e sua fábrica.
//!
//! Este módulo contém a estrutura principal do cliente, que consolida as configurações
//! resolvidas e expõe acesso a todos os recursos da API através da implementação de
//! `Deref` para [`ResourceGroup`].

use std::ops::Deref;
use std::sync::Arc;

use crate::core::config::{GitLabConfig, ResolvedConfig};
use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;

mod resources;
use resources::ResourceGroup;

/// Cliente principal para interagir com a API REST do GitLab.
///
/// Agrupa a configuração resolvida e delega o acesso aos recursos (projetos, issues,
/// merge requests, etc.) através de `Deref` para [`ResourceGroup`].
#[derive(Debug)]
pub struct GitLabClient {
    pub(crate) config: ResolvedConfig,
    inner: ResourceGroup,
}

/// Implementação de `Deref` para acesso transparente aos campos e métodos de [`ResourceGroup`].
///
/// Permite chamar métodos diretamente no [`GitLabClient`] (ex.: `client.projects.list(...)`)
/// sem acessar explicitamente o campo interno.
impl Deref for GitLabClient {
    type Target = ResourceGroup;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl GitLabClient {
    /// Retorna uma referência à configuração resolvida do cliente.
    ///
    /// ## Returns
    /// `&ResolvedConfig` — configuração com valores padrão já aplicados.
    #[must_use]
    pub fn config(&self) -> &ResolvedConfig {
        &self.config
    }

    /// Cria uma nova instância de [`GitLabClient`] a partir da configuração fornecida.
    ///
    /// Resolve valores padrão (`timeout`, `max_rps`, `auth_method`) e inicializa o
    /// cliente HTTP subjacente com rate limiting e retry automático.
    ///
    /// ## Params
    /// - `config`: Configuração fornecida pelo usuário.
    ///
    /// ## Returns
    /// `Result<Self, GitLabError>` — cliente inicializado ou erro de configuração.
    ///
    /// ## Errors
    /// Retorna `GitLabError::Config` se `base_url` estiver vazia.
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
