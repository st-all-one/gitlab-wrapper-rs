use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::{
    Vulnerability, VulnerabilityExport, VulnerabilityFilter, VulnerabilityFinding,
    VulnerabilityFindingFilter,
};
use crate::utils::encoding::filter_to_query;
use std::sync::Arc;

/// Recurso de API para vulnerabilidades no GitLab (Ultimate).
#[derive(Debug)]
pub struct VulnerabilitiesResource {
    http: Arc<HttpClient>,
}

impl VulnerabilitiesResource {
    pub(crate) const fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Executa a operação .
    /// Executa a operacao `get`.
    ///
    /// ## Errors
    /// Retorna `GitLabError::Api` se a vulnerabilidade não existir ou `GitLabError::Http`
    /// em erro de transporte da requisição.
    pub async fn get(&self, vuln_id: u64) -> Result<Vulnerability, GitLabError> {
        let path = format!("vulnerabilities/{vuln_id}");
        self.http.get(&path, &[], "vulnerabilities.get").await
    }

    /// Executa a operação .
    /// Executa a operacao `list_project`.
    ///
    /// ## Errors
    /// Retorna `GitLabError::Api` se o filtro for inválido ou o projeto não existir,
    /// ou `GitLabError::Http` em erro de transporte.
    pub async fn list_project(
        &self,
        project_id: u64,
        filter: Option<&VulnerabilityFilter>,
    ) -> Result<Vec<Vulnerability>, GitLabError> {
        let path = format!("projects/{project_id}/vulnerabilities");
        let query = filter_to_query(filter);
        self.http
            .get(&path, &query, "vulnerabilities.list_project")
            .await
    }

    /// Executa a operação .
    /// Executa a operacao `list_findings`.
    ///
    /// ## Errors
    /// Retorna `GitLabError::Api` se o filtro for inválido ou o projeto não existir,
    /// ou `GitLabError::Http` em erro de transporte.
    pub async fn list_findings(
        &self,
        project_id: u64,
        filter: Option<&VulnerabilityFindingFilter>,
    ) -> Result<Vec<VulnerabilityFinding>, GitLabError> {
        let path = format!("projects/{project_id}/vulnerability_findings");
        let query = filter_to_query(filter);
        self.http
            .get(&path, &query, "vulnerabilities.list_findings")
            .await
    }

    /// Executa a operação .
    /// Executa a operacao `create_export`.
    ///
    /// ## Errors
    /// Retorna `GitLabError::Api` se a exportação for rejeitada ou `GitLabError::Http`
    /// em erro de transporte da criação.
    pub async fn create_export(&self, project_id: u64) -> Result<VulnerabilityExport, GitLabError> {
        let path = format!("projects/{project_id}/vulnerability_exports");
        self.http
            .post(
                &path,
                &serde_json::json!({}),
                "vulnerabilities.create_export",
            )
            .await
    }

    /// Executa a operação .
    /// Executa a operacao `export_status`.
    ///
    /// ## Errors
    /// Retorna `GitLabError::Api` se a exportação não existir ou `GitLabError::Http`
    /// em erro de transporte da consulta.
    pub async fn export_status(
        &self,
        project_id: u64,
        export_id: u64,
    ) -> Result<VulnerabilityExport, GitLabError> {
        let path = format!("projects/{project_id}/vulnerability_exports/{export_id}");
        self.http
            .get(&path, &[], "vulnerabilities.export_status")
            .await
    }
}
