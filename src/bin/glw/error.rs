//! Erros da CLI e mapeamento para códigos de saída.

use std::fmt;

use gitlab_wrapper::GitLabError;

/// Código de saída da CLI, conforme contrato.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitCode {
    /// Operação concluída com sucesso.
    Success = 0,
    /// Uso inválido ou configuração inválida.
    Usage = 2,
    /// Falha na API GitLab.
    Api = 3,
    /// Falha de leitura/escrita de arquivos ou stdin/stdout.
    Io = 4,
}

impl From<ExitCode> for std::process::ExitCode {
    fn from(code: ExitCode) -> Self {
        Self::from(code as u8)
    }
}

/// Erro apresentado ao usuário pela CLI.
#[derive(Debug)]
pub struct CliError {
    /// Categoria do erro, usada para determinar o código de saída.
    pub kind: ErrorKind,
    /// Mensagem legível, sem dados sensíveis.
    pub message: String,
}

/// Classificação interna de um erro da CLI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    /// Argumentos ou configuração inválidos.
    Usage,
    /// Falha de API (com status e correlation ID quando disponíveis).
    Api,
    /// Falha de IO.
    Io,
}

impl CliError {
    /// Cria um erro de uso/configuração.
    pub fn usage(message: impl Into<String>) -> Self {
        Self {
            kind: ErrorKind::Usage,
            message: message.into(),
        }
    }

    /// Cria um erro de IO.
    pub fn io(message: impl Into<String>) -> Self {
        Self {
            kind: ErrorKind::Io,
            message: message.into(),
        }
    }

    /// Cria um erro de API com detalhes extraídos do erro da biblioteca.
    pub fn api(error: &GitLabError) -> Self {
        let mut parts = vec![error.to_string()];
        if let GitLabError::Api {
            status,
            detail,
            instance,
            category,
            context,
            ..
        } = error
        {
            let reason = category.description();
            parts = vec![format!("GitLab API error {status} ({reason}): {detail}")];
            if !instance.is_empty() {
                parts.push(format!("instance={instance}"));
            }
            if let Some(operation) = context.operation.as_deref() {
                parts.push(format!("operation={operation}"));
            }
            if let Some(api_errors) = context.api_errors.as_deref() {
                for message in api_errors {
                    parts.push(format!("api_error={message}"));
                }
            }
        } else {
            match error {
                GitLabError::Config(message) => {
                    return Self {
                        kind: ErrorKind::Usage,
                        message: format!("configuration error: {message}"),
                    };
                }
                GitLabError::Url(message) => {
                    return Self {
                        kind: ErrorKind::Usage,
                        message: format!("invalid URL: {message}"),
                    };
                }
                GitLabError::Serialization(source) => {
                    return Self {
                        kind: ErrorKind::Usage,
                        message: format!("serialization error: {source}"),
                    };
                }
                GitLabError::RateLimited { retry_after, .. } => {
                    parts = vec![format!(
                        "GitLab API rate limit exceeded (retry after {:?})",
                        retry_after
                    )];
                }
                GitLabError::Timeout { duration, .. } => {
                    parts = vec![format!("GitLab API timeout after {duration:?}")];
                }
                GitLabError::Http(source) => {
                    parts = vec![format!("GitLab HTTP error: {source}")];
                }
                GitLabError::Api { .. } => {}
            }
        }
        Self {
            kind: ErrorKind::Api,
            message: parts.join(" | "),
        }
    }

    /// Renderiza o erro para stderr com um prefixo estável.
    pub fn render(&self) -> String {
        let label = match self.kind {
            ErrorKind::Usage => "error",
            ErrorKind::Api => "api error",
            ErrorKind::Io => "io error",
        };
        format!("glw: {label}: {}", self.message)
    }

    /// Retorna o código de saída associado.
    pub const fn exit_code(&self) -> ExitCode {
        match self.kind {
            ErrorKind::Usage => ExitCode::Usage,
            ErrorKind::Api => ExitCode::Api,
            ErrorKind::Io => ExitCode::Io,
        }
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.render())
    }
}

impl std::error::Error for CliError {}

impl From<GitLabError> for CliError {
    fn from(error: GitLabError) -> Self {
        Self::api(&error)
    }
}

impl From<serde_json::Error> for CliError {
    fn from(error: serde_json::Error) -> Self {
        Self::usage(format!("invalid JSON: {error}"))
    }
}

impl From<std::io::Error> for CliError {
    fn from(error: std::io::Error) -> Self {
        Self::io(format!("{error}"))
    }
}
