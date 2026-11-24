//! Handlers dos subcomandos da CLI.

pub mod batch;
pub mod config;
pub mod groups;
pub mod issues;
pub mod merge_requests;
pub mod pipelines;
pub mod projects;
pub mod raw;
pub mod users;
pub mod whoami;

use std::collections::BTreeMap;

use gitlab_wrapper::{GitLabClient, GitLabConfig};
use serde_json::{Map, Value};

use crate::args::{GlobalArgs, OutputFormat};
use crate::config::ResolvedOptions;
use crate::error::CliError;
use crate::input;
use crate::output;

/// Estado compartilhado por todos os handlers.
pub struct Context {
    /// Configuração resolvida.
    pub options: ResolvedOptions,
    /// Formato de saída escolhido.
    pub format: OutputFormat,
    /// Modo dry-run.
    pub dry_run: bool,
    /// Parâmetros globais `--params` já resolvidos (chave=valor).
    pub global_query: Vec<(String, String)>,
}
impl Context {
    /// Constrói o contexto a partir dos argumentos globais.
    pub fn new(args: &GlobalArgs) -> Result<Self, CliError> {
        let options = crate::config::resolve(args)?;
        let global_query = match args.params.as_deref() {
            Some(raw) => input::object_to_query(&input::parse_object(&input::resolve_value(raw)?)?),
            None => Vec::new(),
        };

        Ok(Self {
            options,
            format: args.format,
            dry_run: args.dry_run,
            global_query,
        })
    }

    /// Cria o cliente da biblioteca com a configuração resolvida.
    pub fn client(&self) -> Result<GitLabClient, CliError> {
        let config: GitLabConfig = self.options.to_gitlab_config();
        GitLabClient::new(config).map_err(Into::into)
    }
}

/// Parâmetros de uma operação de batch.
#[derive(Debug, Default)]
pub struct OperationParams {
    /// Argumentos posicionais (IDs, caminhos etc.).
    pub args: Vec<String>,
    /// Parâmetros de consulta.
    pub query: Map<String, Value>,
    /// Argumentos nomeados extras (usado por `raw`).
    pub named: BTreeMap<String, Value>,
}

/// Escreve o resultado de uma operação no formato escolhido.
pub fn emit<T: serde::Serialize>(context: &Context, value: &T) -> Result<(), CliError> {
    output::write_value(context.format, value)
}

/// Imprime a requisição planejada em dry-run.
pub fn dry_run(
    operation: &str,
    method: &str,
    path: &str,
    base_url: &str,
    query: Vec<(String, String)>,
    body: Option<Value>,
) -> Result<(), CliError> {
    let query: BTreeMap<String, Value> = query
        .into_iter()
        .map(|(key, value)| (key, Value::String(value)))
        .collect();
    let mut report = serde_json::json!({
        "dry_run": true,
        "op": operation,
        "method": method,
        "path": path,
        "base_url": base_url,
        "query": query,
    });
    if let Some(body) = body {
        report["body"] = body;
    }
    output::write_compact(&report)
}
