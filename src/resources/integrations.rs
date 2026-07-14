use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use std::sync::Arc;

/// Recurso de API para operações com integrações de projeto no GitLab.
#[derive(Debug)]
pub struct IntegrationsResource {
    http: Arc<HttpClient>,
}

impl IntegrationsResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista todas as integrações de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<serde_json::Value>, GitLabError>` — lista de integrações.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list(&self, project_id: u64) -> Result<Vec<serde_json::Value>, GitLabError> {
        let path = format!("projects/{}/integrations", project_id);
        self.http.get(&path, &[], "integrations.list").await
    }

    /// Atualiza (ou habilita) uma integração em um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `slug`: Slug da integração (ex.: "slack", "jira").
    /// - `payload`: Parâmetros da integração.
    ///
    /// ## Returns
    /// `Result<serde_json::Value, GitLabError>` — dados da integração atualizada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn update(
        &self,
        project_id: u64,
        slug: &str,
        payload: &serde_json::Value,
    ) -> Result<serde_json::Value, GitLabError> {
        let path = format!("projects/{}/integrations/{}", project_id, slug);
        self.http.put(&path, payload, "integrations.update").await
    }

    /// Desabilita uma integração em um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `slug`: Slug da integração a desabilitar.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn disable(&self, project_id: u64, slug: &str) -> Result<(), GitLabError> {
        let path = format!("projects/{}/integrations/{}", project_id, slug);
        self.http.delete(&path, &[], "integrations.disable").await
    }
}
