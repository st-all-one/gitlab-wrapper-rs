use crate::types::base::GitLabId;
use serde::{Deserialize, Serialize};

/// Representação de um link entre issues no GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IssueLink {
    /// ID único do link entre issues.
    pub id: GitLabId,
    /// Issue de origem do link.
    pub source_issue: Option<IssueLinkIssue>,
    /// Issue de destino do link.
    pub target_issue: Option<IssueLinkIssue>,
    /// Data de criação do link no formato ISO 8601.
    pub created_at: Option<String>,
    /// Data da última atualização do link no formato ISO 8601.
    pub updated_at: Option<String>,
    /// Tipo do link ("relates_to", "blocks", "is_blocked_by").
    pub link_type: Option<String>,
}

/// Informações resumidas de uma issue participante de um link.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IssueLinkIssue {
    /// ID único da issue (global).
    pub id: Option<GitLabId>,
    /// ID interno da issue (por projeto).
    pub iid: Option<u32>,
    /// ID do projeto ao qual a issue pertence.
    pub project_id: Option<GitLabId>,
    /// Título da issue.
    pub title: Option<String>,
    /// Estado da issue ("opened", "closed").
    pub state: Option<String>,
    /// Data de criação no formato ISO 8601.
    pub created_at: Option<String>,
    /// Data da última atualização no formato ISO 8601.
    pub updated_at: Option<String>,
}

/// Payload para criar um link entre issues no GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateIssueLinkPayload {
    /// ID do projeto de destino (opcional, padrão é o mesmo projeto).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_project_id: Option<GitLabId>,
    /// IID da issue de destino no projeto.
    pub target_issue_iid: u32,
    /// Tipo do link ("relates_to", "blocks", "is_blocked_by").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_type: Option<String>,
}
