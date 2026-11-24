//! Handler do comando `whoami`.

use serde_json::Value;

use crate::commands::projects::dry_run_value;
use crate::commands::{Context, OperationParams, dry_run, emit};
use crate::error::CliError;

/// Retorna os dados do usuário autenticado.
pub async fn run(context: &Context) -> Result<(), CliError> {
    if context.dry_run {
        return dry_run(
            "whoami",
            "GET",
            "user",
            &context.options.base_url,
            Vec::new(),
            None,
        );
    }
    let client = context.client()?;
    let user = client.users.get_current().await?;
    emit(context, &user)
}

/// Executa `whoami` em batch.
pub async fn batch(context: &Context, _params: &OperationParams) -> Result<Value, CliError> {
    if context.dry_run {
        return Ok(dry_run_value(
            "whoami",
            "GET",
            "user",
            &context.options.base_url,
            Vec::new(),
            None,
        ));
    }
    let client = context.client()?;
    serde_json::to_value(client.users.get_current().await?).map_err(Into::into)
}
