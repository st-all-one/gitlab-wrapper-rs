use serde::{Deserialize, Serialize};

/// Identificador numérico de recurso no GitLab.
use crate::types::base::GitLabId;

/// Período de congelamento (freeze period) no GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FreezePeriod {
    /// ID único do período de congelamento.
    pub id: GitLabId,
    /// Início do período de congelamento (formato cron).
    pub freeze_start: String,
    /// Fim do período de congelamento (formato cron).
    pub freeze_end: String,
    /// Fuso horário do cron.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cron_timezone: Option<String>,
    /// Data de criação.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Data da última atualização.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Payload para criar um novo período de congelamento.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateFreezePeriodPayload {
    /// Início do período de congelamento (formato cron).
    pub freeze_start: String,
    /// Fim do período de congelamento (formato cron).
    pub freeze_end: String,
    /// Fuso horário do cron.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cron_timezone: Option<String>,
}

/// Payload para atualizar um período de congelamento existente.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateFreezePeriodPayload {
    /// Início do período de congelamento (formato cron).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freeze_start: Option<String>,
    /// Fim do período de congelamento (formato cron).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freeze_end: Option<String>,
    /// Fuso horário do cron.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cron_timezone: Option<String>,
}
