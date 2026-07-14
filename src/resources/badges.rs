use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use std::sync::Arc;

/// Recurso de API para operações com badges (emblemas) de projetos e grupos no GitLab.
#[derive(Debug)]
pub struct BadgesResource {
    http: Arc<HttpClient>,
}

impl BadgesResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista todos os badges de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<Badge>, GitLabError>` — lista de badges do projeto.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list_project_badges(&self, project_id: u64) -> Result<Vec<Badge>, GitLabError> {
        let path = format!("projects/{}/badges", project_id);
        self.http.get(&path, &[], "badges.list_project").await
    }

    /// Obtém um badge específico de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `badge_id`: ID do badge.
    ///
    /// ## Returns
    /// `Result<Badge, GitLabError>` — dados do badge solicitado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get_project_badge(
        &self,
        project_id: u64,
        badge_id: u64,
    ) -> Result<Badge, GitLabError> {
        let path = format!("projects/{}/badges/{}", project_id, badge_id);
        self.http.get(&path, &[], "badges.get_project").await
    }

    /// Cria um novo badge em um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `payload`: Dados para criar o badge.
    ///
    /// ## Returns
    /// `Result<Badge, GitLabError>` — dados do badge criado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn create_project_badge(
        &self,
        project_id: u64,
        payload: &CreateBadgePayload,
    ) -> Result<Badge, GitLabError> {
        let path = format!("projects/{}/badges", project_id);
        self.http.post(&path, &payload, "badges.create_project").await
    }

    /// Atualiza um badge de projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `badge_id`: ID do badge a ser atualizado.
    /// - `payload`: Dados para atualizar o badge.
    ///
    /// ## Returns
    /// `Result<Badge, GitLabError>` — dados do badge atualizado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn update_project_badge(
        &self,
        project_id: u64,
        badge_id: u64,
        payload: &UpdateBadgePayload,
    ) -> Result<Badge, GitLabError> {
        let path = format!("projects/{}/badges/{}", project_id, badge_id);
        self.http.put(&path, &payload, "badges.update_project").await
    }

    /// Remove um badge de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `badge_id`: ID do badge a ser removido.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn delete_project_badge(
        &self,
        project_id: u64,
        badge_id: u64,
    ) -> Result<(), GitLabError> {
        let path = format!("projects/{}/badges/{}", project_id, badge_id);
        self.http.delete(&path, &[], "badges.delete_project").await
    }

    /// Lista todos os badges de um grupo.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<Badge>, GitLabError>` — lista de badges do grupo.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list_group_badges(&self, group_id: u64) -> Result<Vec<Badge>, GitLabError> {
        let path = format!("groups/{}/badges", group_id);
        self.http.get(&path, &[], "badges.list_group").await
    }

    /// Obtém um badge específico de um grupo.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    /// - `badge_id`: ID do badge.
    ///
    /// ## Returns
    /// `Result<Badge, GitLabError>` — dados do badge solicitado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get_group_badge(
        &self,
        group_id: u64,
        badge_id: u64,
    ) -> Result<Badge, GitLabError> {
        let path = format!("groups/{}/badges/{}", group_id, badge_id);
        self.http.get(&path, &[], "badges.get_group").await
    }

    /// Cria um novo badge em um grupo.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    /// - `payload`: Dados para criar o badge.
    ///
    /// ## Returns
    /// `Result<Badge, GitLabError>` — dados do badge criado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn create_group_badge(
        &self,
        group_id: u64,
        payload: &CreateBadgePayload,
    ) -> Result<Badge, GitLabError> {
        let path = format!("groups/{}/badges", group_id);
        self.http.post(&path, &payload, "badges.create_group").await
    }

    /// Atualiza um badge de grupo.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    /// - `badge_id`: ID do badge a ser atualizado.
    /// - `payload`: Dados para atualizar o badge.
    ///
    /// ## Returns
    /// `Result<Badge, GitLabError>` — dados do badge atualizado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn update_group_badge(
        &self,
        group_id: u64,
        badge_id: u64,
        payload: &UpdateBadgePayload,
    ) -> Result<Badge, GitLabError> {
        let path = format!("groups/{}/badges/{}", group_id, badge_id);
        self.http.put(&path, &payload, "badges.update_group").await
    }

    /// Remove um badge de um grupo.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    /// - `badge_id`: ID do badge a ser removido.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn delete_group_badge(
        &self,
        group_id: u64,
        badge_id: u64,
    ) -> Result<(), GitLabError> {
        let path = format!("groups/{}/badges/{}", group_id, badge_id);
        self.http.delete(&path, &[], "badges.delete_group").await
    }
}
