//! Módulo principal de configuração, erros e constantes.
//!
//! Contém os tipos fundamentais usados por toda a biblioteca: [`config::GitLabConfig`],
//! [`config::ResolvedConfig`], [`errors::GitLabError`], [`errors::ErrorCategory`],
//! e [`constants`] com valores padrão.

/// Configuração do cliente (métodos de autenticação, timeouts, etc.).
pub mod config;
/// Constantes padrão (timeout, rate limit, versão da API, etc.).
pub mod constants;
/// Tipos de erro (categorias, contexto e enum principal de erro).
pub mod errors;
