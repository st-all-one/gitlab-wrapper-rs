use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use std::sync::Arc;

/// Recurso de API para operações com eventos de auditoria no GitLab.
#[derive(Debug)]
pub struct AuditEventsResource {
    http: Arc<HttpClient>,
}

impl AuditEventsResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista todos os eventos de auditoria.
    ///
    /// ## Returns
    /// `Result<Vec<AuditEvent>, GitLabError>` — lista de eventos de auditoria.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list(&self) -> Result<Vec<AuditEvent>, GitLabError> {
        let path = "audit_events".to_string();
        self.http.get(&path, &[], "audit_events.list").await
    }

    /// Obtém um evento de auditoria pelo ID.
    ///
    /// ## Params
    /// - `event_id`: ID do evento de auditoria.
    ///
    /// ## Returns
    /// `Result<AuditEvent, GitLabError>` — dados do evento.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get(&self, event_id: u64) -> Result<AuditEvent, GitLabError> {
        let path = format!("audit_events/{}", event_id);
        self.http.get(&path, &[], "audit_events.get").await
    }
}
