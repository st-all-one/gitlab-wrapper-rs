//! Handlers utilitários `config path` e `config check`.

use crate::args::{ConfigCommand, GlobalArgs};
use crate::config;
use crate::error::CliError;
use crate::io;

/// Executa um subcomando de configuração (não exige token).
pub fn run(args: &GlobalArgs, command: &ConfigCommand) -> Result<(), CliError> {
    match command {
        ConfigCommand::Path => {
            io::write_stdout(&config::default_config_path().display().to_string())
        }
        ConfigCommand::Check => {
            let (path, options) = config::check(args)?;
            let lines = [
                format!("config: {}", path.display()),
                format!("base_url: {}", options.base_url),
                format!(
                    "token: {}",
                    if options.token.is_some() {
                        "set"
                    } else {
                        "unset"
                    }
                ),
                format!("auth_method: {}", auth_method_name(options.auth_method)),
                format!("sudo: {}", options.sudo.as_deref().unwrap_or("unset")),
                format!("timeout_secs: {}", options.timeout_secs),
                format!("max_rps: {}", options.max_rps),
                format!(
                    "profile: {}",
                    options.profile.as_deref().unwrap_or("default")
                ),
                "status: ok".to_owned(),
            ];
            for line in lines {
                io::write_stdout(&line)?;
            }
            Ok(())
        }
    }
}

/// Nome legível do método de autenticação.
const fn auth_method_name(method: gitlab_wrapper::AuthMethod) -> &'static str {
    match method {
        gitlab_wrapper::AuthMethod::Bearer => "bearer",
        gitlab_wrapper::AuthMethod::Header => "header",
    }
}
