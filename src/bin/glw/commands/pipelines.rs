//! Handler do recurso `pipelines`.

use gitlab_wrapper::PipelineFilter;
use serde_json::Value;

use crate::args::{GetArgs, ListArgs, ResourceCommand};
use crate::commands::projects::{dry_run_value, parse_id};
use crate::commands::{Context, OperationParams, dry_run, emit};
use crate::error::CliError;
use crate::input;

/// Executa um subcomando de pipelines.
pub async fn run(context: &Context, command: ResourceCommand) -> Result<(), CliError> {
    match command {
        ResourceCommand::List(args) => list(context, args, context.global_query.clone()).await,
        ResourceCommand::Get(args) => get(context, &args).await,
        ResourceCommand::Latest(args) => latest(context, &args).await,
        ResourceCommand::GetByPath(_) | ResourceCommand::ForProject(_) => Err(CliError::usage(
            "pipelines supports only 'list', 'get' and 'latest'",
        )),
    }
}

/// Lista pipelines de um projeto.
pub async fn list(
    context: &Context,
    args: ListArgs,
    query: Vec<(String, String)>,
) -> Result<(), CliError> {
    let project_id = args
        .project_id
        .ok_or_else(|| CliError::usage("pipelines list requires a project_id argument"))?;
    let filter = filter_from_query(&query)?;
    let path = format!("projects/{project_id}/pipelines");
    if context.dry_run {
        return dry_run(
            "pipelines.list",
            "GET",
            &path,
            &context.options.base_url,
            query,
            None,
        );
    }
    let client = context.client()?;
    let pipelines = client.pipelines.list(project_id, filter.as_ref()).await?;
    emit(context, &pipelines)
}

/// Obtém uma pipeline pelo ID.
pub async fn get(context: &Context, args: &GetArgs) -> Result<(), CliError> {
    let pipeline_id = args
        .resource_id
        .ok_or_else(|| CliError::usage("pipelines get requires <project_id> <pipeline_id>"))?;
    let path = format!("projects/{}/pipelines/{pipeline_id}", args.project_id);
    if context.dry_run {
        return dry_run(
            "pipelines.get",
            "GET",
            &path,
            &context.options.base_url,
            Vec::new(),
            None,
        );
    }
    let client = context.client()?;
    let pipeline = client.pipelines.get(args.project_id, pipeline_id).await?;
    emit(context, &pipeline)
}

/// Obtém a pipeline mais recente de um projeto.
pub async fn latest(context: &Context, args: &GetArgs) -> Result<(), CliError> {
    let path = format!("projects/{}/pipelines/latest", args.project_id);
    if context.dry_run {
        return dry_run(
            "pipelines.latest",
            "GET",
            &path,
            &context.options.base_url,
            Vec::new(),
            None,
        );
    }
    let client = context.client()?;
    let pipeline = client.pipelines.get_latest(args.project_id).await?;
    emit(context, &pipeline)
}

/// Constrói um `PipelineFilter` a partir de pares chave=valor.
pub fn filter_from_query(query: &[(String, String)]) -> Result<Option<PipelineFilter>, CliError> {
    if query.is_empty() {
        return Ok(None);
    }
    let map = input::filter_object(query, &[], &["page", "per_page"]);
    let filter: PipelineFilter = serde_json::from_value(Value::Object(map))?;
    Ok(Some(filter))
}

/// Executa uma operação de batch.
pub async fn batch(context: &Context, params: &OperationParams) -> Result<Value, CliError> {
    match params.args.first().map(String::as_str) {
        Some("list") | None => {
            let project_id = parse_id(params.args.get(1), "project_id")?;
            let query = input::object_to_query(&params.query);
            let filter = filter_from_query(&query)?;
            let path = format!("projects/{project_id}/pipelines");
            if context.dry_run {
                return Ok(dry_run_value(
                    "pipelines.list",
                    "GET",
                    &path,
                    &context.options.base_url,
                    query,
                    None,
                ));
            }
            let client = context.client()?;
            serde_json::to_value(client.pipelines.list(project_id, filter.as_ref()).await?)
                .map_err(Into::into)
        }
        Some("get") => {
            let project_id = parse_id(params.args.get(1), "project_id")?;
            let pipeline_id = parse_id(params.args.get(2), "pipeline_id")?;
            if context.dry_run {
                return Ok(dry_run_value(
                    "pipelines.get",
                    "GET",
                    &format!("projects/{project_id}/pipelines/{pipeline_id}"),
                    &context.options.base_url,
                    Vec::new(),
                    None,
                ));
            }
            let client = context.client()?;
            serde_json::to_value(client.pipelines.get(project_id, pipeline_id).await?)
                .map_err(Into::into)
        }
        Some("latest") => {
            let project_id = parse_id(params.args.get(1), "project_id")?;
            if context.dry_run {
                return Ok(dry_run_value(
                    "pipelines.latest",
                    "GET",
                    &format!("projects/{project_id}/pipelines/latest"),
                    &context.options.base_url,
                    Vec::new(),
                    None,
                ));
            }
            let client = context.client()?;
            serde_json::to_value(client.pipelines.get_latest(project_id).await?).map_err(Into::into)
        }
        Some(other) => Err(CliError::usage(format!(
            "unknown pipelines operation '{other}'"
        ))),
    }
}
