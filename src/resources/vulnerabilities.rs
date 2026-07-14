use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use crate::utils::encoding::filter_to_query;
use std::sync::Arc;

/// Recurso de API para vulnerabilidades no GitLab (Ultimate).
#[derive(Debug)]
pub struct VulnerabilitiesResource {
    http: Arc<HttpClient>,
}

impl VulnerabilitiesResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Executa a operação .
    /// Executa a operacao `get`.
    pub async fn get(&self, vuln_id: u64) -> Result<Vulnerability, GitLabError> {
        let path = format!("vulnerabilities/{}", vuln_id);
        self.http.get(&path, &[], "vulnerabilities.get").await
    }

    /// Executa a operação .
    /// Executa a operacao `list_project`.
    pub async fn list_project(
        &self,
        project_id: u64,
        filter: Option<&VulnerabilityFilter>,
    ) -> Result<Vec<Vulnerability>, GitLabError> {
        let path = format!("projects/{}/vulnerabilities", project_id);
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "vulnerabilities.list_project").await
    }

    /// Executa a operação .
    /// Executa a operacao `list_findings`.
    pub async fn list_findings(
        &self,
        project_id: u64,
        filter: Option<&VulnerabilityFindingFilter>,
    ) -> Result<Vec<VulnerabilityFinding>, GitLabError> {
        let path = format!("projects/{}/vulnerability_findings", project_id);
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "vulnerabilities.list_findings").await
    }

    /// Executa a operação .
    /// Executa a operacao `create_export`.
    pub async fn create_export(&self, project_id: u64) -> Result<VulnerabilityExport, GitLabError> {
        let path = format!("projects/{}/vulnerability_exports", project_id);
        self.http.post(&path, &serde_json::json!({}), "vulnerabilities.create_export").await
    }

    /// Executa a operação .
    /// Executa a operacao `export_status`.
    pub async fn export_status(
        &self,
        project_id: u64,
        export_id: u64,
    ) -> Result<VulnerabilityExport, GitLabError> {
        let path = format!("projects/{}/vulnerability_exports/{}", project_id, export_id);
        self.http.get(&path, &[], "vulnerabilities.export_status").await
    }
}
