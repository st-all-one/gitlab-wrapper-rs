use serde::{Deserialize, Serialize};
use crate::types::base::*;

/// Resposta da API GitLab representando uma nota (comentário).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Note {
    /// ID da nota.
    pub id: GitLabId,
    /// Corpo da nota.
    pub body: Option<String>,
    /// Anexo da nota.
    pub attachment: Option<String>,
    /// Autor da nota.
    pub author: Option<AuthorInfo>,
    /// Data de criação.
    pub created_at: Option<String>,
    /// Data de atualização.
    pub updated_at: Option<String>,
    /// Indica se a nota foi criada pelo sistema.
    pub system: Option<bool>,
    /// ID do recurso noteable ao qual a nota pertence.
    pub noteable_id: Option<GitLabId>,
    /// Tipo do recurso noteable (ex: MergeRequest, Issue).
    pub noteable_type: Option<String>,
    /// ID do projeto.
    pub project_id: Option<GitLabId>,
    /// Indica se a nota é resolvível.
    pub resolvable: Option<bool>,
    /// Indica se a nota foi resolvida.
    pub resolved: Option<bool>,
    /// Usuário que resolveu a nota.
    pub resolved_by: Option<AuthorInfo>,
    /// Data em que a nota foi resolvida.
    pub resolved_at: Option<String>,
    /// Posição da nota (para notas em código).
    pub position: Option<NotePosition>,
    /// Links relacionados à nota.
    pub _links: Option<Links>,
}

/// Posição de uma nota em um diff de código.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NotePosition {
    /// SHA da base do diff.
    pub base_sha: Option<String>,
    /// SHA de início do diff.
    pub start_sha: Option<String>,
    /// SHA de cabeça do diff.
    pub head_sha: Option<String>,
    /// Tipo de posição (text, image).
    pub position_type: Option<String>,
    /// Caminho do arquivo novo.
    pub new_path: Option<String>,
    /// Caminho do arquivo antigo.
    pub old_path: Option<String>,
    /// Número da linha nova.
    pub new_line: Option<u32>,
    /// Número da linha antiga.
    pub old_line: Option<u32>,
    /// Intervalo de linhas.
    pub line_range: Option<serde_json::Value>,
}

/// Payload para criar uma nota na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateNotePayload {
    /// Corpo da nota (obrigatório).
    pub body: String,
    /// Indica se a nota é confidencial.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidential: Option<bool>,
}

/// Payload para atualizar uma nota na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateNotePayload {
    /// Novo corpo da nota (obrigatório).
    pub body: String,
    /// Indica se a nota é confidencial.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidential: Option<bool>,
}
