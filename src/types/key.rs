use serde::{Deserialize, Serialize};

/// Filtros disponíveis para buscar chaves SSH.
/// Use `..Default::default()` para valores padrão.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct KeyFilter {
    /// Impressão digital da chave SSH para busca.
    pub fingerprint: Option<String>,
}
