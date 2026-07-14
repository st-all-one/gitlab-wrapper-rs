use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use std::sync::Arc;

/// Recurso de API para operações com emojis de premiação (award emoji) no GitLab.
///
/// Suporta emojis em issues, merge requests e snippets.
#[derive(Debug)]
pub struct EmojiResource {
    http: Arc<HttpClient>,
}

impl EmojiResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    fn base_issue(project_id: u64, issue_iid: u32) -> String {
        format!("projects/{}/issues/{}/award_emoji", project_id, issue_iid)
    }

    fn base_mr(project_id: u64, mr_iid: u32) -> String {
        format!("projects/{}/merge_requests/{}/award_emoji", project_id, mr_iid)
    }

    fn base_snippet(project_id: u64, snippet_id: u64) -> String {
        format!("projects/{}/snippets/{}/award_emoji", project_id, snippet_id)
    }

    /// Lista emojis de premiação de uma issue.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `issue_iid`: IID da issue no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<AwardEmoji>, GitLabError>` — lista de emojis da issue.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list_issue_emoji(
        &self,
        project_id: u64,
        issue_iid: u32,
    ) -> Result<Vec<AwardEmoji>, GitLabError> {
        let path = Self::base_issue(project_id, issue_iid);
        self.http.get(&path, &[], "emoji.list_issue_emoji").await
    }

    /// Obtém um emoji de premiação específico de uma issue.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `issue_iid`: IID da issue no GitLab.
    /// - `award_id`: ID do emoji de premiação no GitLab.
    ///
    /// ## Returns
    /// `Result<AwardEmoji, GitLabError>` — dados do emoji solicitado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get_issue_emoji(
        &self,
        project_id: u64,
        issue_iid: u32,
        award_id: u64,
    ) -> Result<AwardEmoji, GitLabError> {
        let path = format!("{}/{}", Self::base_issue(project_id, issue_iid), award_id);
        self.http.get(&path, &[], "emoji.get_issue_emoji").await
    }

    /// Cria um emoji de premiação em uma issue.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `issue_iid`: IID da issue no GitLab.
    /// - `payload`: Dados para criar o emoji.
    ///
    /// ## Returns
    /// `Result<AwardEmoji, GitLabError>` — dados do emoji criado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn create_issue_emoji(
        &self,
        project_id: u64,
        issue_iid: u32,
        payload: &CreateEmojiPayload,
    ) -> Result<AwardEmoji, GitLabError> {
        let path = Self::base_issue(project_id, issue_iid);
        self.http.post(&path, &payload, "emoji.create_issue_emoji").await
    }

    /// Remove um emoji de premiação de uma issue.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `issue_iid`: IID da issue no GitLab.
    /// - `award_id`: ID do emoji de premiação no GitLab.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn delete_issue_emoji(
        &self,
        project_id: u64,
        issue_iid: u32,
        award_id: u64,
    ) -> Result<(), GitLabError> {
        let path = format!("{}/{}", Self::base_issue(project_id, issue_iid), award_id);
        self.http.delete(&path, &[], "emoji.delete_issue_emoji").await
    }

    /// Lista emojis de premiação de um merge request.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `mr_iid`: IID do merge request no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<AwardEmoji>, GitLabError>` — lista de emojis do merge request.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list_mr_emoji(
        &self,
        project_id: u64,
        mr_iid: u32,
    ) -> Result<Vec<AwardEmoji>, GitLabError> {
        let path = Self::base_mr(project_id, mr_iid);
        self.http.get(&path, &[], "emoji.list_mr_emoji").await
    }

    /// Obtém um emoji de premiação específico de um merge request.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `mr_iid`: IID do merge request no GitLab.
    /// - `award_id`: ID do emoji de premiação no GitLab.
    ///
    /// ## Returns
    /// `Result<AwardEmoji, GitLabError>` — dados do emoji solicitado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get_mr_emoji(
        &self,
        project_id: u64,
        mr_iid: u32,
        award_id: u64,
    ) -> Result<AwardEmoji, GitLabError> {
        let path = format!("{}/{}", Self::base_mr(project_id, mr_iid), award_id);
        self.http.get(&path, &[], "emoji.get_mr_emoji").await
    }

    /// Cria um emoji de premiação em um merge request.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `mr_iid`: IID do merge request no GitLab.
    /// - `payload`: Dados para criar o emoji.
    ///
    /// ## Returns
    /// `Result<AwardEmoji, GitLabError>` — dados do emoji criado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn create_mr_emoji(
        &self,
        project_id: u64,
        mr_iid: u32,
        payload: &CreateEmojiPayload,
    ) -> Result<AwardEmoji, GitLabError> {
        let path = Self::base_mr(project_id, mr_iid);
        self.http.post(&path, &payload, "emoji.create_mr_emoji").await
    }

    /// Remove um emoji de premiação de um merge request.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `mr_iid`: IID do merge request no GitLab.
    /// - `award_id`: ID do emoji de premiação no GitLab.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn delete_mr_emoji(
        &self,
        project_id: u64,
        mr_iid: u32,
        award_id: u64,
    ) -> Result<(), GitLabError> {
        let path = format!("{}/{}", Self::base_mr(project_id, mr_iid), award_id);
        self.http.delete(&path, &[], "emoji.delete_mr_emoji").await
    }

    /// Lista emojis de premiação de um snippet.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `snippet_id`: ID do snippet no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<AwardEmoji>, GitLabError>` — lista de emojis do snippet.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list_snippet_emoji(
        &self,
        project_id: u64,
        snippet_id: u64,
    ) -> Result<Vec<AwardEmoji>, GitLabError> {
        let path = Self::base_snippet(project_id, snippet_id);
        self.http.get(&path, &[], "emoji.list_snippet_emoji").await
    }

    /// Cria um emoji de premiação em um snippet.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `snippet_id`: ID do snippet no GitLab.
    /// - `payload`: Dados para criar o emoji.
    ///
    /// ## Returns
    /// `Result<AwardEmoji, GitLabError>` — dados do emoji criado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn create_snippet_emoji(
        &self,
        project_id: u64,
        snippet_id: u64,
        payload: &CreateEmojiPayload,
    ) -> Result<AwardEmoji, GitLabError> {
        let path = Self::base_snippet(project_id, snippet_id);
        self.http.post(&path, &payload, "emoji.create_snippet_emoji").await
    }

    /// Remove um emoji de premiação de um snippet.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `snippet_id`: ID do snippet no GitLab.
    /// - `award_id`: ID do emoji de premiação no GitLab.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn delete_snippet_emoji(
        &self,
        project_id: u64,
        snippet_id: u64,
        award_id: u64,
    ) -> Result<(), GitLabError> {
        let path = format!("{}/{}", Self::base_snippet(project_id, snippet_id), award_id);
        self.http.delete(&path, &[], "emoji.delete_snippet_emoji").await
    }
}
