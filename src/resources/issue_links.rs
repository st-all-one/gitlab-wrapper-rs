use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use std::sync::Arc;

/// Recurso de API para operações com links entre issues no GitLab.
#[derive(Debug)]
pub struct IssueLinksResource {
    http: Arc<HttpClient>,
}

impl IssueLinksResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista os links de uma issue específica.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `issue_iid`: IID da issue no projeto.
    ///
    /// ## Returns
    /// `Result<Vec<IssueLink>, GitLabError>` — lista de links da issue.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list(
        &self,
        project_id: u64,
        issue_iid: u32,
    ) -> Result<Vec<IssueLink>, GitLabError> {
        let path = format!("projects/{}/issues/{}/links", project_id, issue_iid);
        self.http.get(&path, &[], "issue_links.list").await
    }

    /// Cria um novo link entre issues.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `issue_iid`: IID da issue de origem no projeto.
    /// - `payload`: Dados para criar o link.
    ///
    /// ## Returns
    /// `Result<IssueLink, GitLabError>` — dados do link criado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn create(
        &self,
        project_id: u64,
        issue_iid: u32,
        payload: &CreateIssueLinkPayload,
    ) -> Result<IssueLink, GitLabError> {
        let path = format!("projects/{}/issues/{}/links", project_id, issue_iid);
        self.http.post(&path, &payload, "issue_links.create").await
    }

    /// Remove um link entre issues.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `issue_iid`: IID da issue no projeto.
    /// - `link_id`: ID do link a ser removido.
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
        issue_iid: u32,
        link_id: u64,
    ) -> Result<(), GitLabError> {
        let path = format!("projects/{}/issues/{}/links/{}", project_id, issue_iid, link_id);
        self.http.delete(&path, &[], "issue_links.delete").await
    }
}
