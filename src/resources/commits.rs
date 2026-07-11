use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use std::sync::Arc;
use crate::types::*;
use crate::utils::encoding::filter_to_query;

/// Recurso de API para operações com commits no GitLab.
#[derive(Debug)]
pub struct CommitsResource {
    http: Arc<HttpClient>,
}

impl CommitsResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista commits de um projeto com filtros opcionais.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `filter`: Filtros opcionais para a consulta.
    ///
    /// ## Returns
    /// `Result<Vec<Commit>, GitLabError>` — lista de commits.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn list(&self, project_id: u64, filter: Option<&CommitFilter>) -> Result<Vec<Commit>, GitLabError> {
        let path = format!("projects/{}/repository/commits", project_id);
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "commits.list")
    }

    /// Obtém um commit pelo SHA.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `sha`: SHA do commit.
    ///
    /// ## Returns
    /// `Result<Commit, GitLabError>` — dados do commit solicitado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn get(&self, project_id: u64, sha: &str) -> Result<Commit, GitLabError> {
        let path = format!("projects/{}/repository/commits/{}", project_id, sha);
        self.http.get(&path, &[], "commits.get")
    }

    /// Cria um novo commit em um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `payload`: Dados para criar o commit.
    ///
    /// ## Returns
    /// `Result<Commit, GitLabError>` — dados do commit criado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn create(&self, project_id: u64, payload: &CreateCommitPayload) -> Result<Commit, GitLabError> {
        let path = format!("projects/{}/repository/commits", project_id);
        self.http.post(&path, &payload, "commits.create")
    }

    /// Aplica um cherry-pick de um commit para outra branch.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `sha`: SHA do commit a ser cherry-picked.
    /// - `target_branch`: Nome da branch de destino.
    ///
    /// ## Returns
    /// `Result<Commit, GitLabError>` — dados do novo commit criado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn cherry_pick(&self, project_id: u64, sha: &str, target_branch: &str) -> Result<Commit, GitLabError> {
        let path = format!("projects/{}/repository/commits/{}/cherry_pick", project_id, sha);
        let body = serde_json::json!({ "branch": target_branch });
        self.http.post(&path, &body, "commits.cherry_pick")
    }

    /// Reverte um commit em uma branch alvo.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `sha`: SHA do commit a ser revertido.
    /// - `target_branch`: Nome da branch onde o revert será aplicado.
    ///
    /// ## Returns
    /// `Result<Commit, GitLabError>` — dados do commit de reversão.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn revert(&self, project_id: u64, sha: &str, target_branch: &str) -> Result<Commit, GitLabError> {
        let path = format!("projects/{}/repository/commits/{}/revert", project_id, sha);
        let body = serde_json::json!({ "branch": target_branch });
        self.http.post(&path, &body, "commits.revert")
    }

    /// Obtém o diff de um commit.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `sha`: SHA do commit.
    ///
    /// ## Returns
    /// `Result<Vec<CommitDiff>, GitLabError>` — lista de diferenças do commit.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn diff(&self, project_id: u64, sha: &str) -> Result<Vec<CommitDiff>, GitLabError> {
        let path = format!("projects/{}/repository/commits/{}/diff", project_id, sha);
        self.http.get(&path, &[], "commits.diff")
    }

    /// Obtém as referências (branches/tags) que contêm um commit.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `sha`: SHA do commit.
    ///
    /// ## Returns
    /// `Result<serde_json::Value, GitLabError>` — dados das referências do commit.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn refs(&self, project_id: u64, sha: &str) -> Result<serde_json::Value, GitLabError> {
        let path = format!("projects/{}/repository/commits/{}/refs", project_id, sha);
        self.http.get(&path, &[], "commits.refs")
    }

    /// Lista comentários de um commit.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `sha`: SHA do commit.
    ///
    /// ## Returns
    /// `Result<Vec<Note>, GitLabError>` — lista de comentários do commit.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn comments(&self, project_id: u64, sha: &str) -> Result<Vec<Note>, GitLabError> {
        let path = format!("projects/{}/repository/commits/{}/comments", project_id, sha);
        self.http.get(&path, &[], "commits.comments")
    }

    /// Adiciona um comentário a um commit.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `sha`: SHA do commit.
    /// - `note`: Texto do comentário.
    ///
    /// ## Returns
    /// `Result<Note, GitLabError>` — dados do comentário criado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn add_comment(&self, project_id: u64, sha: &str, note: &str) -> Result<Note, GitLabError> {
        let path = format!("projects/{}/repository/commits/{}/comments", project_id, sha);
        let body = serde_json::json!({ "note": note });
        self.http.post(&path, &body, "commits.add_comment")
    }
}
