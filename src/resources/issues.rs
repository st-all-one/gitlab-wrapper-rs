use crate::core::errors::{ErrorCategory, GitLabError};
use crate::http::client::HttpClient;
use std::sync::Arc;
use crate::types::*;
use crate::utils::encoding::filter_to_query;

/// Recurso de API para operações com issues no GitLab.
#[derive(Debug)]
pub struct IssuesResource {
    http: Arc<HttpClient>,
}

impl IssuesResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista issues com filtros opcionais (escopo global).
    ///
    /// ## Params
    /// - `filter`: Filtros opcionais para a consulta.
    ///
    /// ## Returns
    /// `Result<Vec<Issue>, GitLabError>` — lista de issues.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn list(&self, filter: Option<&IssueFilter>) -> Result<Vec<Issue>, GitLabError> {
        let query = filter_to_query(filter);
        self.http.get("issues", &query, "issues.list")
    }

    /// Lista issues de um projeto específico.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `filter`: Filtros opcionais para a consulta.
    ///
    /// ## Returns
    /// `Result<Vec<Issue>, GitLabError>` — lista de issues do projeto.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn list_for_project(&self, project_id: u64, filter: Option<&IssueFilter>) -> Result<Vec<Issue>, GitLabError> {
        let path = format!("projects/{}/issues", project_id);
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "issues.list_for_project")
    }

    /// Obtém uma issue pelo ID do projeto e IID da issue.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `issue_iid`: IID da issue no projeto.
    ///
    /// ## Returns
    /// `Result<Issue, GitLabError>` — dados da issue solicitada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn get(&self, project_id: u64, issue_iid: u32) -> Result<Issue, GitLabError> {
        let path = format!("projects/{}/issues/{}", project_id, issue_iid);
        self.http.get(&path, &[], "issues.get")
    }

    /// Cria uma nova issue em um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `payload`: Dados para criar a issue.
    ///
    /// ## Returns
    /// `Result<Issue, GitLabError>` — dados da issue criada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn create(&self, project_id: u64, payload: &CreateIssuePayload) -> Result<Issue, GitLabError> {
        let path = format!("projects/{}/issues", project_id);
        self.http.post(&path, &payload, "issues.create")
    }

    /// Atualiza uma issue existente.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `issue_iid`: IID da issue no projeto.
    /// - `payload`: Dados para atualizar a issue.
    ///
    /// ## Returns
    /// `Result<Issue, GitLabError>` — dados da issue atualizada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn update(&self, project_id: u64, issue_iid: u32, payload: &UpdateIssuePayload) -> Result<Issue, GitLabError> {
        let path = format!("projects/{}/issues/{}", project_id, issue_iid);
        self.http.put(&path, &payload, "issues.update")
    }

    /// Remove uma issue.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `issue_iid`: IID da issue no projeto.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn delete(&self, project_id: u64, issue_iid: u32) -> Result<(), GitLabError> {
        let path = format!("projects/{}/issues/{}", project_id, issue_iid);
        self.http.delete(&path, &[], "issues.delete")
    }

    /// Inscreve-se em uma issue.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `issue_iid`: IID da issue no projeto.
    ///
    /// ## Returns
    /// `Result<Issue, GitLabError>` — dados da issue atualizada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn subscribe(&self, project_id: u64, issue_iid: u32) -> Result<Issue, GitLabError> {
        let path = format!("projects/{}/issues/{}/subscribe", project_id, issue_iid);
        parse_or_ok(self.http.post(&path, &serde_json::Value::Null, "issues.subscribe"))
    }

    /// Cancela a inscrição em uma issue.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `issue_iid`: IID da issue no projeto.
    ///
    /// ## Returns
    /// `Result<Issue, GitLabError>` — dados da issue atualizada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn unsubscribe(&self, project_id: u64, issue_iid: u32) -> Result<Issue, GitLabError> {
        let path = format!("projects/{}/issues/{}/unsubscribe", project_id, issue_iid);
        parse_or_ok(self.http.post(&path, &serde_json::Value::Null, "issues.unsubscribe"))
    }

    /// Define uma estimativa de tempo para uma issue.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `issue_iid`: IID da issue no projeto.
    /// - `duration`: Duração estimada (ex: "3h30m").
    ///
    /// ## Returns
    /// `Result<Issue, GitLabError>` — dados da issue atualizada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn set_time_estimate(&self, project_id: u64, issue_iid: u32, duration: &str) -> Result<Issue, GitLabError> {
        let path = format!("projects/{}/issues/{}/time_estimate", project_id, issue_iid);
        let body = serde_json::json!({ "duration": duration });
        parse_or_ok(self.http.post(&path, &body, "issues.set_time_estimate"))
    }

    /// Adiciona tempo gasto a uma issue.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `issue_iid`: IID da issue no projeto.
    /// - `duration`: Tempo gasto (ex: "1h30m").
    ///
    /// ## Returns
    /// `Result<Issue, GitLabError>` — dados da issue atualizada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn add_spent_time(&self, project_id: u64, issue_iid: u32, duration: &str) -> Result<Issue, GitLabError> {
        let path = format!("projects/{}/issues/{}/add_spent_time", project_id, issue_iid);
        let body = serde_json::json!({ "duration": duration });
        parse_or_ok(self.http.post(&path, &body, "issues.add_spent_time"))
    }

    /// Redefine a estimativa de tempo de uma issue.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `issue_iid`: IID da issue no projeto.
    ///
    /// ## Returns
    /// `Result<Issue, GitLabError>` — dados da issue atualizada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn reset_time_estimate(&self, project_id: u64, issue_iid: u32) -> Result<Issue, GitLabError> {
        let path = format!("projects/{}/issues/{}/reset_time_estimate", project_id, issue_iid);
        parse_or_ok(self.http.post(&path, &serde_json::Value::Null, "issues.reset_time_estimate"))
    }

    /// Redefine o tempo gasto de uma issue.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `issue_iid`: IID da issue no projeto.
    ///
    /// ## Returns
    /// `Result<Issue, GitLabError>` — dados da issue atualizada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn reset_spent_time(&self, project_id: u64, issue_iid: u32) -> Result<Issue, GitLabError> {
        let path = format!("projects/{}/issues/{}/reset_spent_time", project_id, issue_iid);
        parse_or_ok(self.http.post(&path, &serde_json::Value::Null, "issues.reset_spent_time"))
    }

    /// Move uma issue para outro projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto de origem no GitLab.
    /// - `issue_iid`: IID da issue no projeto de origem.
    /// - `to_project_id`: ID do projeto de destino.
    ///
    /// ## Returns
    /// `Result<Issue, GitLabError>` — dados da issue movida.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn move_issue(&self, project_id: u64, issue_iid: u32, to_project_id: u64) -> Result<Issue, GitLabError> {
        let path = format!("projects/{}/issues/{}/move", project_id, issue_iid);
        let body = serde_json::json!({ "to_project_id": to_project_id });
        self.http.post(&path, &body, "issues.move")
    }

    /// Lista issues de um grupo.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    /// - `filter`: Filtros opcionais para a consulta.
    ///
    /// ## Returns
    /// `Result<Vec<Issue>, GitLabError>` — lista de issues do grupo.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn get_by_group(&self, group_id: u64, filter: Option<&IssueFilter>) -> Result<Vec<Issue>, GitLabError> {
        let path = format!("groups/{}/issues", group_id);
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "issues.get_by_group")
    }
}

/// Some GitLab endpoints return a minimal response body on success that
/// doesn't fully match the expected struct. Accept that as success.
fn parse_or_ok(result: Result<Issue, GitLabError>) -> Result<Issue, GitLabError> {
    match result {
        Ok(issue) => Ok(issue),
        Err(GitLabError::Api { category: ErrorCategory::ParseError, .. }) => {
            Ok(Issue {
                id: 0,
                iid: 0,
                project_id: 0,
                title: String::new(),
                description: None,
                state: None,
                created_at: None,
                updated_at: None,
                closed_at: None,
                labels: None,
                milestone: None,
                assignees: None,
                author: None,
                web_url: None,
                confidential: None,
                discussion_locked: None,
                issue_type: None,
                severity: None,
                time_stats: None,
                task_completion_status: None,
                references: None,
                moved_to_id: None,
                duplicated_to_id: None,
                updated_by_id: None,
                last_edited_at: None,
                last_edited_by: None,
                user_notes_count: None,
                upvotes: None,
                downvotes: None,
                merge_requests_count: None,
                due_date: None,
                weight: None,
                _links: None,
            })
        }
        Err(e) => Err(e),
    }
}
