use serde::{Deserialize, Serialize};

/// Payload genérico para configurar ou atualizar uma integração no GitLab.
///
/// Os campos variam conforme o tipo de integração. Utilize `serde_json::json!`
/// para construir o payload com os parâmetros esperados pela integração específica.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IntegrationPayload {
    /// Parâmetros dinâmicos da integração.
    #[serde(flatten)]
    pub inner: serde_json::Value,
}
