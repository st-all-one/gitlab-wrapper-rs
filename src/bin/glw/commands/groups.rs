//! Handler do recurso `groups`.

use gitlab_wrapper::GroupFilter;
use serde_json::Value;

use crate::args::{ForProjectArgs, GetArgs, ListArgs, PathArgs, ResourceCommand};
use crate::commands::projects::{dry_run_value, parse_id, urlencode};
use crate::commands::{Context, OperationParams, dry_run, emit};
use crate::error::CliError;
use crate::input;

/// Executa um subcomando de grupos.
pub async fn run(context: &Context, command: ResourceCommand) -> Result<(), CliError> {
    match command {
        ResourceCommand::List(args) => list(context, args, context.global_query.clone()).await,
        ResourceCommand::Get(args) => get(context, &args).await,
        ResourceCommand::GetByPath(args) => get_by_path(context, &args).await,
        ResourceCommand::ForProject(args) => projects(context, &args).await,
        ResourceCommand::Latest(_) => Err(CliError::usage(
            "groups supports 'list', 'get', 'get-by-path' and 'for-project'",
        )),
    }
}

/// Lista grupos.
pub async fn list(
    context: &Context,
    _args: ListArgs,
    query: Vec<(String, String)>,
) -> Result<(), CliError> {
    let filter = filter_from_query(&query)?;
    if context.dry_run {
        return dry_run(
            "groups.list",
            "GET",
            "groups",
            &context.options.base_url,
            query,
            None,
        );
    }
    let client = context.client()?;
    let groups = client.groups.list(filter.as_ref()).await?;
    emit(context, &groups)
}

/// Obtém um grupo pelo ID.
pub async fn get(context: &Context, args: &GetArgs) -> Result<(), CliError> {
    if context.dry_run {
        return dry_run(
            "groups.get",
            "GET",
            &format!("groups/{}", args.project_id),
            &context.options.base_url,
            Vec::new(),
            None,
        );
    }
    let client = context.client()?;
    let group = client.groups.get(args.project_id).await?;
    emit(context, &group)
}

/// Obtém um grupo pelo caminho.
pub async fn get_by_path(context: &Context, args: &PathArgs) -> Result<(), CliError> {
    if context.dry_run {
        return dry_run(
            "groups.get_by_path",
            "GET",
            &format!("groups/{}", urlencode(&args.path)),
            &context.options.base_url,
            Vec::new(),
            None,
        );
    }
    let client = context.client()?;
    let group = client.groups.get_by_path(&args.path).await?;
    emit(context, &group)
}

/// Lista os projetos de um grupo.
pub async fn projects(context: &Context, args: &ForProjectArgs) -> Result<(), CliError> {
    let path = format!("groups/{}/projects", args.project_id);
    if context.dry_run {
        return dry_run(
            "groups.projects",
            "GET",
            &path,
            &context.options.base_url,
            Vec::new(),
            None,
        );
    }
    let client = context.client()?;
    let projects = client.groups.projects(args.project_id).await?;
    emit(context, &projects)
}

/// Constrói um `GroupFilter` a partir de pares chave=valor.
pub fn filter_from_query(query: &[(String, String)]) -> Result<Option<GroupFilter>, CliError> {
    if query.is_empty() {
        return Ok(None);
    }
    let map = input::filter_object(query, &["top_level_only"], &["page", "per_page"]);
    let filter: GroupFilter = serde_json::from_value(Value::Object(map))?;
    Ok(Some(filter))
}

/// Executa uma operação de batch.
pub async fn batch(context: &Context, params: &OperationParams) -> Result<Value, CliError> {
    match params.args.first().map(String::as_str) {
        Some("list") | None => {
            let query = input::object_to_query(&params.query);
            let filter = filter_from_query(&query)?;
            if context.dry_run {
                return Ok(dry_run_value(
                    "groups.list",
                    "GET",
                    "groups",
                    &context.options.base_url,
                    query,
                    None,
                ));
            }
            let client = context.client()?;
            serde_json::to_value(client.groups.list(filter.as_ref()).await?).map_err(Into::into)
        }
        Some("get") => {
            let id = parse_id(params.args.get(1), "group_id")?;
            if context.dry_run {
                return Ok(dry_run_value(
                    "groups.get",
                    "GET",
                    &format!("groups/{id}"),
                    &context.options.base_url,
                    Vec::new(),
                    None,
                ));
            }
            let client = context.client()?;
            serde_json::to_value(client.groups.get(id).await?).map_err(Into::into)
        }
        Some("get_by_path" | "get-by-path") => {
            let path = params
                .args
                .get(1)
                .ok_or_else(|| CliError::usage("groups.get_by_path requires a path argument"))?;
            if context.dry_run {
                return Ok(dry_run_value(
                    "groups.get_by_path",
                    "GET",
                    &format!("groups/{}", urlencode(path)),
                    &context.options.base_url,
                    Vec::new(),
                    None,
                ));
            }
            let client = context.client()?;
            serde_json::to_value(client.groups.get_by_path(path).await?).map_err(Into::into)
        }
        Some(other) => Err(CliError::usage(format!(
            "unknown groups operation '{other}'"
        ))),
    }
}
