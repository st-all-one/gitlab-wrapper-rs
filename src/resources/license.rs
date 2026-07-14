use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use std::sync::Arc;

/// Recurso de API para operações com a licença do GitLab.
#[derive(Debug)]
pub struct LicenseResource {
    http: Arc<HttpClient>,
}

impl LicenseResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Obtém as informações da licença atual.
    ///
    /// ## Returns
    /// `Result<LicenseInfo, GitLabError>` — dados da licença.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get(&self) -> Result<LicenseInfo, GitLabError> {
        let path = "license".to_string();
        self.http.get(&path, &[], "license.get").await
    }

    /// Cria ou atualiza uma licença.
    ///
    /// ## Params
    /// - `payload`: Dados da licença a criar.
    ///
    /// ## Returns
    /// `Result<LicenseInfo, GitLabError>` — dados da licença criada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn create(&self, payload: &CreateLicensePayload) -> Result<LicenseInfo, GitLabError> {
        let path = "license".to_string();
        self.http.post(&path, payload, "license.create").await
    }

    /// Remove a licença atual.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn delete(&self) -> Result<(), GitLabError> {
        let path = "license".to_string();
        self.http.delete(&path, &[], "license.delete").await
    }
}
