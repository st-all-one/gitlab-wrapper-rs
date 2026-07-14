use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use std::sync::Arc;

/// Recurso de API para operações com espelhos remotos (remote mirrors) no GitLab.
#[derive(Debug)]
pub struct RemoteMirrorsResource {
    http: Arc<HttpClient>,
}

impl RemoteMirrorsResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista todos os espelhos remotos de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<RemoteMirror>, GitLabError>` — lista de espelhos remotos.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list(&self, project_id: u64) -> Result<Vec<RemoteMirror>, GitLabError> {
        let path = format!("projects/{}/remote_mirrors", project_id);
        self.http.get(&path, &[], "remote_mirrors.list").await
    }

    /// Cria um novo espelho remoto para um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `payload`: Dados do espelho remoto a criar.
    ///
    /// ## Returns
    /// `Result<RemoteMirror, GitLabError>` — dados do espelho criado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn create(
        &self,
        project_id: u64,
        payload: &CreateRemoteMirrorPayload,
    ) -> Result<RemoteMirror, GitLabError> {
        let path = format!("projects/{}/remote_mirrors", project_id);
        self.http.post(&path, payload, "remote_mirrors.create").await
    }

    /// Atualiza um espelho remoto existente.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `mirror_id`: ID do espelho remoto.
    /// - `payload`: Dados do espelho a atualizar.
    ///
    /// ## Returns
    /// `Result<RemoteMirror, GitLabError>` — dados do espelho atualizado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn update(
        &self,
        project_id: u64,
        mirror_id: u64,
        payload: &UpdateRemoteMirrorPayload,
    ) -> Result<RemoteMirror, GitLabError> {
        let path = format!("projects/{}/remote_mirrors/{}", project_id, mirror_id);
        self.http.put(&path, payload, "remote_mirrors.update").await
    }
}
