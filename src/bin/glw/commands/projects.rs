//! Handler do recurso `projects`.

use gitlab_wrapper::ProjectFilter;
use serde_json::Value;

use crate::args::{ForProjectArgs, GetArgs, ListArgs, PathArgs, ResourceCommand};
use crate::commands::{Context, OperationParams, dry_run, emit};
use crate::error::CliError;
use crate::input;

/// Executa um subcomando de projetos.
pub async fn run(context: &Context, command: ResourceCommand) -> Result<(), CliError> {
    match command {
        ResourceCommand::List(args) => list(context, args, context.global_query.clone()).await,
        ResourceCommand::Get(args) | ResourceCommand::Latest(args) => get(context, &args).await,
        ResourceCommand::GetByPath(args) => get_by_path(context, &args).await,
        ResourceCommand::ForProject(args) => forks(context, &args).await,
    }
}

/// Lista projetos (ou todas as páginas com `--all`).
pub async fn list(
    context: &Context,
    args: ListArgs,
    query: Vec<(String, String)>,
) -> Result<(), CliError> {
    let filter = filter_from_query(&query)?;
    if context.dry_run {
        return dry_run(
            "projects.list",
            "GET",
            "projects",
            &context.options.base_url,
            query,
            None,
        );
    }
    let client = context.client()?;
    let projects = if args.all {
        client.projects.list_all(filter.as_ref()).await?
    } else {
        client.projects.list(filter.as_ref()).await?
    };
    emit(context, &projects)
}

/// Obtém um projeto pelo ID numérico.
pub async fn get(context: &Context, args: &GetArgs) -> Result<(), CliError> {
    if context.dry_run {
        return dry_run(
            "projects.get",
            "GET",
            &format!("projects/{}", args.project_id),
            &context.options.base_url,
            Vec::new(),
            None,
        );
    }
    let client = context.client()?;
    let project = client.projects.get(args.project_id).await?;
    emit(context, &project)
}

/// Obtém um projeto pelo caminho URL-encoded.
pub async fn get_by_path(context: &Context, args: &PathArgs) -> Result<(), CliError> {
    if context.dry_run {
        return dry_run(
            "projects.get_by_path",
            "GET",
            &format!("projects/{}", urlencode(&args.path)),
            &context.options.base_url,
            Vec::new(),
            None,
        );
    }
    let client = context.client()?;
    let project = client.projects.get_by_path(&args.path).await?;
    emit(context, &project)
}

/// Lista os forks de um projeto (`for-project`).
pub async fn forks(context: &Context, args: &ForProjectArgs) -> Result<(), CliError> {
    if context.dry_run {
        return dry_run(
            "projects.list_forks",
            "GET",
            &format!("projects/{}/forks", args.project_id),
            &context.options.base_url,
            Vec::new(),
            None,
        );
    }
    let client = context.client()?;
    let projects = client.projects.list_forks(args.project_id, None).await?;
    emit(context, &projects)
}

/// Constrói um `ProjectFilter` a partir de pares chave=valor.
pub fn filter_from_query(query: &[(String, String)]) -> Result<Option<ProjectFilter>, CliError> {
    if query.is_empty() {
        return Ok(None);
    }
    let map = input::filter_object(
        query,
        &["membership", "owned", "starred", "archived", "statistics"],
        &["page", "per_page"],
    );
    let filter: ProjectFilter = serde_json::from_value(Value::Object(map))?;
    Ok(Some(filter))
}

/// Executa uma operação de batch.
pub async fn batch(context: &Context, params: &OperationParams) -> Result<Value, CliError> {
    let query = input::object_to_query(&params.query);
    match params.args.first().map(String::as_str) {
        Some("list") | None => emit_batch_list(context, params, query).await,
        Some("get") => {
            let id = parse_id(params.args.get(1), "project_id")?;
            if context.dry_run {
                return Ok(dry_run_value(
                    "projects.get",
                    "GET",
                    &format!("projects/{id}"),
                    &context.options.base_url,
                    query,
                    None,
                ));
            }
            let client = context.client()?;
            serde_json::to_value(client.projects.get(id).await?).map_err(Into::into)
        }
        Some("get_by_path" | "get-by-path") => {
            let path = params
                .args
                .get(1)
                .ok_or_else(|| CliError::usage("projects.get_by_path requires a path argument"))?;
            if context.dry_run {
                return Ok(dry_run_value(
                    "projects.get_by_path",
                    "GET",
                    &format!("projects/{}", urlencode(path)),
                    &context.options.base_url,
                    query,
                    None,
                ));
            }
            let client = context.client()?;
            serde_json::to_value(client.projects.get_by_path(path).await?).map_err(Into::into)
        }
        Some("list_forks" | "list-forks") => {
            let id = parse_id(params.args.get(1), "project_id")?;
            if context.dry_run {
                return Ok(dry_run_value(
                    "projects.list_forks",
                    "GET",
                    &format!("projects/{id}/forks"),
                    &context.options.base_url,
                    query,
                    None,
                ));
            }
            let client = context.client()?;
            serde_json::to_value(client.projects.list_forks(id, None).await?).map_err(Into::into)
        }
        Some(other) => Err(CliError::usage(format!(
            "unknown projects operation '{other}'"
        ))),
    }
}

/// Lista projetos em modo batch, retornando o valor em vez de escrevê-lo.
async fn emit_batch_list(
    context: &Context,
    params: &OperationParams,
    query: Vec<(String, String)>,
) -> Result<Value, CliError> {
    let filter = filter_from_query(&query)?;
    if context.dry_run {
        return Ok(dry_run_value(
            "projects.list",
            "GET",
            "projects",
            &context.options.base_url,
            query,
            None,
        ));
    }
    let client = context.client()?;
    let projects = if is_all(params) {
        client.projects.list_all(filter.as_ref()).await?
    } else {
        client.projects.list(filter.as_ref()).await?
    };
    serde_json::to_value(projects).map_err(Into::into)
}

/// Constrói o relatório de dry-run como valor JSON.
pub fn dry_run_value(
    op: &str,
    method: &str,
    path: &str,
    base_url: &str,
    query: Vec<(String, String)>,
    body: Option<Value>,
) -> Value {
    let query: serde_json::Map<String, Value> = query
        .into_iter()
        .map(|(key, value)| (key, Value::String(value)))
        .collect();
    let mut report = serde_json::json!({
        "dry_run": true,
        "op": op,
        "method": method,
        "path": path,
        "base_url": base_url,
        "query": query,
    });
    if let Some(body) = body {
        report["body"] = body;
    }
    report
}

/// Verifica se o batch pediu todas as páginas.
pub fn is_all(params: &OperationParams) -> bool {
    params
        .named
        .get("all")
        .and_then(Value::as_bool)
        .unwrap_or(false)
}

/// Faz parse de um ID posicional.
pub fn parse_id(value: Option<&String>, name: &str) -> Result<u64, CliError> {
    value
        .ok_or_else(|| CliError::usage(format!("missing {name} argument")))?
        .parse::<u64>()
        .map_err(|error| CliError::usage(format!("invalid {name}: {error}")))
}

/// Codifica um caminho para uso em URL.
pub fn urlencode(path: &str) -> String {
    url::form_urlencoded::byte_serialize(path.as_bytes()).collect()
}
