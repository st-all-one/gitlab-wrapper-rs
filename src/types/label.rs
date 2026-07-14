use crate::types::base::GitLabId;
use serde::{Deserialize, Serialize};

/// Resposta da API GitLab representando uma label de projeto.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Label {
    /// ID único da label.
    pub id: GitLabId,
    /// Nome da label.
    pub name: String,
    /// Cor da label em formato hexadecimal (ex.: "#FF0000").
    pub color: Option<String>,
    /// Cor do texto da label em formato hexadecimal.
    pub text_color: Option<String>,
    /// Descrição da label.
    pub description: Option<String>,
    /// Prioridade da label (quanto menor, maior a prioridade).
    pub priority: Option<i32>,
    /// Indica se o usuário atual está inscrito na label.
    pub subscribed: Option<bool>,
    /// Indica se a label é uma label de projeto (vs. label de grupo).
    pub is_project_label: Option<bool>,
}

/// Resposta da API GitLab representando uma label de grupo.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GroupLabel {
    /// ID único da label.
    pub id: GitLabId,
    /// Nome da label.
    pub name: String,
    /// Cor da label em formato hexadecimal (ex.: "#FF0000").
    pub color: Option<String>,
    /// Cor do texto da label em formato hexadecimal.
    pub text_color: Option<String>,
    /// Descrição da label.
    pub description: Option<String>,
    /// Prioridade da label (quanto menor, maior a prioridade).
    pub priority: Option<i32>,
    /// Indica se o usuário atual está inscrito na label.
    pub subscribed: Option<bool>,
    /// ID do grupo ao qual a label pertence.
    pub group_id: Option<GitLabId>,
}

/// Payload para criar uma label na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateLabelPayload {
    /// Nome da label.
    pub name: String,
    /// Cor da label em formato hexadecimal (ex.: "#FF0000").
    pub color: String,
    /// Descrição da label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Prioridade da label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
}

/// Payload para atualizar uma label na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateLabelPayload {
    /// Novo nome da label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Nova cor da label em formato hexadecimal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// Nova descrição da label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Nova prioridade da label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
}
