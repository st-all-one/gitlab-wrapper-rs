use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use std::sync::Arc;

/// Recurso de API para operações com notas no GitLab.
#[derive(Debug)]
pub struct NotesResource {
    http: Arc<HttpClient>,
}

impl NotesResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    fn base_issue(project_id: u64, issue_iid: u32) -> String {
        format!("projects/{}/issues/{}/notes", project_id, issue_iid)
    }

    fn base_mr(project_id: u64, mr_iid: u32) -> String {
        format!("projects/{}/merge_requests/{}/notes", project_id, mr_iid)
    }

    fn base_commit(project_id: u64, sha: &str) -> String {
        format!("projects/{}/repository/commits/{}/notes", project_id, sha)
    }

    fn base_snippet(project_id: u64, snippet_id: u64) -> String {
        format!("projects/{}/snippets/{}/notes", project_id, snippet_id)
    }

    fn base_wiki(project_id: u64, slug: &str) -> String {
        format!("projects/{}/wikis/{}/notes", project_id, slug)
    }

    /// Lista notas de uma issue.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `issue_iid`: IID da issue no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<Note>, GitLabError>` — lista de notas da issue.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list_issue_notes(
        &self,
        project_id: u64,
        issue_iid: u32,
    ) -> Result<Vec<Note>, GitLabError> {
        let path = Self::base_issue(project_id, issue_iid);
        self.http.get(&path, &[], "notes.list_issue").await
    }

    /// Cria uma nota em uma issue.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `issue_iid`: IID da issue no GitLab.
    /// - `payload`: Dados para criar a nota.
    ///
    /// ## Returns
    /// `Result<Note, GitLabError>` — dados da nota criada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn create_issue_note(
        &self,
        project_id: u64,
        issue_iid: u32,
        payload: &CreateNotePayload,
    ) -> Result<Note, GitLabError> {
        let path = Self::base_issue(project_id, issue_iid);
        self.http.post(&path, &payload, "notes.create_issue").await
    }

    /// Obtém uma nota específica de uma issue.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `issue_iid`: IID da issue no GitLab.
    /// - `note_id`: ID da nota no GitLab.
    ///
    /// ## Returns
    /// `Result<Note, GitLabError>` — dados da nota solicitada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get_issue_note(
        &self,
        project_id: u64,
        issue_iid: u32,
        note_id: u64,
    ) -> Result<Note, GitLabError> {
        let path = format!("{}/{}", Self::base_issue(project_id, issue_iid), note_id);
        self.http.get(&path, &[], "notes.get_issue").await
    }

    /// Atualiza uma nota de uma issue.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `issue_iid`: IID da issue no GitLab.
    /// - `note_id`: ID da nota no GitLab.
    /// - `payload`: Dados para atualizar a nota.
    ///
    /// ## Returns
    /// `Result<Note, GitLabError>` — dados da nota atualizada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn update_issue_note(
        &self,
        project_id: u64,
        issue_iid: u32,
        note_id: u64,
        payload: &UpdateNotePayload,
    ) -> Result<Note, GitLabError> {
        let path = format!("{}/{}", Self::base_issue(project_id, issue_iid), note_id);
        self.http.put(&path, &payload, "notes.update_issue").await
    }

    /// Remove uma nota de uma issue.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `issue_iid`: IID da issue no GitLab.
    /// - `note_id`: ID da nota no GitLab.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn delete_issue_note(
        &self,
        project_id: u64,
        issue_iid: u32,
        note_id: u64,
    ) -> Result<(), GitLabError> {
        let path = format!("{}/{}", Self::base_issue(project_id, issue_iid), note_id);
        self.http.delete(&path, &[], "notes.delete_issue").await
    }

    /// Lista notas de um merge request.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `mr_iid`: IID do merge request no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<Note>, GitLabError>` — lista de notas do merge request.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list_mr_notes(
        &self,
        project_id: u64,
        mr_iid: u32,
    ) -> Result<Vec<Note>, GitLabError> {
        let path = Self::base_mr(project_id, mr_iid);
        self.http.get(&path, &[], "notes.list_mr").await
    }

    /// Cria uma nota em um merge request.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `mr_iid`: IID do merge request no GitLab.
    /// - `payload`: Dados para criar a nota.
    ///
    /// ## Returns
    /// `Result<Note, GitLabError>` — dados da nota criada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn create_mr_note(
        &self,
        project_id: u64,
        mr_iid: u32,
        payload: &CreateNotePayload,
    ) -> Result<Note, GitLabError> {
        let path = Self::base_mr(project_id, mr_iid);
        self.http.post(&path, &payload, "notes.create_mr").await
    }

    /// Obtém uma nota específica de um merge request.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `mr_iid`: IID do merge request no GitLab.
    /// - `note_id`: ID da nota no GitLab.
    ///
    /// ## Returns
    /// `Result<Note, GitLabError>` — dados da nota solicitada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get_mr_note(
        &self,
        project_id: u64,
        mr_iid: u32,
        note_id: u64,
    ) -> Result<Note, GitLabError> {
        let path = format!("{}/{}", Self::base_mr(project_id, mr_iid), note_id);
        self.http.get(&path, &[], "notes.get_mr").await
    }

    /// Atualiza uma nota de um merge request.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `mr_iid`: IID do merge request no GitLab.
    /// - `note_id`: ID da nota no GitLab.
    /// - `payload`: Dados para atualizar a nota.
    ///
    /// ## Returns
    /// `Result<Note, GitLabError>` — dados da nota atualizada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn update_mr_note(
        &self,
        project_id: u64,
        mr_iid: u32,
        note_id: u64,
        payload: &UpdateNotePayload,
    ) -> Result<Note, GitLabError> {
        let path = format!("{}/{}", Self::base_mr(project_id, mr_iid), note_id);
        self.http.put(&path, &payload, "notes.update_mr").await
    }

    /// Remove uma nota de um merge request.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `mr_iid`: IID do merge request no GitLab.
    /// - `note_id`: ID da nota no GitLab.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn delete_mr_note(
        &self,
        project_id: u64,
        mr_iid: u32,
        note_id: u64,
    ) -> Result<(), GitLabError> {
        let path = format!("{}/{}", Self::base_mr(project_id, mr_iid), note_id);
        self.http.delete(&path, &[], "notes.delete_mr").await
    }

    /// Lista notas de um commit.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `sha`: SHA do commit no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<Note>, GitLabError>` — lista de notas do commit.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list_commit_notes(
        &self,
        project_id: u64,
        sha: &str,
    ) -> Result<Vec<Note>, GitLabError> {
        let path = Self::base_commit(project_id, sha);
        self.http.get(&path, &[], "notes.list_commit").await
    }

    /// Cria uma nota em um commit.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `sha`: SHA do commit no GitLab.
    /// - `payload`: Dados para criar a nota.
    ///
    /// ## Returns
    /// `Result<Note, GitLabError>` — dados da nota criada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn create_commit_note(
        &self,
        project_id: u64,
        sha: &str,
        payload: &CreateNotePayload,
    ) -> Result<Note, GitLabError> {
        let path = Self::base_commit(project_id, sha);
        self.http.post(&path, &payload, "notes.create_commit").await
    }

    /// Obtém uma nota específica de um commit.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `sha`: SHA do commit no GitLab.
    /// - `note_id`: ID da nota no GitLab.
    ///
    /// ## Returns
    /// `Result<Note, GitLabError>` — dados da nota solicitada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get_commit_note(
        &self,
        project_id: u64,
        sha: &str,
        note_id: u64,
    ) -> Result<Note, GitLabError> {
        let path = format!("{}/{}", Self::base_commit(project_id, sha), note_id);
        self.http.get(&path, &[], "notes.get_commit").await
    }

    /// Atualiza uma nota de um commit.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `sha`: SHA do commit no GitLab.
    /// - `note_id`: ID da nota no GitLab.
    /// - `payload`: Dados para atualizar a nota.
    ///
    /// ## Returns
    /// `Result<Note, GitLabError>` — dados da nota atualizada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn update_commit_note(
        &self,
        project_id: u64,
        sha: &str,
        note_id: u64,
        payload: &UpdateNotePayload,
    ) -> Result<Note, GitLabError> {
        let path = format!("{}/{}", Self::base_commit(project_id, sha), note_id);
        self.http.put(&path, &payload, "notes.update_commit").await
    }

    /// Remove uma nota de um commit.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `sha`: SHA do commit no GitLab.
    /// - `note_id`: ID da nota no GitLab.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn delete_commit_note(
        &self,
        project_id: u64,
        sha: &str,
        note_id: u64,
    ) -> Result<(), GitLabError> {
        let path = format!("{}/{}", Self::base_commit(project_id, sha), note_id);
        self.http.delete(&path, &[], "notes.delete_commit").await
    }

    /// Lista notas de um snippet.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `snippet_id`: ID do snippet no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<Note>, GitLabError>` — lista de notas do snippet.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list_snippet_notes(
        &self,
        project_id: u64,
        snippet_id: u64,
    ) -> Result<Vec<Note>, GitLabError> {
        let path = Self::base_snippet(project_id, snippet_id);
        self.http.get(&path, &[], "notes.list_snippet").await
    }

    /// Cria uma nota em um snippet.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `snippet_id`: ID do snippet no GitLab.
    /// - `payload`: Dados para criar a nota.
    ///
    /// ## Returns
    /// `Result<Note, GitLabError>` — dados da nota criada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn create_snippet_note(
        &self,
        project_id: u64,
        snippet_id: u64,
        payload: &CreateNotePayload,
    ) -> Result<Note, GitLabError> {
        let path = Self::base_snippet(project_id, snippet_id);
        self.http.post(&path, &payload, "notes.create_snippet").await
    }

    /// Obtém uma nota específica de um snippet.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `snippet_id`: ID do snippet no GitLab.
    /// - `note_id`: ID da nota no GitLab.
    ///
    /// ## Returns
    /// `Result<Note, GitLabError>` — dados da nota solicitada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get_snippet_note(
        &self,
        project_id: u64,
        snippet_id: u64,
        note_id: u64,
    ) -> Result<Note, GitLabError> {
        let path = format!("{}/{}", Self::base_snippet(project_id, snippet_id), note_id);
        self.http.get(&path, &[], "notes.get_snippet").await
    }

    /// Atualiza uma nota de um snippet.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `snippet_id`: ID do snippet no GitLab.
    /// - `note_id`: ID da nota no GitLab.
    /// - `payload`: Dados para atualizar a nota.
    ///
    /// ## Returns
    /// `Result<Note, GitLabError>` — dados da nota atualizada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn update_snippet_note(
        &self,
        project_id: u64,
        snippet_id: u64,
        note_id: u64,
        payload: &UpdateNotePayload,
    ) -> Result<Note, GitLabError> {
        let path = format!("{}/{}", Self::base_snippet(project_id, snippet_id), note_id);
        self.http.put(&path, &payload, "notes.update_snippet").await
    }

    /// Remove uma nota de um snippet.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `snippet_id`: ID do snippet no GitLab.
    /// - `note_id`: ID da nota no GitLab.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn delete_snippet_note(
        &self,
        project_id: u64,
        snippet_id: u64,
        note_id: u64,
    ) -> Result<(), GitLabError> {
        let path = format!("{}/{}", Self::base_snippet(project_id, snippet_id), note_id);
        self.http.delete(&path, &[], "notes.delete_snippet").await
    }

    // -- Wiki notes --

    /// Lista notas de uma wiki.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `slug`: Slug da página wiki no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<Note>, GitLabError>` — lista de notas da wiki.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list_wiki_notes(
        &self,
        project_id: u64,
        slug: &str,
    ) -> Result<Vec<Note>, GitLabError> {
        let path = Self::base_wiki(project_id, slug);
        self.http.get(&path, &[], "notes.list_wiki").await
    }

    /// Obtém uma nota específica de uma wiki.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `slug`: Slug da página wiki no GitLab.
    /// - `note_id`: ID da nota no GitLab.
    ///
    /// ## Returns
    /// `Result<Note, GitLabError>` — dados da nota solicitada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get_wiki_note(
        &self,
        project_id: u64,
        slug: &str,
        note_id: u64,
    ) -> Result<Note, GitLabError> {
        let path = format!("{}/{}", Self::base_wiki(project_id, slug), note_id);
        self.http.get(&path, &[], "notes.get_wiki").await
    }

    /// Cria uma nota em uma página wiki.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `slug`: Slug da página wiki no GitLab.
    /// - `payload`: Dados para criar a nota.
    ///
    /// ## Returns
    /// `Result<Note, GitLabError>` — dados da nota criada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn create_wiki_note(
        &self,
        project_id: u64,
        slug: &str,
        payload: &CreateNotePayload,
    ) -> Result<Note, GitLabError> {
        let path = Self::base_wiki(project_id, slug);
        self.http.post(&path, &payload, "notes.create_wiki").await
    }

    /// Atualiza uma nota de uma página wiki.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `slug`: Slug da página wiki no GitLab.
    /// - `note_id`: ID da nota no GitLab.
    /// - `payload`: Dados para atualizar a nota.
    ///
    /// ## Returns
    /// `Result<Note, GitLabError>` — dados da nota atualizada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn update_wiki_note(
        &self,
        project_id: u64,
        slug: &str,
        note_id: u64,
        payload: &UpdateNotePayload,
    ) -> Result<Note, GitLabError> {
        let path = format!("{}/{}", Self::base_wiki(project_id, slug), note_id);
        self.http.put(&path, &payload, "notes.update_wiki").await
    }

    /// Remove uma nota de uma página wiki.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `slug`: Slug da página wiki no GitLab.
    /// - `note_id`: ID da nota no GitLab.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn delete_wiki_note(
        &self,
        project_id: u64,
        slug: &str,
        note_id: u64,
    ) -> Result<(), GitLabError> {
        let path = format!("{}/{}", Self::base_wiki(project_id, slug), note_id);
        self.http.delete(&path, &[], "notes.delete_wiki").await
    }
}
