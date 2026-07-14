use crate::types::base::GitLabId;
use serde::{Deserialize, Serialize};

/// Variável de CI/CD em nível de grupo no GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GroupVariable {
    /// ID da variável.
    pub id: GitLabId,
    /// Chave (nome) da variável.
    pub key: String,
    /// Valor da variável.
    pub value: String,
    /// Tipo da variável (`"env_var"` ou `"file"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable_type: Option<String>,
    /// Indica se a variável é protegida.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protected: Option<bool>,
    /// Indica se a variável é mascarada nos logs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub masked: Option<bool>,
    /// Escopo de ambiente (ex.: `"production"`, `"*"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_scope: Option<String>,
    /// Descrição da variável.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Payload para criar uma nova variável de grupo.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateGroupVariablePayload {
    /// Chave (nome) da variável.
    pub key: String,
    /// Valor da variável.
    pub value: String,
    /// Tipo da variável (`"env_var"` ou `"file"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable_type: Option<String>,
    /// Se a variável é protegida (disponível apenas em branches protegidos).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protected: Option<bool>,
    /// Se a variável é mascarada nos logs de job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub masked: Option<bool>,
    /// Escopo de ambiente da variável.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_scope: Option<String>,
    /// Descrição da variável.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Payload para atualizar uma variável de grupo existente.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateGroupVariablePayload {
    /// Novo valor da variável.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Novo tipo da variável.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable_type: Option<String>,
    /// Se a variável deve ser protegida.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protected: Option<bool>,
    /// Se a variável deve ser mascarada.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub masked: Option<bool>,
    /// Novo escopo de ambiente.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_scope: Option<String>,
    /// Nova descrição da variável.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Filtros para listar variáveis de grupo.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GroupVariableFilter {
    /// Número da página.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Quantidade de itens por página.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>,
}
