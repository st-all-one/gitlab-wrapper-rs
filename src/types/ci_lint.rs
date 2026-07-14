use serde::{Deserialize, Serialize};

/// Resultado da validação de um arquivo `.gitlab-ci.yml`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CiLintResult {
    /// Status da validação ("valid", "invalid").
    /// Campo `pub status`.
    pub status: String,
    /// Lista de erros encontrados.
    /// Campo `pub errors`.
    pub errors: Vec<String>,
    /// Lista de warnings encontrados.
    /// Campo `pub warnings`.
    pub warnings: Vec<String>,
    /// YAML mesclado após inclusões (se solicitado).
    /// Campo `pub merged_yaml`.
    pub merged_yaml: Option<String>,
}

/// Payload para validar um arquivo `.gitlab-ci.yml`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CiLintPayload {
    /// Conteúdo do arquivo `.gitlab-ci.yml`.
    /// Campo `pub content`.
    pub content: String,
    /// Incluir o YAML mesclado na resposta.
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Campo `pub include_merged_yaml`.
    pub include_merged_yaml: Option<bool>,
}
