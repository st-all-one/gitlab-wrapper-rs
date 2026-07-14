use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use std::sync::Arc;

/// Recurso de API para validação de CI/CD (CI Lint).
#[derive(Debug)]
pub struct CiLintResource {
    http: Arc<HttpClient>,
}

impl CiLintResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Valida um arquivo `.gitlab-ci.yml`.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `payload`: Conteúdo do YAML a validar.
    ///
    /// ## Returns
    /// `Result<CiLintResult, GitLabError>` — resultado da validação.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    /// Executa a operação .
    /// Executa a operacao `validate`.
    pub async fn validate(
        &self,
        project_id: u64,
        payload: &CiLintPayload,
    ) -> Result<CiLintResult, GitLabError> {
        let path = format!("projects/{}/ci/lint", project_id);
        self.http.post(&path, payload, "ci_lint.validate").await
    }
}
