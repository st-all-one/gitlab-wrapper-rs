use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use std::sync::Arc;

/// Recurso de API para operações com chaves SSH no GitLab.
#[derive(Debug)]
pub struct KeysResource {
    http: Arc<HttpClient>,
}

impl KeysResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Obtém uma chave SSH pela sua impressão digital (fingerprint).
    ///
    /// ## Params
    /// - `fingerprint`: Impressão digital da chave SSH.
    ///
    /// ## Returns
    /// `Result<serde_json::Value, GitLabError>` — dados da chave SSH encontrada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get_by_fingerprint(
        &self,
        fingerprint: &str,
    ) -> Result<serde_json::Value, GitLabError> {
        let query = vec![("fingerprint".to_string(), fingerprint.to_string())];
        self.http.get("keys", &query, "keys.get_by_fingerprint").await
    }
}
