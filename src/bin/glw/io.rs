//! Abstração de IO: único ponto da CLI que escreve em stdout/stderr.
//!
//! `print_stdout`/`print_stderr` são `deny` no manifesto; aqui o allow é local e
//! justificado: o contrato do binário é justamente escrever dados em stdout e
//! diagnósticos em stderr. Todo o restante do código usa estas funções.

use crate::error::CliError;
use std::io::Read as _;
use std::io::{self, Write as _};

/// Escreve uma linha de dados em stdout.
#[allow(clippy::print_stdout, reason = "stdout é o canal de dados da CLI")]
pub fn write_stdout(line: &str) -> Result<(), CliError> {
    let mut stdout = io::stdout().lock();
    stdout
        .write_all(line.as_bytes())
        .and_then(|()| stdout.write_all(b"\n"))
        .and_then(|()| stdout.flush())
        .map_err(Into::into)
}

/// Escreve um diagnóstico em stderr.
#[allow(
    clippy::print_stderr,
    reason = "stderr é o canal de diagnóstico da CLI"
)]
pub fn write_stderr(line: &str) {
    let mut stderr = io::stderr().lock();
    // Não há como propagar falha de stderr no caminho de erro; registrar o
    // resultado é o melhor que se pode fazer sem introduzir pânico.
    let _ = stderr.write_all(line.as_bytes()).and_then(|()| {
        stderr.write_all(b"\n")?;
        stderr.flush()
    });
}

/// Lê todo o stdin como texto UTF-8.
pub fn read_stdin() -> Result<String, CliError> {
    let mut buffer = String::new();
    io::stdin()
        .lock()
        .read_to_string(&mut buffer)
        .map_err(|error| CliError::io(format!("failed to read stdin: {error}")))?;
    Ok(buffer)
}

/// Lê um arquivo de texto UTF-8.
pub fn read_file(path: &str) -> Result<String, CliError> {
    std::fs::read_to_string(path)
        .map_err(|error| CliError::io(format!("failed to read '{path}': {error}")))
}
