//! Configuração do subscriber de tracing (sempre em stderr).

use tracing::Level;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::writer::MakeWriterExt as _;

/// Inicializa o subscriber e retorna o nível efetivo.
///
/// - `--quiet`: nenhum subscriber é instalado (apenas erros impressos pela CLI).
/// - padrão: `WARN` (com filtro `GLW_LOG`/`RUST_LOG`, se definidos).
/// - `--verbose`: `INFO`; repetir a flag: `DEBUG`/`TRACE`.
pub fn init(quiet: bool, verbose: u8) -> Option<Level> {
    if quiet {
        return None;
    }
    let default_level = match verbose {
        0 => Level::WARN,
        1 => Level::INFO,
        2 => Level::DEBUG,
        _ => Level::TRACE,
    };
    let directive = default_level.as_str().to_owned();
    let filter = EnvFilter::try_from_env("GLW_LOG")
        .or_else(|_| EnvFilter::try_from_env("RUST_LOG"))
        .unwrap_or_else(|_| EnvFilter::new(directive));

    let stderr = std::io::stderr.with_max_level(default_level);
    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(stderr)
        .with_target(false)
        .with_ansi(false)
        .try_init();
    Some(default_level)
}
