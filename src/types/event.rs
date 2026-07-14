use crate::types::base::*;
use crate::types::note::Note;
use serde::{Deserialize, Serialize};

/// Resposta da API GitLab representando um evento de atividade.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Event {
    /// ID do evento.
    pub id: GitLabId,
    /// Título do evento.
    pub title: Option<String>,
    /// ID do projeto associado ao evento.
    pub project_id: Option<GitLabId>,
    /// ID do grupo associado ao evento.
    pub group_id: Option<GitLabId>,
    /// Nome da ação que gerou o evento.
    pub action_name: Option<String>,
    /// ID do alvo do evento.
    pub target_id: Option<GitLabId>,
    /// IID do alvo do evento.
    pub target_iid: Option<u32>,
    /// Tipo do alvo do evento.
    pub target_type: Option<String>,
    /// ID do autor do evento.
    pub author_id: Option<GitLabId>,
    /// Autor do evento.
    pub author: Option<AuthorInfo>,
    /// Título do alvo do evento.
    pub target_title: Option<String>,
    /// Data de criação.
    pub created_at: Option<String>,
    /// Nota associada ao evento (se houver).
    pub note: Option<Note>,
    /// Dados de push associados ao evento.
    pub push_data: Option<EventPushData>,
    /// Nome de usuário do autor.
    pub author_username: Option<String>,
}

/// Dados de push associados a um evento.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventPushData {
    /// Número de commits no push.
    pub commit_count: Option<u32>,
    /// Ação do push (pushed, created, deleted).
    pub action: Option<String>,
    /// Referência (branch/tag) — renomeado de `ref` para `ref_` devido a palavra reservada.
    #[serde(rename = "ref")]
    pub ref_: Option<String>,
    /// Tipo da referência (branch, tag).
    pub ref_type: Option<String>,
    /// Título do commit.
    pub commit_title: Option<String>,
    /// Número de referências.
    pub ref_count: Option<u32>,
}

/// Filtros para listar eventos. Use `..Default::default()`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EventFilter {
    /// Filtro por ação.
    pub action: Option<String>,
    /// Filtro por tipo de alvo.
    pub target_type: Option<String>,
    /// Filtro por data inicial (ISO 8601).
    pub after: Option<String>,
    /// Filtro por data final (ISO 8601).
    pub before: Option<String>,
    /// Escopo dos eventos (all, owned, etc.).
    pub scope: Option<String>,
    /// Número da página para paginação.
    pub page: Option<u32>,
    /// Número de itens por página.
    pub per_page: Option<u32>,
}
