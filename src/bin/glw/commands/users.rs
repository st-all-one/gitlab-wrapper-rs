//! Handler do recurso `users`.

use gitlab_wrapper::UserFilter;
use serde_json::Value;

use crate::args::{GetArgs, ListArgs, ResourceCommand};
use crate::commands::projects::{dry_run_value, parse_id};
use crate::commands::{Context, OperationParams, dry_run, emit};
use crate::error::CliError;
use crate::input;

/// Executa um subcomando de usuários.
pub async fn run(context: &Context, command: ResourceCommand) -> Result<(), CliError> {
    match command {
        ResourceCommand::List(args) => list(context, args, context.global_query.clone()).await,
        ResourceCommand::Get(args) => get(context, &args).await,
        ResourceCommand::GetByPath(_)
        | ResourceCommand::Latest(_)
        | ResourceCommand::ForProject(_) => {
            Err(CliError::usage("users supports only 'list' and 'get'"))
        }
    }
}

/// Lista usuários.
pub async fn list(
    context: &Context,
    _args: ListArgs,
    query: Vec<(String, String)>,
) -> Result<(), CliError> {
    let filter = filter_from_query(&query)?;
    if context.dry_run {
        return dry_run(
            "users.list",
            "GET",
            "users",
            &context.options.base_url,
            query,
            None,
        );
    }
    let client = context.client()?;
    let users = client.users.list(filter.as_ref()).await?;
    emit(context, &users)
}

/// Obtém um usuário pelo ID.
pub async fn get(context: &Context, args: &GetArgs) -> Result<(), CliError> {
    if context.dry_run {
        return dry_run(
            "users.get",
            "GET",
            &format!("users/{}", args.project_id),
            &context.options.base_url,
            Vec::new(),
            None,
        );
    }
    let client = context.client()?;
    let user = client.users.get(args.project_id).await?;
    emit(context, &user)
}

/// Constrói um `UserFilter` a partir de pares chave=valor.
pub fn filter_from_query(query: &[(String, String)]) -> Result<Option<UserFilter>, CliError> {
    if query.is_empty() {
        return Ok(None);
    }
    let map = input::filter_object(query, &["active", "blocked"], &["page", "per_page"]);
    let filter: UserFilter = serde_json::from_value(Value::Object(map))?;
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
                    "users.list",
                    "GET",
                    "users",
                    &context.options.base_url,
                    query,
                    None,
                ));
            }
            let client = context.client()?;
            serde_json::to_value(client.users.list(filter.as_ref()).await?).map_err(Into::into)
        }
        Some("get") => {
            let id = parse_id(params.args.get(1), "user_id")?;
            if context.dry_run {
                return Ok(dry_run_value(
                    "users.get",
                    "GET",
                    &format!("users/{id}"),
                    &context.options.base_url,
                    Vec::new(),
                    None,
                ));
            }
            let client = context.client()?;
            serde_json::to_value(client.users.get(id).await?).map_err(Into::into)
        }
        Some(other) => Err(CliError::usage(format!(
            "unknown users operation '{other}'"
        ))),
    }
}
