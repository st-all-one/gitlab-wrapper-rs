//! Resolução de configuração: arquivo TOML, variáveis de ambiente e flags.

use std::path::{Path, PathBuf};
use std::time::Duration;

use gitlab_wrapper::{AuthMethod, GitLabConfig};

use crate::args::GlobalArgs;
use crate::error::CliError;
use crate::input;

/// URL padrão quando nada é configurado.
pub const DEFAULT_BASE_URL: &str = "https://gitlab.com";
/// Timeout padrão em segundos.
pub const DEFAULT_TIMEOUT_SECS: u64 = 30;
/// Limite padrão de requisições por segundo.
pub const DEFAULT_MAX_RPS: u32 = 10;

/// Configuração lida do arquivo TOML.
#[derive(Debug, Default, serde::Deserialize)]
#[serde(default, rename_all = "snake_case", deny_unknown_fields)]
pub struct ConfigFile {
    /// URL base.
    pub base_url: Option<String>,
    /// Token em texto claro.
    pub token: Option<String>,
    /// Caminho do arquivo com o token.
    pub token_file: Option<String>,
    /// Método de autenticação.
    pub auth_method: Option<String>,
    /// Usuário para sudo.
    pub sudo: Option<String>,
    /// Timeout em segundos.
    pub timeout_secs: Option<u64>,
    /// Limite de requisições por segundo.
    pub max_rps: Option<u32>,
    /// Perfis nomeados.
    pub profiles: std::collections::BTreeMap<String, Profile>,
}

/// Configuração de um perfil nomeado.
#[derive(Debug, Default, Clone, serde::Deserialize)]
#[serde(default, rename_all = "snake_case", deny_unknown_fields)]
pub struct Profile {
    /// URL base.
    pub base_url: Option<String>,
    /// Token em texto claro.
    pub token: Option<String>,
    /// Caminho do arquivo com o token.
    pub token_file: Option<String>,
    /// Método de autenticação.
    pub auth_method: Option<String>,
    /// Usuário para sudo.
    pub sudo: Option<String>,
    /// Timeout em segundos.
    pub timeout_secs: Option<u64>,
    /// Limite de requisições por segundo.
    pub max_rps: Option<u32>,
}

/// Configuração resolvida usada pelos handlers.
#[derive(Debug, Clone)]
pub struct ResolvedOptions {
    /// URL base normalizada (sem barra final).
    pub base_url: String,
    /// Token efetivo, quando fornecido.
    pub token: Option<String>,
    /// Método de autenticação efetivo.
    pub auth_method: AuthMethod,
    /// Usuário para sudo.
    pub sudo: Option<String>,
    /// Timeout em segundos.
    pub timeout_secs: u64,
    /// Limite de requisições por segundo.
    pub max_rps: u32,

    /// Perfil selecionado.
    pub profile: Option<String>,
}

impl ResolvedOptions {
    /// Constrói a [`GitLabConfig`] da biblioteca a partir das opções resolvidas.
    pub fn to_gitlab_config(&self) -> GitLabConfig {
        GitLabConfig {
            base_url: self.base_url.clone(),
            token: self.token.clone(),
            auth_method: Some(self.auth_method),
            sudo: self.sudo.clone(),
            timeout: Some(Duration::from_secs(self.timeout_secs)),
            max_rps: Some(self.max_rps),
        }
    }
}

/// Retorna o caminho padrão do arquivo de configuração.
pub fn default_config_path() -> PathBuf {
    if let Some(path) = std::env::var_os("GLW_CONFIG") {
        let path = PathBuf::from(path);
        if !path.as_os_str().is_empty() {
            return path;
        }
    }
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("glw")
        .join("config.toml")
}

/// Lê e interpreta o arquivo de configuração.
pub fn load_file(path: &Path) -> Result<Option<ConfigFile>, CliError> {
    let content = match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => {
            return Err(CliError::io(format!(
                "failed to read config '{}': {error}",
                path.display()
            )));
        }
    };
    let config: ConfigFile = toml::from_str(&content).map_err(|error| {
        CliError::usage(format!("invalid config '{}': {error}", path.display()))
    })?;
    Ok(Some(config))
}

/// Resolve a configuração efetiva aplicando a precedência:
/// CLI > variáveis de ambiente > perfil > arquivo > padrões.
pub fn resolve(args: &GlobalArgs) -> Result<ResolvedOptions, CliError> {
    let config_path = args
        .config
        .as_ref()
        .map_or_else(default_config_path, PathBuf::from);
    let file = load_file(&config_path)?;
    let profile_name = args.profile.clone().or_else(|| env_nonempty("GLW_PROFILE"));
    let (base, profile) = select_profile(file.as_ref(), profile_name.as_deref())?;
    // O perfil é mais específico que a seção base: seus valores substituem os
    // da base. Para credenciais, qualquer fonte definida no perfil substitui
    // integralmente as fontes da base (evita que um token_file base mascare
    // um token definido no perfil).
    let effective = merge_profile(&base, profile);

    let base_url = first_some([
        args.base_url.clone(),
        env_nonempty("GLW_BASE_URL"),
        env_nonempty("GITLAB_URL"),
        effective.base_url.clone(),
        Some(DEFAULT_BASE_URL.to_owned()),
    ]);
    let base_url = normalize_url(base_url.as_deref().unwrap_or(DEFAULT_BASE_URL))?;

    let token = if let Some(token) = args.token.clone() {
        Some(token)
    } else if let Some(path) = args.token_file.clone() {
        Some(read_token_file(&path)?)
    } else if let Some(token) = env_nonempty("GLW_TOKEN") {
        Some(token)
    } else if let Some(token) = env_nonempty("GITLAB_TOKEN") {
        Some(token)
    } else if let Some(path) = env_nonempty("GLW_TOKEN_FILE") {
        Some(read_token_file(&path)?)
    } else if let Some(token) = effective.token.clone() {
        Some(token)
    } else {
        effective
            .token_file
            .as_deref()
            .map(read_token_file)
            .transpose()?
    };

    let auth_method = first_some([
        args.auth_method.clone(),
        env_nonempty("GLW_AUTH_METHOD"),
        effective.auth_method.clone(),
    ]);
    let auth_method = parse_auth_method(auth_method.as_deref())?;

    let sudo = first_some([
        args.sudo.clone(),
        env_nonempty("GLW_SUDO"),
        effective.sudo.clone(),
    ]);

    let timeout_secs = first_some([
        args.timeout,
        env_nonempty("GLW_TIMEOUT").and_then(|value| value.parse::<u64>().ok()),
        effective.timeout_secs,
        Some(DEFAULT_TIMEOUT_SECS),
    ])
    .unwrap_or(DEFAULT_TIMEOUT_SECS);
    let max_rps = first_some([
        args.max_rps,
        env_nonempty("GLW_MAX_RPS").and_then(|value| value.parse::<u32>().ok()),
        effective.max_rps,
        Some(DEFAULT_MAX_RPS),
    ])
    .unwrap_or(DEFAULT_MAX_RPS);

    Ok(ResolvedOptions {
        base_url,
        token,
        auth_method,
        sudo,
        timeout_secs,
        max_rps,
        profile: profile_name,
    })
}

/// Mescla um perfil sobre a configuração base, com o perfil tendo precedência.
///
/// Credenciais são tratadas como um bloco: se o perfil define `token` ou
/// `token_file`, nenhuma credencial da base é herdada. Isso evita que um
/// `token` da seção base mascare o `token_file` de um perfil.
fn merge_profile(base: &Profile, profile: Option<&Profile>) -> Profile {
    let Some(profile) = profile else {
        return base.clone();
    };
    let has_profile_credential = profile.token.is_some() || profile.token_file.is_some();
    Profile {
        base_url: profile.base_url.clone().or_else(|| base.base_url.clone()),
        token: profile.token.clone().or_else(|| {
            (!has_profile_credential)
                .then(|| base.token.clone())
                .flatten()
        }),
        token_file: profile.token_file.clone().or_else(|| {
            (!has_profile_credential)
                .then(|| base.token_file.clone())
                .flatten()
        }),
        auth_method: profile
            .auth_method
            .clone()
            .or_else(|| base.auth_method.clone()),
        sudo: profile.sudo.clone().or_else(|| base.sudo.clone()),
        timeout_secs: profile.timeout_secs.or(base.timeout_secs),
        max_rps: profile.max_rps.or(base.max_rps),
    }
}

/// Verifica se o arquivo de configuração é válido, sem exigir token.
pub fn check(args: &GlobalArgs) -> Result<(PathBuf, ResolvedOptions), CliError> {
    let path = args
        .config
        .as_ref()
        .map_or_else(default_config_path, PathBuf::from);
    if !path.exists() {
        return Err(CliError::usage(format!(
            "config file not found: {}",
            path.display()
        )));
    }
    let options = resolve(args)?;
    Ok((path, options))
}

/// Seleciona o perfil nomeado, retornando a configuração base e o perfil.
fn select_profile<'a>(
    file: Option<&'a ConfigFile>,
    name: Option<&str>,
) -> Result<(Profile, Option<&'a Profile>), CliError> {
    let Some(file) = file else {
        if name.is_some() {
            return Err(CliError::usage(
                "profile requested but config file not found",
            ));
        }
        return Ok((Profile::default(), None));
    };
    let base = Profile {
        base_url: file.base_url.clone(),
        token: file.token.clone(),
        token_file: file.token_file.clone(),
        auth_method: file.auth_method.clone(),
        sudo: file.sudo.clone(),
        timeout_secs: file.timeout_secs,
        max_rps: file.max_rps,
    };
    let Some(name) = name else {
        return Ok((base, None));
    };
    let profile = file.profiles.get(name).ok_or_else(|| {
        CliError::usage(format!(
            "profile '{name}' not found in {}",
            file.profiles
                .keys()
                .map(String::as_str)
                .collect::<Vec<_>>()
                .join(", ")
        ))
    })?;
    Ok((base, Some(profile)))
}

fn read_token_file(path: &str) -> Result<String, CliError> {
    let token = if path == "@-" {
        input::read_line_from_stdin("token")?
    } else {
        crate::io::read_file(path)?.trim_end().to_owned()
    };
    if token.trim().is_empty() {
        return Err(CliError::usage(format!(
            "token vazio em '{path}'; escreva o token em uma linha ou use GLW_TOKEN/--token. Veja 'glw config check'."
        )));
    }
    Ok(token)
}

/// Retorna a primeira opção presente.
fn first_some<T>(values: impl IntoIterator<Item = Option<T>>) -> Option<T> {
    values.into_iter().flatten().next()
}

/// Lê uma variável de ambiente não vazia.
fn env_nonempty(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|value| !value.is_empty())
}

/// Converte uma string em `AuthMethod`.
fn parse_auth_method(value: Option<&str>) -> Result<AuthMethod, CliError> {
    match value.unwrap_or("bearer").to_ascii_lowercase().as_str() {
        "bearer" => Ok(AuthMethod::Bearer),
        "header" | "private-token" => Ok(AuthMethod::Header),
        other => Err(CliError::usage(format!(
            "invalid auth method '{other}' (expected 'bearer' or 'header')"
        ))),
    }
}

/// Normaliza a URL base, rejeitando valores vazios e URLs relativas.
fn normalize_url(url: &str) -> Result<String, CliError> {
    let url = url.trim().trim_end_matches('/');
    if url.is_empty() {
        return Err(CliError::usage(
            "base URL is empty (use --base-url, GITLAB_URL or config file)",
        ));
    }
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err(CliError::usage(format!(
            "base URL must start with http:// or https://: '{url}'"
        )));
    }
    Ok(url.to_owned())
}
