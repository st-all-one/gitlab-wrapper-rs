use crate::types::base::*;
use serde::{Deserialize, Serialize};

/// Resposta da API GitLab representando um agendamento de pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PipelineSchedule {
    /// ID do agendamento.
    pub id: GitLabId,
    /// Descrição do agendamento.
    pub description: Option<String>,
    /// Referência (branch/tag) — renomeado de `ref` para `ref_` devido a palavra reservada.
    #[serde(rename = "ref")]
    pub ref_: Option<String>,
    /// Expressão cron do agendamento.
    pub cron: Option<String>,
    /// Fuso horário da expressão cron.
    pub cron_timezone: Option<String>,
    /// Próxima execução agendada.
    pub next_run_at: Option<String>,
    /// Indica se o agendamento está ativo.
    pub active: Option<bool>,
    /// Data de criação.
    pub created_at: Option<String>,
    /// Data de atualização.
    pub updated_at: Option<String>,
    /// Proprietário do agendamento.
    pub owner: Option<AuthorInfo>,
    /// Último pipeline executado pelo agendamento.
    pub last_pipeline: Option<PipelineScheduleLastPipeline>,
    /// Variáveis do agendamento.
    pub variables: Option<Vec<PipelineScheduleVariable>>,
}

/// Variável de um agendamento de pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PipelineScheduleVariable {
    /// ID da variável.
    pub id: Option<GitLabId>,
    /// Chave da variável.
    pub key: Option<String>,
    /// Valor da variável.
    pub value: Option<String>,
    /// Tipo da variável (env_var, file).
    pub variable_type: Option<String>,
}

/// Último pipeline executado por um agendamento.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PipelineScheduleLastPipeline {
    /// ID do pipeline.
    pub id: Option<GitLabId>,
    /// Referência (branch/tag) — renomeado de `ref` para `ref_` devido a palavra reservada.
    #[serde(rename = "ref")]
    pub ref_: Option<String>,
    /// SHA do commit do pipeline.
    pub sha: Option<String>,
    /// Status do pipeline.
    pub status: Option<String>,
}

/// Payload para criar um agendamento de pipeline na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreatePipelineSchedulePayload {
    /// Descrição do agendamento (obrigatório).
    pub description: String,
    /// Referência (branch/tag) — renomeado de `ref` para `ref_` devido a palavra reservada.
    #[serde(rename = "ref")]
    pub ref_: String,
    /// Expressão cron (obrigatório).
    pub cron: String,
    /// Fuso horário da expressão cron.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cron_timezone: Option<String>,
    /// Indica se o agendamento está ativo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}

/// Payload para atualizar um agendamento de pipeline na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdatePipelineSchedulePayload {
    /// Descrição do agendamento.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Referência (branch/tag) — renomeado de `ref` para `ref_` devido a palavra reservada.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ref")]
    pub ref_: Option<String>,
    /// Expressão cron.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cron: Option<String>,
    /// Fuso horário da expressão cron.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cron_timezone: Option<String>,
    /// Indica se o agendamento está ativo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}
