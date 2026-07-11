//! Constantes padrão usadas em toda a biblioteca.
//!
//! Define valores como timeout padrão para requisições, limite de taxa (RPS),
//! versão da API do GitLab, nome da biblioteca e valor do cabeçalho `User-Agent`.

use std::time::Duration;

/// Nome da biblioteca, usado em logs e identificação.
pub const LIB_NAME: &str = "gitlab-wrapper-rs";
/// Namespace usado para mensagens de log da biblioteca.
pub const LOG_NAMESPACE: &str = "gitlab_wrapper";
/// Tempo limite padrão para requisições HTTP (30 segundos).
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);
/// Máximo de requisições por segundo padrão (10 RPS).
pub const DEFAULT_MAX_RPS: u32 = 10;
/// Número padrão de itens por página em requisições paginadas.
pub const DEFAULT_PER_PAGE: u32 = 20;
/// Versão da API do GitLab utilizada (v4).
pub const API_VERSION: &str = "v4";
/// Valor do cabeçalho `User-Agent` enviado em todas as requisições.
pub const USER_AGENT_VALUE: &str = concat!("gitlab-wrapper-rs/", env!("CARGO_PKG_VERSION"));
