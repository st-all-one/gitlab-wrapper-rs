use crate::types::base::*;
use serde::{Deserialize, Serialize};

/// Resposta da API GitLab representando um evento de alteração de estado
/// em uma issue ou merge request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ResourceStateEvent {
    /// ID do evento.
    pub id: GitLabId,
    /// Usuário que realizou a alteração de estado.
    pub user: Option<AuthorInfo>,
    /// Novo estado do recurso (ex.: "opened", "closed", "reopened").
    pub state: Option<String>,
    /// Data de criação do evento.
    pub created_at: Option<String>,
}

/// Resposta da API GitLab representando um evento de alteração de label
/// em uma issue ou merge request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ResourceLabelEvent {
    /// ID do evento.
    pub id: GitLabId,
    /// Usuário que realizou a alteração de label.
    pub user: Option<AuthorInfo>,
    /// Dados da label adicionada ou removida.
    pub label: Option<serde_json::Value>,
    /// Ação realizada ("add" ou "remove").
    pub action: Option<String>,
    /// Data de criação do evento.
    pub created_at: Option<String>,
}

/// Resposta da API GitLab representando um evento de alteração de milestone
/// em uma issue ou merge request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ResourceMilestoneEvent {
    /// ID do evento.
    pub id: GitLabId,
    /// Usuário que realizou a alteração de milestone.
    pub user: Option<AuthorInfo>,
    /// Dados do milestone adicionado ou removido.
    pub milestone: Option<serde_json::Value>,
    /// Ação realizada ("add" ou "remove").
    pub action: Option<String>,
    /// Data de criação do evento.
    pub created_at: Option<String>,
}

/// Resposta da API GitLab representando um evento de alteração de peso
/// em uma issue.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ResourceWeightEvent {
    /// ID do evento.
    pub id: GitLabId,
    /// Usuário que realizou a alteração de peso.
    pub user: Option<AuthorInfo>,
    /// Novo peso atribuído à issue.
    pub weight: Option<i32>,
    /// Data de criação do evento.
    pub created_at: Option<String>,
}

/// Resposta da API GitLab representando um evento de alteração de iteração
/// em uma issue.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ResourceIterationEvent {
    /// ID do evento.
    pub id: GitLabId,
    /// Usuário que realizou a alteração de iteração.
    pub user: Option<AuthorInfo>,
    /// Dados da iteração adicionada ou removida.
    pub iteration: Option<serde_json::Value>,
    /// Ação realizada ("add" ou "remove").
    pub action: Option<String>,
    /// Data de criação do evento.
    pub created_at: Option<String>,
}
