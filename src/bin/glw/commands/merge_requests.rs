//! Handler do recurso `merge_requests`.

use gitlab_wrapper::MergeRequestFilter;
use serde_json::Value;

use crate::args::{ForProjectArgs, GetArgs, ListArgs, ResourceCommand};
use crate::commands::issues::{parse_iid, parse_iid_str};
use crate::commands::projects::{dry_run_value, parse_id};
use crate::commands::{Context, OperationParams, dry_run, emit};
use crate::error::CliError;
use crate::input;

/// Executa um subcomando de merge requests.
pub async fn run(context: &Context, command: ResourceCommand) -> Result<(), CliError> {
    match command {
        ResourceCommand::List(args) => list(context, args, context.global_query.clone()).await,
        ResourceCommand::Get(args) => get(context, &args).await,
        ResourceCommand::ForProject(args) => for_project(context, &args, Vec::new()).await,
        ResourceCommand::GetByPath(_) | ResourceCommand::Latest(_) => Err(CliError::usage(
            "merge-requests supports only 'list', 'get' and 'for-project'",
        )),
    }
}

/// Lista merge requests.
pub async fn list(
    context: &Context,
    _args: ListArgs,
    query: Vec<(String, String)>,
) -> Result<(), CliError> {
    let filter = filter_from_query(&query)?;
    if context.dry_run {
        return dry_run(
            "merge_requests.list",
            "GET",
            "merge_requests",
            &context.options.base_url,
            query,
            None,
        );
    }
    let client = context.client()?;
    let mrs = client.merge_requests.list(filter.as_ref()).await?;
    emit(context, &mrs)
}

/// Obtém um merge request pelo projeto e IID.
pub async fn get(context: &Context, args: &GetArgs) -> Result<(), CliError> {
    let iid = parse_iid(args.resource_id)?;
    let path = format!("projects/{}/merge_requests/{iid}", args.project_id);
    if context.dry_run {
        return dry_run(
            "merge_requests.get",
            "GET",
            &path,
            &context.options.base_url,
            Vec::new(),
            None,
        );
    }
    let client = context.client()?;
    let mr = client.merge_requests.get(args.project_id, iid).await?;
    emit(context, &mr)
}

/// Lista merge requests de um projeto.
pub async fn for_project(
    context: &Context,
    args: &ForProjectArgs,
    query: Vec<(String, String)>,
) -> Result<(), CliError> {
    let filter = filter_from_query(&query)?;
    let path = format!("projects/{}/merge_requests", args.project_id);
    if context.dry_run {
        return dry_run(
            "merge_requests.list_for_project",
            "GET",
            &path,
            &context.options.base_url,
            query,
            None,
        );
    }
    let client = context.client()?;
    let mrs = client
        .merge_requests
        .list_for_project(args.project_id, filter.as_ref())
        .await?;
    emit(context, &mrs)
}

/// Constrói um `MergeRequestFilter` a partir de pares chave=valor.
pub fn filter_from_query(
    query: &[(String, String)],
) -> Result<Option<MergeRequestFilter>, CliError> {
    if query.is_empty() {
        return Ok(None);
    }
    let map = input::filter_object(
        query,
        &["draft"],
        &[
            "page",
            "per_page",
            "assignee_id",
            "author_id",
            "reviewer_id",
        ],
    );
    let filter: MergeRequestFilter = serde_json::from_value(Value::Object(map))?;
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
                    "merge_requests.list",
                    "GET",
                    "merge_requests",
                    &context.options.base_url,
                    query,
                    None,
                ));
            }
            let client = context.client()?;
            serde_json::to_value(client.merge_requests.list(filter.as_ref()).await?)
                .map_err(Into::into)
        }
        Some("get") => {
            let project_id = parse_id(params.args.get(1), "project_id")?;
            let iid = parse_iid_str(params.args.get(2))?;
            if context.dry_run {
                return Ok(dry_run_value(
                    "merge_requests.get",
                    "GET",
                    &format!("projects/{project_id}/merge_requests/{iid}"),
                    &context.options.base_url,
                    Vec::new(),
                    None,
                ));
            }
            let client = context.client()?;
            serde_json::to_value(client.merge_requests.get(project_id, iid).await?)
                .map_err(Into::into)
        }
        Some("list_for_project" | "list-for-project" | "for-project") => {
            let project_id = parse_id(params.args.get(1), "project_id")?;
            let query = input::object_to_query(&params.query);
            let filter = filter_from_query(&query)?;
            if context.dry_run {
                return Ok(dry_run_value(
                    "merge_requests.list_for_project",
                    "GET",
                    &format!("projects/{project_id}/merge_requests"),
                    &context.options.base_url,
                    query,
                    None,
                ));
            }
            let client = context.client()?;
            let mrs = client
                .merge_requests
                .list_for_project(project_id, filter.as_ref())
                .await?;
            serde_json::to_value(mrs).map_err(Into::into)
        }
        Some(other) => Err(CliError::usage(format!(
            "unknown merge_requests operation '{other}'"
        ))),
    }
}
