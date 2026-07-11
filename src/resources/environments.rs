use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use std::sync::Arc;
use crate::types::*;

/// Recurso de API para operações com ambientes no GitLab.
#[derive(Debug)]
pub struct EnvironmentsResource {
    http: Arc<HttpClient>,
}

impl EnvironmentsResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista todos os ambientes de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<Environment>, GitLabError>` — lista de ambientes.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn list(&self, project_id: u64) -> Result<Vec<Environment>, GitLabError> {
        let path = format!("projects/{}/environments", project_id);
        self.http.get(&path, &[], "environments.list")
    }

    /// Obtém um ambiente pelo ID.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `env_id`: ID do ambiente no GitLab.
    ///
    /// ## Returns
    /// `Result<Environment, GitLabError>` — dados do ambiente solicitado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn get(&self, project_id: u64, env_id: u64) -> Result<Environment, GitLabError> {
        let path = format!("projects/{}/environments/{}", project_id, env_id);
        self.http.get(&path, &[], "environments.get")
    }

    /// Cria um novo ambiente em um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `payload`: Dados para criar o ambiente.
    ///
    /// ## Returns
    /// `Result<Environment, GitLabError>` — dados do ambiente criado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn create(&self, project_id: u64, payload: &CreateEnvironmentPayload) -> Result<Environment, GitLabError> {
        let path = format!("projects/{}/environments", project_id);
        self.http.post(&path, &payload, "environments.create")
    }

    /// Atualiza um ambiente existente.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `env_id`: ID do ambiente no GitLab.
    /// - `payload`: Dados para atualizar o ambiente.
    ///
    /// ## Returns
    /// `Result<Environment, GitLabError>` — dados do ambiente atualizado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn update(&self, project_id: u64, env_id: u64, payload: &UpdateEnvironmentPayload) -> Result<Environment, GitLabError> {
        let path = format!("projects/{}/environments/{}", project_id, env_id);
        self.http.put(&path, &payload, "environments.update")
    }

    /// Remove um ambiente.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `env_id`: ID do ambiente no GitLab.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn delete(&self, project_id: u64, env_id: u64) -> Result<(), GitLabError> {
        let path = format!("projects/{}/environments/{}", project_id, env_id);
        self.http.delete(&path, &[], "environments.delete")
    }

    /// Para um ambiente ativo.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `env_id`: ID do ambiente no GitLab.
    ///
    /// ## Returns
    /// `Result<Environment, GitLabError>` — dados do ambiente parado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn stop(&self, project_id: u64, env_id: u64) -> Result<Environment, GitLabError> {
        let path = format!("projects/{}/environments/{}/stop", project_id, env_id);
        self.http.post(&path, &serde_json::Value::Null, "environments.stop")
    }
}
