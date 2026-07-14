use crate::types::base::GitLabId;
use serde::{Deserialize, Serialize};

/// Resposta da API GitLab representando um tópico.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Topic {
    /// ID do tópico.
    pub id: GitLabId,
    /// Nome do tópico.
    pub name: String,
    /// Título do tópico.
    pub title: Option<String>,
    /// Descrição do tópico.
    pub description: Option<String>,
    /// URL do avatar do tópico.
    pub avatar_url: Option<String>,
    /// Número total de projetos associados ao tópico.
    pub total_projects_count: Option<u32>,
}

/// Payload para criar um tópico na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateTopicPayload {
    /// Nome do tópico (obrigatório).
    pub name: String,
    /// Título do tópico.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Descrição do tópico.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Avatar do tópico (arquivo ou URL).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
}

/// Payload para atualizar um tópico na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateTopicPayload {
    /// Nome do tópico.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Título do tópico.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Descrição do tópico.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Avatar do tópico (arquivo ou URL).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
}

/// Filtros disponíveis para listar tópicos.
/// Use `..Default::default()` para valores padrão.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TopicFilter {
    /// Número da página para recuperação.
    pub page: Option<u32>,
    /// Número de itens por página.
    pub per_page: Option<u32>,
    /// Termo de busca para filtrar tópicos.
    pub search: Option<String>,
}
