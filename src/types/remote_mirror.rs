use serde::{Deserialize, Serialize};

/// Identificador numérico de recurso no GitLab.
use crate::types::base::GitLabId;

/// Espelho remoto (remote mirror) de um projeto no GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RemoteMirror {
    /// ID único do espelho remoto.
    pub id: GitLabId,
    /// URL de destino do espelho.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Indica se o espelho está habilitado.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Apenas branches protegidos são espelhados.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_protected_branches: Option<bool>,
    /// Mantém refs divergentes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_divergent_refs: Option<bool>,
    /// Status da última atualização do espelho.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_status: Option<String>,
    /// Data da última atualização.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_at: Option<String>,
    /// Data da última atualização bem-sucedida.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successful_update_at: Option<String>,
    /// Data de início da última atualização.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_started_at: Option<String>,
    /// Último erro ocorrido.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error: Option<String>,
}

/// Payload para criar um novo espelho remoto.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateRemoteMirrorPayload {
    /// URL de destino do espelho.
    pub url: String,
    /// Indica se o espelho deve ser habilitado.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Apenas branches protegidos devem ser espelhados.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_protected_branches: Option<bool>,
    /// Mantém refs divergentes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_divergent_refs: Option<bool>,
}

/// Payload para atualizar um espelho remoto existente.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateRemoteMirrorPayload {
    /// Indica se o espelho está habilitado.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Apenas branches protegidos são espelhados.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_protected_branches: Option<bool>,
    /// Mantém refs divergentes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_divergent_refs: Option<bool>,
}
