//! Cliente GitLab wrapper — crate principal que re-exporta todos os tipos, recursos e utilitários.
//!
//! Este crate fornece um cliente HTTP assíncrono (tokio) para a API REST do GitLab v4, incluindo
//! gerenciamento de configuração, tratamento de erros, rate limiting e paginação com streams.
//! Basta configurar [`GitLabConfig`] e chamar [`GitLabClient::new`] para começar.

#![deny(unsafe_code)]
#![deny(missing_docs)]

/// Módulo central: configuração, constantes e tipos de erro.
pub mod core;
/// Cliente HTTP com rate limiting, retry e paginação.
pub mod http;
/// Estruturas de recurso para cada endpoint da API (projetos, issues, merge requests, etc.).
pub mod resources;
/// Tipos de dados serializáveis e deserializáveis da API do GitLab.
pub mod types;
/// Utilitários compartilhados (encoding de parâmetros de query, etc.).
pub mod utils;

mod client;
/// Módulo de autenticação OAuth 2.0 (PKCE, device grant, refresh token).
pub mod oauth;

/// Cliente principal para interagir com a API do GitLab.
pub use client::GitLabClient;
/// Método de autenticação e configurações resolvidas do cliente.
pub use core::config::{AuthMethod, GitLabConfig, ResolvedConfig};
/// Tipos de erro, categorias e contexto usados em toda a biblioteca.
pub use core::errors::{ErrorCategory, ErrorContext, GitLabError};
/// Informações de paginação retornadas por respostas paginadas.
pub use http::pagination::PaginationInfo;
/// Re-exportação de todos os recursos (projects, issues, MRs, etc.).
pub use resources::*;
/// Re-exportação de todos os tipos serializáveis/deserializáveis da API.
pub use types::*;
