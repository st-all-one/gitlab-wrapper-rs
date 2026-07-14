use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use std::sync::Arc;

/// Recurso de API para operações com notas em rascunho (draft notes) no GitLab.
#[derive(Debug)]
pub struct DraftNotesResource {
    http: Arc<HttpClient>,
}

impl DraftNotesResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista todas as notas em rascunho de um merge request.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `mr_iid`: IID do merge request no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<DraftNote>, GitLabError>` — lista de notas em rascunho.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list(&self, project_id: u64, mr_iid: u32) -> Result<Vec<DraftNote>, GitLabError> {
        let path = format!("projects/{}/merge_requests/{}/draft_notes", project_id, mr_iid);
        self.http.get(&path, &[], "draft_notes.list").await
    }

    /// Cria uma nova nota em rascunho em um merge request.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `mr_iid`: IID do merge request no GitLab.
    /// - `payload`: Dados para criar a nota em rascunho.
    ///
    /// ## Returns
    /// `Result<DraftNote, GitLabError>` — dados da nota em rascunho criada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn create(
        &self,
        project_id: u64,
        mr_iid: u32,
        payload: &CreateDraftNotePayload,
    ) -> Result<DraftNote, GitLabError> {
        let path = format!("projects/{}/merge_requests/{}/draft_notes", project_id, mr_iid);
        self.http.post(&path, &payload, "draft_notes.create").await
    }

    /// Atualiza uma nota em rascunho existente.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `mr_iid`: IID do merge request no GitLab.
    /// - `draft_note_id`: ID da nota em rascunho no GitLab.
    /// - `payload`: Dados para atualizar a nota em rascunho.
    ///
    /// ## Returns
    /// `Result<DraftNote, GitLabError>` — dados da nota em rascunho atualizada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn update(
        &self,
        project_id: u64,
        mr_iid: u32,
        draft_note_id: u64,
        payload: &UpdateDraftNotePayload,
    ) -> Result<DraftNote, GitLabError> {
        let path = format!(
            "projects/{}/merge_requests/{}/draft_notes/{}",
            project_id, mr_iid, draft_note_id
        );
        self.http.put(&path, &payload, "draft_notes.update").await
    }

    /// Remove uma nota em rascunho.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `mr_iid`: IID do merge request no GitLab.
    /// - `draft_note_id`: ID da nota em rascunho no GitLab.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn delete(
        &self,
        project_id: u64,
        mr_iid: u32,
        draft_note_id: u64,
    ) -> Result<(), GitLabError> {
        let path = format!(
            "projects/{}/merge_requests/{}/draft_notes/{}",
            project_id, mr_iid, draft_note_id
        );
        self.http.delete(&path, &[], "draft_notes.delete").await
    }

    /// Publica todas as notas em rascunho de um merge request.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `mr_iid`: IID do merge request no GitLab.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn publish(&self, project_id: u64, mr_iid: u32) -> Result<(), GitLabError> {
        let path = format!("projects/{}/merge_requests/{}/draft_notes/publish", project_id, mr_iid);
        self.http.post(&path, &serde_json::json!({}), "draft_notes.publish").await
    }
}
