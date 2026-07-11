use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::utils::encoding::encode_query_param;
use std::sync::Arc;
use crate::types::*;

/// Recurso de API para operações com discussões no GitLab.
#[derive(Debug)]
pub struct DiscussionsResource {
    http: Arc<HttpClient>,
}

impl DiscussionsResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    fn base_issue(project_id: u64, issue_iid: u32) -> String {
        format!("projects/{}/issues/{}/discussions", project_id, issue_iid)
    }

    fn base_mr(project_id: u64, mr_iid: u32) -> String {
        format!("projects/{}/merge_requests/{}/discussions", project_id, mr_iid)
    }

    fn base_commit(project_id: u64, sha: &str) -> String {
        format!("projects/{}/repository/commits/{}/discussions", project_id, sha)
    }

    /// Lista discussões de uma issue.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `issue_iid`: IID da issue no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<Discussion>, GitLabError>` — lista de discussões da issue.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn list_issue_discussions(&self, project_id: u64, issue_iid: u32) -> Result<Vec<Discussion>, GitLabError> {
        let path = Self::base_issue(project_id, issue_iid);
        self.http.get(&path, &[], "discussions.list_issue")
    }

    /// Cria uma discussão em uma issue.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `issue_iid`: IID da issue no GitLab.
    /// - `body`: Dados para criar a discussão.
    ///
    /// ## Returns
    /// `Result<Discussion, GitLabError>` — dados da discussão criada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn create_issue_discussion(&self, project_id: u64, issue_iid: u32, body: &CreateDiscussionPayload) -> Result<Discussion, GitLabError> {
        let path = Self::base_issue(project_id, issue_iid);
        self.http.post(&path, &body, "discussions.create_issue")
    }

    /// Adiciona uma nota a uma discussão de issue.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `issue_iid`: IID da issue no GitLab.
    /// - `discussion_id`: ID da discussão no GitLab.
    /// - `note`: Dados para criar a nota.
    ///
    /// ## Returns
    /// `Result<Note, GitLabError>` — dados da nota adicionada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn add_issue_discussion_note(&self, project_id: u64, issue_iid: u32, discussion_id: &str, note: &CreateNotePayload) -> Result<Note, GitLabError> {
        let path = format!("{}/{}/notes", Self::base_issue(project_id, issue_iid), discussion_id);
        self.http.post(&path, &note, "discussions.add_issue_note")
    }

    /// Obtém uma discussão específica de uma issue.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `issue_iid`: IID da issue no GitLab.
    /// - `discussion_id`: ID da discussão no GitLab.
    ///
    /// ## Returns
    /// `Result<Discussion, GitLabError>` — dados da discussão solicitada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn get_issue_discussion(&self, project_id: u64, issue_iid: u32, discussion_id: &str) -> Result<Discussion, GitLabError> {
        let path = format!("{}/{}", Self::base_issue(project_id, issue_iid), encode_query_param(discussion_id));
        self.http.get(&path, &[], "discussions.get_issue")
    }

    /// Atualiza uma nota de uma discussão de issue.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `issue_iid`: IID da issue no GitLab.
    /// - `discussion_id`: ID da discussão no GitLab.
    /// - `note_id`: ID da nota no GitLab.
    /// - `body`: Novo corpo da nota.
    ///
    /// ## Returns
    /// `Result<Note, GitLabError>` — dados da nota atualizada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn update_issue_discussion_note(&self, project_id: u64, issue_iid: u32, discussion_id: &str, note_id: u64, body: &str) -> Result<Note, GitLabError> {
        let path = format!("{}/{}/notes/{}", Self::base_issue(project_id, issue_iid), encode_query_param(discussion_id), note_id);
        let payload = serde_json::json!({ "body": body });
        self.http.put(&path, &payload, "discussions.update_issue_note")
    }

    /// Remove uma nota de uma discussão de issue.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `issue_iid`: IID da issue no GitLab.
    /// - `discussion_id`: ID da discussão no GitLab.
    /// - `note_id`: ID da nota no GitLab.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn delete_issue_discussion_note(&self, project_id: u64, issue_iid: u32, discussion_id: &str, note_id: u64) -> Result<(), GitLabError> {
        let path = format!("{}/{}/notes/{}", Self::base_issue(project_id, issue_iid), encode_query_param(discussion_id), note_id);
        self.http.delete(&path, &[], "discussions.delete_issue_note")
    }

    /// Resolve ou não resolve uma discussão de issue.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `issue_iid`: IID da issue no GitLab.
    /// - `discussion_id`: ID da discussão no GitLab.
    /// - `resolved`: `true` para resolver, `false` para reabrir.
    ///
    /// ## Returns
    /// `Result<Discussion, GitLabError>` — dados da discussão atualizada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn resolve_issue_discussion(&self, project_id: u64, issue_iid: u32, discussion_id: &str, resolved: bool) -> Result<Discussion, GitLabError> {
        let path = format!("{}/{}", Self::base_issue(project_id, issue_iid), encode_query_param(discussion_id));
        let payload = serde_json::json!({ "resolved": resolved });
        self.http.put(&path, &payload, "discussions.resolve_issue")
    }

    /// Lista discussões de um merge request.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `mr_iid`: IID do merge request no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<Discussion>, GitLabError>` — lista de discussões do merge request.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn list_mr_discussions(&self, project_id: u64, mr_iid: u32) -> Result<Vec<Discussion>, GitLabError> {
        let path = Self::base_mr(project_id, mr_iid);
        self.http.get(&path, &[], "discussions.list_mr")
    }

    /// Cria uma discussão em um merge request.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `mr_iid`: IID do merge request no GitLab.
    /// - `body`: Dados para criar a discussão.
    ///
    /// ## Returns
    /// `Result<Discussion, GitLabError>` — dados da discussão criada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn create_mr_discussion(&self, project_id: u64, mr_iid: u32, body: &CreateDiscussionPayload) -> Result<Discussion, GitLabError> {
        let path = Self::base_mr(project_id, mr_iid);
        self.http.post(&path, &body, "discussions.create_mr")
    }

    /// Adiciona uma nota a uma discussão de merge request.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `mr_iid`: IID do merge request no GitLab.
    /// - `discussion_id`: ID da discussão no GitLab.
    /// - `note`: Dados para criar a nota.
    ///
    /// ## Returns
    /// `Result<Note, GitLabError>` — dados da nota adicionada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn add_mr_discussion_note(&self, project_id: u64, mr_iid: u32, discussion_id: &str, note: &CreateNotePayload) -> Result<Note, GitLabError> {
        let path = format!("{}/{}/notes", Self::base_mr(project_id, mr_iid), discussion_id);
        self.http.post(&path, &note, "discussions.add_mr_note")
    }

    /// Obtém uma discussão específica de um merge request.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `mr_iid`: IID do merge request no GitLab.
    /// - `discussion_id`: ID da discussão no GitLab.
    ///
    /// ## Returns
    /// `Result<Discussion, GitLabError>` — dados da discussão solicitada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn get_mr_discussion(&self, project_id: u64, mr_iid: u32, discussion_id: &str) -> Result<Discussion, GitLabError> {
        let path = format!("{}/{}", Self::base_mr(project_id, mr_iid), encode_query_param(discussion_id));
        self.http.get(&path, &[], "discussions.get_mr")
    }

    /// Atualiza uma nota de uma discussão de merge request.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `mr_iid`: IID do merge request no GitLab.
    /// - `discussion_id`: ID da discussão no GitLab.
    /// - `note_id`: ID da nota no GitLab.
    /// - `body`: Novo corpo da nota.
    ///
    /// ## Returns
    /// `Result<Note, GitLabError>` — dados da nota atualizada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn update_mr_discussion_note(&self, project_id: u64, mr_iid: u32, discussion_id: &str, note_id: u64, body: &str) -> Result<Note, GitLabError> {
        let path = format!("{}/{}/notes/{}", Self::base_mr(project_id, mr_iid), encode_query_param(discussion_id), note_id);
        let payload = serde_json::json!({ "body": body });
        self.http.put(&path, &payload, "discussions.update_mr_note")
    }

    /// Remove uma nota de uma discussão de merge request.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `mr_iid`: IID do merge request no GitLab.
    /// - `discussion_id`: ID da discussão no GitLab.
    /// - `note_id`: ID da nota no GitLab.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn delete_mr_discussion_note(&self, project_id: u64, mr_iid: u32, discussion_id: &str, note_id: u64) -> Result<(), GitLabError> {
        let path = format!("{}/{}/notes/{}", Self::base_mr(project_id, mr_iid), encode_query_param(discussion_id), note_id);
        self.http.delete(&path, &[], "discussions.delete_mr_note")
    }

    /// Resolve ou não resolve uma discussão de merge request.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `mr_iid`: IID do merge request no GitLab.
    /// - `discussion_id`: ID da discussão no GitLab.
    /// - `resolved`: `true` para resolver, `false` para reabrir.
    ///
    /// ## Returns
    /// `Result<Discussion, GitLabError>` — dados da discussão atualizada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn resolve_mr_discussion(&self, project_id: u64, mr_iid: u32, discussion_id: &str, resolved: bool) -> Result<Discussion, GitLabError> {
        let path = format!("{}/{}", Self::base_mr(project_id, mr_iid), encode_query_param(discussion_id));
        let payload = serde_json::json!({ "resolved": resolved });
        self.http.put(&path, &payload, "discussions.resolve_mr")
    }

    /// Lista discussões de um commit.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `sha`: SHA do commit no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<Discussion>, GitLabError>` — lista de discussões do commit.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn list_commit_discussions(&self, project_id: u64, sha: &str) -> Result<Vec<Discussion>, GitLabError> {
        let path = Self::base_commit(project_id, sha);
        self.http.get(&path, &[], "discussions.list_commit")
    }

    /// Cria uma discussão em um commit.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `sha`: SHA do commit no GitLab.
    /// - `body`: Dados para criar a discussão.
    ///
    /// ## Returns
    /// `Result<Discussion, GitLabError>` — dados da discussão criada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn create_commit_discussion(&self, project_id: u64, sha: &str, body: &CreateDiscussionPayload) -> Result<Discussion, GitLabError> {
        let path = Self::base_commit(project_id, sha);
        self.http.post(&path, &body, "discussions.create_commit")
    }

    /// Adiciona uma nota a uma discussão de commit.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `sha`: SHA do commit no GitLab.
    /// - `discussion_id`: ID da discussão no GitLab.
    /// - `note`: Dados para criar a nota.
    ///
    /// ## Returns
    /// `Result<Note, GitLabError>` — dados da nota adicionada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn add_commit_discussion_note(&self, project_id: u64, sha: &str, discussion_id: &str, note: &CreateNotePayload) -> Result<Note, GitLabError> {
        let path = format!("{}/{}/notes", Self::base_commit(project_id, sha), discussion_id);
        self.http.post(&path, &note, "discussions.add_commit_note")
    }

    /// Obtém uma discussão específica de um commit.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `sha`: SHA do commit no GitLab.
    /// - `discussion_id`: ID da discussão no GitLab.
    ///
    /// ## Returns
    /// `Result<Discussion, GitLabError>` — dados da discussão solicitada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn get_commit_discussion(&self, project_id: u64, sha: &str, discussion_id: &str) -> Result<Discussion, GitLabError> {
        let path = format!("{}/{}", Self::base_commit(project_id, sha), encode_query_param(discussion_id));
        self.http.get(&path, &[], "discussions.get_commit")
    }

    /// Atualiza uma nota de uma discussão de commit.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `sha`: SHA do commit no GitLab.
    /// - `discussion_id`: ID da discussão no GitLab.
    /// - `note_id`: ID da nota no GitLab.
    /// - `body`: Novo corpo da nota.
    ///
    /// ## Returns
    /// `Result<Note, GitLabError>` — dados da nota atualizada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn update_commit_discussion_note(&self, project_id: u64, sha: &str, discussion_id: &str, note_id: u64, body: &str) -> Result<Note, GitLabError> {
        let path = format!("{}/{}/notes/{}", Self::base_commit(project_id, sha), encode_query_param(discussion_id), note_id);
        let payload = serde_json::json!({ "body": body });
        self.http.put(&path, &payload, "discussions.update_commit_note")
    }

    /// Remove uma nota de uma discussão de commit.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `sha`: SHA do commit no GitLab.
    /// - `discussion_id`: ID da discussão no GitLab.
    /// - `note_id`: ID da nota no GitLab.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn delete_commit_discussion_note(&self, project_id: u64, sha: &str, discussion_id: &str, note_id: u64) -> Result<(), GitLabError> {
        let path = format!("{}/{}/notes/{}", Self::base_commit(project_id, sha), encode_query_param(discussion_id), note_id);
        self.http.delete(&path, &[], "discussions.delete_commit_note")
    }

    /// Resolve ou não resolve uma discussão de commit.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `sha`: SHA do commit no GitLab.
    /// - `discussion_id`: ID da discussão no GitLab.
    /// - `resolved`: `true` para resolver, `false` para reabrir.
    ///
    /// ## Returns
    /// `Result<Discussion, GitLabError>` — dados da discussão atualizada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn resolve_commit_discussion(&self, project_id: u64, sha: &str, discussion_id: &str, resolved: bool) -> Result<Discussion, GitLabError> {
        let path = format!("{}/{}", Self::base_commit(project_id, sha), encode_query_param(discussion_id));
        let payload = serde_json::json!({ "resolved": resolved });
        self.http.put(&path, &payload, "discussions.resolve_commit")
    }
}
