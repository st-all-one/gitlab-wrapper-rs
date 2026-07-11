use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use std::sync::Arc;
use crate::types::*;

/// Recurso de API para operações com membros no GitLab.
#[derive(Debug)]
pub struct MembersResource {
    http: Arc<HttpClient>,
}

impl MembersResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista membros de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<Member>, GitLabError>` — lista de membros do projeto.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn list_project_members(&self, project_id: u64) -> Result<Vec<Member>, GitLabError> {
        let path = format!("projects/{}/members", project_id);
        self.http.get(&path, &[], "members.list_project")
    }

    /// Obtém um membro específico de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `user_id`: ID do usuário no GitLab.
    ///
    /// ## Returns
    /// `Result<Member, GitLabError>` — dados do membro solicitado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn get_project_member(&self, project_id: u64, user_id: u64) -> Result<Member, GitLabError> {
        let path = format!("projects/{}/members/{}", project_id, user_id);
        self.http.get(&path, &[], "members.get_project")
    }

    /// Adiciona um membro a um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `payload`: Dados para adicionar o membro.
    ///
    /// ## Returns
    /// `Result<Member, GitLabError>` — dados do membro adicionado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn add_project_member(&self, project_id: u64, payload: &AddMemberPayload) -> Result<Member, GitLabError> {
        let path = format!("projects/{}/members", project_id);
        self.http.post(&path, &payload, "members.add_project")
    }

    /// Atualiza um membro de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `user_id`: ID do usuário no GitLab.
    /// - `payload`: Dados para atualizar o membro.
    ///
    /// ## Returns
    /// `Result<Member, GitLabError>` — dados do membro atualizado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn update_project_member(&self, project_id: u64, user_id: u64, payload: &UpdateMemberPayload) -> Result<Member, GitLabError> {
        let path = format!("projects/{}/members/{}", project_id, user_id);
        self.http.put(&path, &payload, "members.update_project")
    }

    /// Remove um membro de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `user_id`: ID do usuário no GitLab.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn delete_project_member(&self, project_id: u64, user_id: u64) -> Result<(), GitLabError> {
        let path = format!("projects/{}/members/{}", project_id, user_id);
        self.http.delete(&path, &[], "members.delete_project")
    }

    /// Lista membros de um projeto, incluindo membros herdados.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<Member>, GitLabError>` — lista de membros do projeto (incluindo herdados).
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn list_project_inherited_members(&self, project_id: u64) -> Result<Vec<Member>, GitLabError> {
        let path = format!("projects/{}/members/all", project_id);
        self.http.get(&path, &[], "members.list_project_inherited")
    }

    /// Lista membros de um grupo.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<Member>, GitLabError>` — lista de membros do grupo.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn list_group_members(&self, group_id: u64) -> Result<Vec<Member>, GitLabError> {
        let path = format!("groups/{}/members", group_id);
        self.http.get(&path, &[], "members.list_group")
    }

    /// Obtém um membro específico de um grupo.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    /// - `user_id`: ID do usuário no GitLab.
    ///
    /// ## Returns
    /// `Result<Member, GitLabError>` — dados do membro solicitado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn get_group_member(&self, group_id: u64, user_id: u64) -> Result<Member, GitLabError> {
        let path = format!("groups/{}/members/{}", group_id, user_id);
        self.http.get(&path, &[], "members.get_group")
    }

    /// Adiciona um membro a um grupo.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    /// - `payload`: Dados para adicionar o membro.
    ///
    /// ## Returns
    /// `Result<Member, GitLabError>` — dados do membro adicionado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn add_group_member(&self, group_id: u64, payload: &AddMemberPayload) -> Result<Member, GitLabError> {
        let path = format!("groups/{}/members", group_id);
        self.http.post(&path, &payload, "members.add_group")
    }

    /// Atualiza um membro de um grupo.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    /// - `user_id`: ID do usuário no GitLab.
    /// - `payload`: Dados para atualizar o membro.
    ///
    /// ## Returns
    /// `Result<Member, GitLabError>` — dados do membro atualizado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn update_group_member(&self, group_id: u64, user_id: u64, payload: &UpdateMemberPayload) -> Result<Member, GitLabError> {
        let path = format!("groups/{}/members/{}", group_id, user_id);
        self.http.put(&path, &payload, "members.update_group")
    }

    /// Remove um membro de um grupo.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    /// - `user_id`: ID do usuário no GitLab.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn delete_group_member(&self, group_id: u64, user_id: u64) -> Result<(), GitLabError> {
        let path = format!("groups/{}/members/{}", group_id, user_id);
        self.http.delete(&path, &[], "members.delete_group")
    }

    /// Lista membros de um grupo, incluindo membros herdados.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<Member>, GitLabError>` — lista de membros do grupo (incluindo herdados).
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn list_group_inherited_members(&self, group_id: u64) -> Result<Vec<Member>, GitLabError> {
        let path = format!("groups/{}/members/all", group_id);
        self.http.get(&path, &[], "members.list_group_inherited")
    }
}
