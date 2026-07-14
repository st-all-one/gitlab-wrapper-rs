//! Tipos de erro do cliente GitLab.
//!
//! Define [`ErrorCategory`] (classificação semântica de erros da API),
//! [`ErrorContext`] (metadados adicionais sobre a falha) e [`GitLabError`]
//! (enum principal de erro com suporte a `thiserror`).

use std::fmt;
use uuid::Uuid;

/// Categoria de erro da API do GitLab.
///
/// Mapeia códigos de status HTTP para categorias semânticas. Enumeração não exaustiva
/// (`#[non_exhaustive]`) — novos casos podem ser adicionados em versões futuras sem
/// causar breaking change.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorCategory {
    /// Falha de autenticação — token ausente ou inválido (HTTP 401).
    AuthenticationFailed,
    /// Acesso negado — permissão insuficiente para o recurso (HTTP 403).
    AuthorizationDenied,
    /// Recurso solicitado não encontrado (HTTP 404).
    ResourceNotFound,
    /// Erro de validação nos parâmetros enviados (HTTP 422).
    ValidationError,
    /// Conflito com o estado atual do recurso (HTTP 409).
    Conflict,
    /// Limite de taxa de requisições excedido (HTTP 429).
    RateLimited,
    /// Conteúdo detectado como spam (HTTP 400).
    SpamDetected,
    /// Recurso não modificado desde a última consulta (HTTP 304).
    NotModified,
    /// Tempo limite da requisição excedido (HTTP 504).
    Timeout,
    /// Erro de rede — conexão falhou (HTTP 503).
    NetworkError,
    /// Erro ao interpretar a resposta da API (HTTP 500).
    ParseError,
    /// Erro interno não classificado (HTTP 500).
    InternalError,
}

impl ErrorCategory {
    /// Converte um código de status HTTP para uma [`ErrorCategory`], se houver
    /// correspondência conhecida.
    ///
    /// ## Params
    /// - `status`: Código de status HTTP.
    ///
    /// ## Returns
    /// `Option<ErrorCategory>` — `None` se o status não estiver mapeado.
    pub fn from_status(status: u16) -> Option<Self> {
        match status {
            304 => Some(Self::NotModified),
            400 => Some(Self::SpamDetected),
            401 => Some(Self::AuthenticationFailed),
            403 => Some(Self::AuthorizationDenied),
            404 => Some(Self::ResourceNotFound),
            409 => Some(Self::Conflict),
            422 => Some(Self::ValidationError),
            429 => Some(Self::RateLimited),
            500 => Some(Self::InternalError),
            503 => Some(Self::NetworkError),
            504 => Some(Self::Timeout),
            _ => None,
        }
    }

    /// Retorna o código de status HTTP correspondente a esta categoria.
    ///
    /// ## Returns
    /// `u16` — código de status HTTP.
    pub fn http_status(&self) -> u16 {
        match self {
            Self::NotModified => 304,
            Self::SpamDetected => 400,
            Self::AuthenticationFailed => 401,
            Self::AuthorizationDenied => 403,
            Self::ResourceNotFound => 404,
            Self::Conflict => 409,
            Self::ValidationError => 422,
            Self::RateLimited => 429,
            Self::InternalError | Self::ParseError => 500,
            Self::NetworkError => 503,
            Self::Timeout => 504,
        }
    }

    /// Retorna uma descrição textual (slug) da categoria de erro.
    ///
    /// ## Returns
    /// `&'static str` — identificador em formato kebab-case.
    pub fn description(&self) -> &'static str {
        match self {
            Self::AuthenticationFailed => "authentication-failed",
            Self::AuthorizationDenied => "authorization-denied",
            Self::ResourceNotFound => "resource-not-found",
            Self::ValidationError => "validation-error",
            Self::Conflict => "conflict",
            Self::RateLimited => "rate-limited",
            Self::SpamDetected => "spam-detected",
            Self::NotModified => "not-modified",
            Self::Timeout => "timeout",
            Self::NetworkError => "network-error",
            Self::ParseError => "parse-error",
            Self::InternalError => "internal-error",
        }
    }
}

/// Implementação de `Display` que exibe a descrição textual da categoria.
impl fmt::Display for ErrorCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.description())
    }
}

/// Metadados adicionais associados a um erro do GitLab.
///
/// Contém informações contextuais como a operação que falhou, o código de status
/// HTTP retornado, mensagens de erro da API e o corpo bruto da resposta.
#[derive(Debug, Clone, Default)]
pub struct ErrorContext {
    /// Nome da operação que gerou o erro (ex.: `"projects.list"`).
    pub operation: Option<String>,
    /// Código de status HTTP retornado pela API.
    pub http_status: Option<u16>,
    /// Lista de mensagens de erro retornadas pela API.
    pub api_errors: Option<Vec<String>>,
    /// Corpo da resposta bruta recebido da API.
    pub response_body: Option<String>,
}

/// Erro principal da biblioteca gitlab-wrapper-rs.
///
/// Usa `thiserror` para gerar automaticamente as mensagens de erro e a implementação
/// de `Display` e `Error`. Inclui variantes para erros da API, erros de transporte
/// HTTP, rate limiting, timeout, serialização JSON, URL inválida e configuração.
#[derive(Debug, thiserror::Error)]
pub enum GitLabError {
    /// Erro retornado pela API do GitLab com categoria, status, detalhes e contexto.
    #[error("GitLab API error: {detail} (category: {category})")]
    Api {
        /// Categoria semântica do erro.
        category: ErrorCategory,
        /// Código de status HTTP retornado.
        status: u16,
        /// Mensagem descritiva detalhada do erro.
        detail: String,
        /// Identificador único (UUID v7) para correlação e rastreamento.
        instance: String,
        /// Metadados contextuais adicionais sobre a falha.
        context: Box<ErrorContext>,
    },

    /// Erro na camada de transporte HTTP (conexão, TLS, resolução DNS, etc.).
    #[error("HTTP error: {0}")]
    Http(reqwest::Error),

    /// Limite de taxa excedido — a requisição deve ser repetida após o tempo indicado.
    #[error("Rate limit exceeded, retry after {retry_after:?}")]
    RateLimited {
        /// Tempo recomendado (em segundos) para aguardar antes de tentar novamente.
        retry_after: Option<u64>,
        /// Metadados contextuais adicionais sobre a falha.
        context: Box<ErrorContext>,
    },

    /// Tempo limite da requisição excedido.
    #[error("Timeout after {duration:?}")]
    Timeout {
        /// Duração do timeout configurado para a requisição.
        duration: std::time::Duration,
        /// Metadados contextuais adicionais sobre a falha.
        context: Box<ErrorContext>,
    },

    /// Erro ao interpretar a URL fornecida (mal formatada).
    #[error("URL parse error: {0}")]
    Url(String),

    /// Erro de serialização ou desserialização JSON (via `serde_json`).
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Erro de configuração (ex.: campo obrigatório ausente ou inválido).
    #[error("Configuration error: {0}")]
    Config(String),
}

impl GitLabError {
    /// Constrói um erro da API com categoria, status, detalhe e contexto.
    ///
    /// Gera automaticamente um identificador único (`instance`) usando UUID v7
    /// para correlação e rastreamento.
    ///
    /// ## Params
    /// - `category`: Categoria semântica do erro.
    /// - `status`: Código de status HTTP.
    /// - `detail`: Mensagem descritiva do erro.
    /// - `context`: Metadados contextuais da operação que falhou.
    ///
    /// ## Returns
    /// `GitLabError` — variante `Api` preenchida.
    pub fn api(
        category: ErrorCategory,
        status: u16,
        detail: impl Into<String>,
        context: ErrorContext,
    ) -> Self {
        let instance = Uuid::new_v7(uuid::Timestamp::now(uuid::NoContext)).to_string();
        Self::Api { category, status, detail: detail.into(), instance, context: Box::new(context) }
    }

    /// Extrai a [`ErrorCategory`] do erro, se aplicável.
    ///
    /// ## Returns
    /// `Option<ErrorCategory>` — categoria classificada, ou `None` se o erro não
    /// for classificado em nenhuma categoria conhecida.
    pub fn category(&self) -> Option<ErrorCategory> {
        match self {
            Self::Api { category, .. } => Some(*category),
            Self::RateLimited { .. } => Some(ErrorCategory::RateLimited),
            Self::Timeout { .. } => Some(ErrorCategory::Timeout),
            Self::Http(e) if e.is_timeout() => Some(ErrorCategory::Timeout),
            Self::Http(e) if e.is_connect() => Some(ErrorCategory::NetworkError),
            Self::Http(e) if e.is_status() => {
                e.status().and_then(|s| ErrorCategory::from_status(s.as_u16()))
            }
            _ => None,
        }
    }
}

/// Conversão de `reqwest::Error` para `GitLabError`.
///
/// Erros de timeout são convertidos para a variante `GitLabError::Timeout` com
/// duração padrão de 30 segundos. Demais erros HTTP são encapsulados em
/// `GitLabError::Http`.
impl From<reqwest::Error> for GitLabError {
    fn from(e: reqwest::Error) -> Self {
        tracing::error!(target: "gitlab_wrapper::http", "HTTP error: {}", e);
        if e.is_timeout() {
            Self::Timeout {
                duration: std::time::Duration::from_secs(30),
                context: Box::new(ErrorContext::default()),
            }
        } else {
            Self::Http(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_category_from_status() {
        assert_eq!(ErrorCategory::from_status(304), Some(ErrorCategory::NotModified));
        assert_eq!(ErrorCategory::from_status(401), Some(ErrorCategory::AuthenticationFailed));
        assert_eq!(ErrorCategory::from_status(404), Some(ErrorCategory::ResourceNotFound));
        assert_eq!(ErrorCategory::from_status(429), Some(ErrorCategory::RateLimited));
        assert_eq!(ErrorCategory::from_status(200), None);
    }

    #[test]
    fn test_error_category_http_status_roundtrip() {
        for cat in &[
            ErrorCategory::AuthenticationFailed,
            ErrorCategory::ResourceNotFound,
            ErrorCategory::RateLimited,
            ErrorCategory::Timeout,
        ] {
            let status = cat.http_status();
            let back = ErrorCategory::from_status(status);
            assert_eq!(back, Some(*cat), "roundtrip failed for {cat}");
        }
    }

    #[test]
    fn test_error_category_display() {
        assert_eq!(ErrorCategory::AuthenticationFailed.to_string(), "authentication-failed");
        assert_eq!(ErrorCategory::ResourceNotFound.to_string(), "resource-not-found");
    }

    #[test]
    fn test_api_error_creation() {
        let err = GitLabError::api(
            ErrorCategory::ResourceNotFound,
            404,
            "test error",
            ErrorContext {
                operation: Some("test.op".into()),
                http_status: Some(404),
                ..Default::default()
            },
        );
        match err {
            GitLabError::Api { category, status, detail, .. } => {
                assert_eq!(category, ErrorCategory::ResourceNotFound);
                assert_eq!(status, 404);
                assert_eq!(detail, "test error");
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn test_api_error_has_instance_id() {
        let err =
            GitLabError::api(ErrorCategory::InternalError, 500, "fail", ErrorContext::default());
        match err {
            GitLabError::Api { instance, .. } => {
                assert!(!instance.is_empty(), "instance UUID must not be empty");
                assert!(instance.contains('-'), "instance must be UUID format");
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn test_category_extraction() {
        let api_err =
            GitLabError::api(ErrorCategory::RateLimited, 429, "too many", ErrorContext::default());
        assert_eq!(api_err.category(), Some(ErrorCategory::RateLimited));

        let config_err = GitLabError::Config("bad config".into());
        assert_eq!(config_err.category(), None);
    }
}
