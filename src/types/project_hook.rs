use crate::types::base::GitLabId;
use serde::{Deserialize, Serialize};

/// Webhook de projeto no GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Hook {
    /// ID do webhook.
    pub id: GitLabId,
    /// URL de destino do webhook.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// ID do projeto associado.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<GitLabId>,
    /// Dispara em pushes de código.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_events: Option<bool>,
    /// Filtro de branch para push events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_events_branch_filter: Option<String>,
    /// Dispara em eventos de issue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issues_events: Option<bool>,
    /// Dispara em eventos de issue confidencial.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidential_issues_events: Option<bool>,
    /// Dispara em eventos de merge request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_requests_events: Option<bool>,
    /// Dispara em eventos de nota/comentário.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_events: Option<bool>,
    /// Dispara em eventos de nota confidencial.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidential_note_events: Option<bool>,
    /// Dispara em eventos de pipeline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_events: Option<bool>,
    /// Dispara em eventos de página wiki.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wiki_page_events: Option<bool>,
    /// Dispara em eventos de job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_events: Option<bool>,
    /// Dispara em pushes de tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_push_events: Option<bool>,
    /// Dispara em eventos de feature flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_flag_events: Option<bool>,
    /// Dispara em eventos de release.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub releases_events: Option<bool>,
    /// Se a verificação SSL está habilitada.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_ssl_verification: Option<bool>,
    /// Data de criação.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Data da última atualização.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Token de autenticação do webhook (não enviado em listagens).
    #[serde(skip_serializing)]
    pub token: Option<String>,
    /// Template customizado do corpo do webhook.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_webhook_template: Option<String>,
    /// Headers customizados do webhook.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_headers: Option<Vec<HookCustomHeader>>,
}

/// Header customizado de um webhook.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HookCustomHeader {
    /// Chave do header.
    pub key: String,
    /// Valor do header.
    pub value: String,
}

/// Payload para criar um novo webhook de projeto.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateHookPayload {
    /// URL de destino do webhook.
    pub url: String,
    /// Dispara em pushes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_events: Option<bool>,
    /// Dispara em eventos de issue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issues_events: Option<bool>,
    /// Dispara em eventos de issue confidencial.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidential_issues_events: Option<bool>,
    /// Dispara em eventos de merge request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_requests_events: Option<bool>,
    /// Dispara em eventos de nota.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_events: Option<bool>,
    /// Dispara em eventos de nota confidencial.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidential_note_events: Option<bool>,
    /// Dispara em eventos de pipeline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_events: Option<bool>,
    /// Dispara em eventos de página wiki.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wiki_page_events: Option<bool>,
    /// Dispara em eventos de job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_events: Option<bool>,
    /// Dispara em pushes de tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_push_events: Option<bool>,
    /// Dispara em eventos de feature flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_flag_events: Option<bool>,
    /// Dispara em eventos de release.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub releases_events: Option<bool>,
    /// Se a verificação SSL está habilitada.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_ssl_verification: Option<bool>,
    /// Token de autenticação.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// Filtro de branch para push events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_events_branch_filter: Option<String>,
    /// Template customizado do corpo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_webhook_template: Option<String>,
}

/// Payload para atualizar um webhook de projeto.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateHookPayload {
    /// Nova URL de destino.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Dispara em pushes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_events: Option<bool>,
    /// Dispara em eventos de issue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issues_events: Option<bool>,
    /// Dispara em eventos de issue confidencial.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidential_issues_events: Option<bool>,
    /// Dispara em eventos de merge request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_requests_events: Option<bool>,
    /// Dispara em eventos de nota.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_events: Option<bool>,
    /// Dispara em eventos de nota confidencial.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidential_note_events: Option<bool>,
    /// Dispara em eventos de pipeline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_events: Option<bool>,
    /// Dispara em eventos de página wiki.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wiki_page_events: Option<bool>,
    /// Dispara em eventos de job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_events: Option<bool>,
    /// Dispara em pushes de tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_push_events: Option<bool>,
    /// Dispara em eventos de feature flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_flag_events: Option<bool>,
    /// Dispara em eventos de release.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub releases_events: Option<bool>,
    /// Se a verificação SSL está habilitada.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_ssl_verification: Option<bool>,
    /// Novo token de autenticação.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// Novo filtro de branch para push events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_events_branch_filter: Option<String>,
    /// Novo template customizado.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_webhook_template: Option<String>,
}

/// Filtros para listar webhooks de projeto.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HookFilter {
    /// Número da página.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Quantidade de itens por página.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>,
}
