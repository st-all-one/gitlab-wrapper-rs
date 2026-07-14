use serde::{Deserialize, Serialize};

/// Atributo customizado no GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CustomAttribute {
    /// Campo `pub key`.
    pub key: String,
    /// Campo `pub value`.
    pub value: String,
}

/// Payload para criar/atualizar atributo customizado.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SetCustomAttributePayload {
    /// Campo `pub value`.
    pub value: String,
}
