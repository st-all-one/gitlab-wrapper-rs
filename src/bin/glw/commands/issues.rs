//! Handler do recurso `issues`.

use gitlab_wrapper::IssueFilter;
use serde_json::Value;

use crate::args::{ForProjectArgs, GetArgs, ListArgs, ResourceCommand};
use crate::commands::projects::{dry_run_value, is_all, parse_id};
use crate::commands::{Context, OperationParams, dry_run, emit};
use crate::error::CliError;
use crate::input;

/// Executa um subcomando de issues.
pub async fn run(context: &Context, command: ResourceCommand) -> Result<(), CliError> {
    match command {
        ResourceCommand::List(args) => list(context, args, context.global_query.clone()).await,
        ResourceCommand::Get(args) => get(context, &args).await,
        ResourceCommand::ForProject(args) => for_project(context, &args, Vec::new()).await,
        ResourceCommand::GetByPath(_) | ResourceCommand::Latest(_) => Err(CliError::usage(
            "issues supports only 'list', 'get' and 'for-project'",
        )),
    }
}

/// Lista issues.
pub async fn list(
    context: &Context,
    _args: ListArgs,
    query: Vec<(String, String)>,
) -> Result<(), CliError> {
    let filter = filter_from_query(&query)?;
    if context.dry_run {
        return dry_run(
            "issues.list",
            "GET",
            "issues",
            &context.options.base_url,
            query,
            None,
        );
    }
    let client = context.client()?;
    let issues = client.issues.list(filter.as_ref()).await?;
    emit(context, &issues)
}

/// Obtém uma issue pelo projeto e IID.
pub async fn get(context: &Context, args: &GetArgs) -> Result<(), CliError> {
    let iid = parse_iid(args.resource_id)?;
    let path = format!("projects/{}/issues/{iid}", args.project_id);
    if context.dry_run {
        return dry_run(
            "issues.get",
            "GET",
            &path,
            &context.options.base_url,
            Vec::new(),
            None,
        );
    }
    let client = context.client()?;
    let issue = client.issues.get(args.project_id, iid).await?;
    emit(context, &issue)
}

/// Lista issues de um projeto.
pub async fn for_project(
    context: &Context,
    args: &ForProjectArgs,
    query: Vec<(String, String)>,
) -> Result<(), CliError> {
    let filter = filter_from_query(&query)?;
    let path = format!("projects/{}/issues", args.project_id);
    if context.dry_run {
        return dry_run(
            "issues.list_for_project",
            "GET",
            &path,
            &context.options.base_url,
            query,
            None,
        );
    }
    let client = context.client()?;
    let issues = client
        .issues
        .list_for_project(args.project_id, filter.as_ref())
        .await?;
    emit(context, &issues)
}

/// Constrói um `IssueFilter` a partir de pares chave=valor.
pub fn filter_from_query(query: &[(String, String)]) -> Result<Option<IssueFilter>, CliError> {
    if query.is_empty() {
        return Ok(None);
    }
    let map = input::filter_object(
        query,
        &["confidential"],
        &["page", "per_page", "assignee_id"],
    );
    let filter: IssueFilter = serde_json::from_value(Value::Object(map))?;
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
                    "issues.list",
                    "GET",
                    "issues",
                    &context.options.base_url,
                    query,
                    None,
                ));
            }
            let client = context.client()?;
            serde_json::to_value(client.issues.list(filter.as_ref()).await?).map_err(Into::into)
        }
        Some("get") => {
            let project_id = parse_id(params.args.get(1), "project_id")?;
            let iid = parse_iid_str(params.args.get(2))?;
            if context.dry_run {
                return Ok(dry_run_value(
                    "issues.get",
                    "GET",
                    &format!("projects/{project_id}/issues/{iid}"),
                    &context.options.base_url,
                    Vec::new(),
                    None,
                ));
            }
            let client = context.client()?;
            serde_json::to_value(client.issues.get(project_id, iid).await?).map_err(Into::into)
        }
        Some("list_for_project" | "list-for-project" | "for-project") => {
            let project_id = parse_id(params.args.get(1), "project_id")?;
            let query = input::object_to_query(&params.query);
            let filter = filter_from_query(&query)?;
            if context.dry_run {
                return Ok(dry_run_value(
                    "issues.list_for_project",
                    "GET",
                    &format!("projects/{project_id}/issues"),
                    &context.options.base_url,
                    query,
                    None,
                ));
            }
            let client = context.client()?;
            let issues = client
                .issues
                .list_for_project(project_id, filter.as_ref())
                .await?;
            let _ = is_all(params);
            serde_json::to_value(issues).map_err(Into::into)
        }
        Some(other) => Err(CliError::usage(format!(
            "unknown issues operation '{other}'"
        ))),
    }
}

/// Converte um IID opcional.
pub fn parse_iid(id: Option<u64>) -> Result<u32, CliError> {
    let id = id.ok_or_else(|| CliError::usage("missing issue IID argument"))?;
    u32::try_from(id).map_err(|_| CliError::usage("issue IID must fit u32"))
}

/// Converte um IID em string.
pub fn parse_iid_str(id: Option<&String>) -> Result<u32, CliError> {
    id.ok_or_else(|| CliError::usage("missing issue IID argument"))?
        .parse::<u32>()
        .map_err(|error| CliError::usage(format!("invalid issue IID: {error}")))
}
