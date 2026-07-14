use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use crate::utils::encoding::filter_to_query;
use std::sync::Arc;

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
    pub async fn list(&self, filter: Option<&IssueFilter>) -> Result<Vec<Issue>, GitLabError> {
        let query = filter_to_query(filter);
        self.http.get("issues", &query, "issues.list").await
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
    pub async fn list_for_project(
        &self,
        project_id: u64,
        filter: Option<&IssueFilter>,
    ) -> Result<Vec<Issue>, GitLabError> {
        let path = format!("projects/{}/issues", project_id);
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "issues.list_for_project").await
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
    pub async fn get(&self, project_id: u64, issue_iid: u32) -> Result<Issue, GitLabError> {
        let path = format!("projects/{}/issues/{}", project_id, issue_iid);
        self.http.get(&path, &[], "issues.get").await
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
    pub async fn create(
        &self,
        project_id: u64,
        payload: &CreateIssuePayload,
    ) -> Result<Issue, GitLabError> {
        let path = format!("projects/{}/issues", project_id);
        self.http.post(&path, &payload, "issues.create").await
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
    pub async fn update(
        &self,
        project_id: u64,
        issue_iid: u32,
        payload: &UpdateIssuePayload,
    ) -> Result<Issue, GitLabError> {
        let path = format!("projects/{}/issues/{}", project_id, issue_iid);
        self.http.put(&path, &payload, "issues.update").await
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
    pub async fn delete(&self, project_id: u64, issue_iid: u32) -> Result<(), GitLabError> {
        let path = format!("projects/{}/issues/{}", project_id, issue_iid);
        self.http.delete(&path, &[], "issues.delete").await
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
    pub async fn subscribe(
        &self,
        project_id: u64,
        issue_iid: u32,
    ) -> Result<IssueMinimal, GitLabError> {
        let path = format!("projects/{}/issues/{}/subscribe", project_id, issue_iid);
        self.http.post(&path, &serde_json::Value::Null, "issues.subscribe").await
    }

    /// Cancela a inscrição em uma issue.
    pub async fn unsubscribe(
        &self,
        project_id: u64,
        issue_iid: u32,
    ) -> Result<IssueMinimal, GitLabError> {
        let path = format!("projects/{}/issues/{}/unsubscribe", project_id, issue_iid);
        self.http.post(&path, &serde_json::Value::Null, "issues.unsubscribe").await
    }

    /// Define uma estimativa de tempo para uma issue.
    pub async fn set_time_estimate(
        &self,
        project_id: u64,
        issue_iid: u32,
        duration: &str,
    ) -> Result<IssueMinimal, GitLabError> {
        let path = format!("projects/{}/issues/{}/time_estimate", project_id, issue_iid);
        let body = serde_json::json!({ "duration": duration });
        self.http.post(&path, &body, "issues.set_time_estimate").await
    }

    /// Adiciona tempo gasto a uma issue.
    pub async fn add_spent_time(
        &self,
        project_id: u64,
        issue_iid: u32,
        duration: &str,
    ) -> Result<IssueMinimal, GitLabError> {
        let path = format!("projects/{}/issues/{}/add_spent_time", project_id, issue_iid);
        let body = serde_json::json!({ "duration": duration });
        self.http.post(&path, &body, "issues.add_spent_time").await
    }

    /// Redefine a estimativa de tempo de uma issue.
    pub async fn reset_time_estimate(
        &self,
        project_id: u64,
        issue_iid: u32,
    ) -> Result<IssueMinimal, GitLabError> {
        let path = format!("projects/{}/issues/{}/reset_time_estimate", project_id, issue_iid);
        self.http.post(&path, &serde_json::Value::Null, "issues.reset_time_estimate").await
    }

    /// Redefine o tempo gasto de uma issue.
    pub async fn reset_spent_time(
        &self,
        project_id: u64,
        issue_iid: u32,
    ) -> Result<IssueMinimal, GitLabError> {
        let path = format!("projects/{}/issues/{}/reset_spent_time", project_id, issue_iid);
        self.http.post(&path, &serde_json::Value::Null, "issues.reset_spent_time").await
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
    pub async fn move_issue(
        &self,
        project_id: u64,
        issue_iid: u32,
        to_project_id: u64,
    ) -> Result<Issue, GitLabError> {
        let path = format!("projects/{}/issues/{}/move", project_id, issue_iid);
        let body = serde_json::json!({ "to_project_id": to_project_id });
        self.http.post(&path, &body, "issues.move").await
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
    pub async fn get_by_group(
        &self,
        group_id: u64,
        filter: Option<&IssueFilter>,
    ) -> Result<Vec<Issue>, GitLabError> {
        let path = format!("groups/{}/issues", group_id);
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "issues.get_by_group").await
    }
}
