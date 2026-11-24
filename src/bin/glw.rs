//! `glw` — CLI de linha de comando para a API GitLab v4.
//!
//! O binário é um frontend de stdin/stdout sobre a biblioteca `gitlab_wrapper`:
//! dados em JSON/JSONL/tabela vão para stdout; diagnósticos e logs vão para stderr.

#[path = "glw/args.rs"]
mod args;
#[path = "glw/commands/mod.rs"]
mod commands;
#[path = "glw/config.rs"]
mod config;
#[path = "glw/error.rs"]
mod error;
#[path = "glw/input.rs"]
mod input;
#[path = "glw/io.rs"]
mod io;
#[path = "glw/logging.rs"]
mod logging;
#[path = "glw/output.rs"]
mod output;
#[path = "glw/prime.rs"]
mod prime;

use std::process::ExitCode;

use clap::Parser as _;

use crate::args::{Cli, Command};
use crate::error::CliError;

#[tokio::main(flavor = "current_thread")]
async fn main() -> ExitCode {
    let cli = Cli::parse();
    let quiet = cli.global.quiet;
    let verbose = cli.global.verbose;
    let level = logging::init(quiet, verbose);
    tracing::debug!(target: "glw", log_level = ?level, "logging initialized");

    match run(cli).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(cli_error) => {
            io::write_stderr(&cli_error.render());
            ExitCode::from(cli_error.exit_code())
        }
    }
}

/// Despacha o subcomando escolhido para o handler correspondente.
async fn run(cli: Cli) -> Result<(), CliError> {
    if let Command::Prime(arguments) = &cli.command {
        return io::write_stdout(&format!(
            "protocol={} long={}\n{}",
            prime::version(),
            arguments.long,
            prime::text(arguments.long).trim_end()
        ));
    }
    let context = commands::Context::new(&cli.global)?;
    match cli.command {
        Command::Projects { command } => commands::projects::run(&context, command).await,
        Command::Issues { command } => commands::issues::run(&context, command).await,
        Command::MergeRequests { command } => {
            commands::merge_requests::run(&context, command).await
        }
        Command::Users { command } => commands::users::run(&context, command).await,
        Command::Groups { command } => commands::groups::run(&context, command).await,
        Command::Pipelines { command } => commands::pipelines::run(&context, command).await,
        Command::WhoAmI => commands::whoami::run(&context).await,
        Command::Raw(arguments) => commands::raw::run(&context, arguments).await,
        Command::Batch => commands::batch::run(&context).await,
        Command::Config { command } => commands::config::run(&cli.global, &command),
        Command::Prime(_) => unreachable!("prime is handled before context creation"),
    }
}
