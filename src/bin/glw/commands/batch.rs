//! Handler do comando `batch`: executa operações NDJSON do stdin.

use serde_json::{Map, Value};

use crate::commands::{
    Context, OperationParams, groups, issues, merge_requests, pipelines, projects, raw, users,
    whoami,
};
use crate::error::CliError;
use crate::input;
use crate::io;

/// Executa o batch lido do stdin, uma operação por linha, em ordem determinística.
pub async fn run(context: &Context) -> Result<(), CliError> {
    let stdin = io::read_stdin()?;
    let mut failed = false;
    for (index, line) in stdin.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let (operation, params) = parse_operation(line, index)?;
        let op_name = operation.clone();
        let result = dispatch(context, &operation, params).await;
        match result {
            Ok(value) => {
                let record = serde_json::json!({
                    "ok": true,
                    "op": op_name,
                    "result": value,
                });
                crate::output::write_compact(&record)?;
            }
            Err(error) => {
                failed = true;
                tracing::warn!(target: "glw::batch", line = index + 1, op = %op_name, error = %error, "batch operation failed");
                let record = serde_json::json!({
                    "ok": false,
                    "op": op_name,
                    "error": error.message,
                });
                crate::output::write_compact(&record)?;
            }
        }
    }
    if failed {
        return Err(CliError::api(&gitlab_wrapper::GitLabError::api(
            gitlab_wrapper::ErrorCategory::InternalError,
            500,
            "one or more batch operations failed",
            gitlab_wrapper::ErrorContext::default(),
        )));
    }
    Ok(())
}

/// Faz o parse de uma linha NDJSON em `(op, params)`.
fn parse_operation(line: &str, index: usize) -> Result<(String, OperationParams), CliError> {
    let value: Value = serde_json::from_str(line).map_err(|error| {
        CliError::usage(format!(
            "invalid batch operation on line {}: {error}",
            index + 1
        ))
    })?;
    let object = value.as_object().ok_or_else(|| {
        CliError::usage(format!(
            "batch operation on line {} must be a JSON object",
            index + 1
        ))
    })?;
    let op = object
        .get("op")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            CliError::usage(format!(
                "batch operation on line {} missing 'op'",
                index + 1
            ))
        })?
        .to_owned();
    let canonical = canonical_op(&op);
    let args_value = object.get("args");
    let mut args = Vec::new();
    let mut named = std::collections::BTreeMap::new();
    match args_value {
        Some(Value::Array(values)) => {
            for item in values {
                match item {
                    Value::String(text) => args.push(text.clone()),
                    other => args.push(scalar(other)),
                }
            }
        }
        Some(Value::Object(map)) => {
            for (key, value) in map {
                named.insert(key.clone(), value.clone());
            }
        }
        Some(Value::String(text)) => args.push(text.clone()),
        Some(Value::Null) | None => {}
        Some(_other) => {
            return Err(CliError::usage(format!(
                "batch 'args' on line {} must be an array or object",
                index + 1
            )));
        }
    }
    // Argumentos nomeados também podem aparecer no topo da operação.
    for (key, value) in object {
        if key != "op" && key != "args" && key != "params" {
            named.insert(key.clone(), value.clone());
        }
    }
    let mut query = Map::new();
    if let Some(params) = object.get("params") {
        match params {
            Value::Object(map) => query.clone_from(map),
            Value::String(raw) => query = input::parse_object(raw)?,
            Value::Null => {}
            _ => {
                return Err(CliError::usage(format!(
                    "batch 'params' on line {} must be an object or string",
                    index + 1
                )));
            }
        }
    }
    // 'query' do raw também é aceito como alias de params.
    if let Some(Value::Object(map)) = object.get("query") {
        for (key, value) in map {
            query.insert(key.clone(), value.clone());
        }
    }
    Ok((canonical, OperationParams { args, query, named }))
}

/// Normaliza o nome da operação para a forma canônica com ponto.
fn canonical_op(op: &str) -> String {
    let op = op.trim();
    let op = op
        .replace("merge-requests.", "merge_requests.")
        .replace("merge-requests ", "merge_requests ")
        .replace(' ', ".");
    let op = op.strip_prefix("glw.").unwrap_or(&op);
    op.to_owned()
}

/// Converte um valor JSON escalar em string.
fn scalar(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

/// Despacha uma operação para o handler correspondente.
async fn dispatch(context: &Context, op: &str, params: OperationParams) -> Result<Value, CliError> {
    let (resource, _) = op.split_once('.').unwrap_or((op, ""));
    match resource {
        "whoami" => whoami::batch(context, &params).await,
        "projects" => projects::batch(context, &params).await,
        "issues" => issues::batch(context, &params).await,
        "merge_requests" => merge_requests::batch(context, &params).await,
        "users" => users::batch(context, &params).await,
        "groups" => groups::batch(context, &params).await,
        "pipelines" => pipelines::batch(context, &params).await,
        "raw" => raw::batch(context, &params).await,
        _ => Err(CliError::usage(format!("unknown batch operation '{op}'"))),
    }
}
