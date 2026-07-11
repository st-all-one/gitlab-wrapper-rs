use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use std::sync::Arc;
use crate::types::*;
use crate::utils::encoding::filter_to_query;

/// Recurso de API para operações com milestones no GitLab.
#[derive(Debug)]
pub struct MilestonesResource {
    http: Arc<HttpClient>,
}

impl MilestonesResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista milestones de um projeto com filtros opcionais.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `filter`: Filtros opcionais para a consulta.
    ///
    /// ## Returns
    /// `Result<Vec<Milestone>, GitLabError>` — lista de milestones do projeto.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn list_project_milestones(&self, project_id: u64, filter: Option<&MilestoneFilter>) -> Result<Vec<Milestone>, GitLabError> {
        let path = format!("projects/{}/milestones", project_id);
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "milestones.list_project")
    }

    /// Obtém um milestone de projeto pelo ID.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `milestone_id`: ID do milestone.
    ///
    /// ## Returns
    /// `Result<Milestone, GitLabError>` — dados do milestone solicitado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn get_project_milestone(&self, project_id: u64, milestone_id: u64) -> Result<Milestone, GitLabError> {
        let path = format!("projects/{}/milestones/{}", project_id, milestone_id);
        self.http.get(&path, &[], "milestones.get_project")
    }

    /// Cria um novo milestone em um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `payload`: Dados para criar o milestone.
    ///
    /// ## Returns
    /// `Result<Milestone, GitLabError>` — dados do milestone criado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn create_project_milestone(&self, project_id: u64, payload: &CreateMilestonePayload) -> Result<Milestone, GitLabError> {
        let path = format!("projects/{}/milestones", project_id);
        self.http.post(&path, &payload, "milestones.create_project")
    }

    /// Atualiza um milestone de projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `milestone_id`: ID do milestone.
    /// - `payload`: Dados para atualizar o milestone.
    ///
    /// ## Returns
    /// `Result<Milestone, GitLabError>` — dados do milestone atualizado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn update_project_milestone(&self, project_id: u64, milestone_id: u64, payload: &UpdateMilestonePayload) -> Result<Milestone, GitLabError> {
        let path = format!("projects/{}/milestones/{}", project_id, milestone_id);
        self.http.put(&path, &payload, "milestones.update_project")
    }

    /// Remove um milestone de projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `milestone_id`: ID do milestone.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn delete_project_milestone(&self, project_id: u64, milestone_id: u64) -> Result<(), GitLabError> {
        let path = format!("projects/{}/milestones/{}", project_id, milestone_id);
        self.http.delete(&path, &[], "milestones.delete_project")
    }

    /// Lista issues associadas a um milestone de projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `milestone_id`: ID do milestone.
    ///
    /// ## Returns
    /// `Result<Vec<Issue>, GitLabError>` — lista de issues do milestone.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn list_project_milestone_issues(&self, project_id: u64, milestone_id: u64) -> Result<Vec<Issue>, GitLabError> {
        let path = format!("projects/{}/milestones/{}/issues", project_id, milestone_id);
        self.http.get(&path, &[], "milestones.list_project_issues")
    }

    /// Lista merge requests associados a um milestone de projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `milestone_id`: ID do milestone.
    ///
    /// ## Returns
    /// `Result<Vec<MergeRequest>, GitLabError>` — lista de merge requests do milestone.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn list_project_milestone_merge_requests(&self, project_id: u64, milestone_id: u64) -> Result<Vec<MergeRequest>, GitLabError> {
        let path = format!("projects/{}/milestones/{}/merge_requests", project_id, milestone_id);
        self.http.get(&path, &[], "milestones.list_project_merge_requests")
    }

    /// Lista milestones de um grupo com filtros opcionais.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    /// - `filter`: Filtros opcionais para a consulta.
    ///
    /// ## Returns
    /// `Result<Vec<Milestone>, GitLabError>` — lista de milestones do grupo.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn list_group_milestones(&self, group_id: u64, filter: Option<&MilestoneFilter>) -> Result<Vec<Milestone>, GitLabError> {
        let path = format!("groups/{}/milestones", group_id);
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "milestones.list_group")
    }

    /// Obtém um milestone de grupo pelo ID.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    /// - `milestone_id`: ID do milestone.
    ///
    /// ## Returns
    /// `Result<Milestone, GitLabError>` — dados do milestone solicitado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn get_group_milestone(&self, group_id: u64, milestone_id: u64) -> Result<Milestone, GitLabError> {
        let path = format!("groups/{}/milestones/{}", group_id, milestone_id);
        self.http.get(&path, &[], "milestones.get_group")
    }

    /// Cria um novo milestone em um grupo.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    /// - `payload`: Dados para criar o milestone.
    ///
    /// ## Returns
    /// `Result<Milestone, GitLabError>` — dados do milestone criado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn create_group_milestone(&self, group_id: u64, payload: &CreateMilestonePayload) -> Result<Milestone, GitLabError> {
        let path = format!("groups/{}/milestones", group_id);
        self.http.post(&path, &payload, "milestones.create_group")
    }

    /// Atualiza um milestone de grupo.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    /// - `milestone_id`: ID do milestone.
    /// - `payload`: Dados para atualizar o milestone.
    ///
    /// ## Returns
    /// `Result<Milestone, GitLabError>` — dados do milestone atualizado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn update_group_milestone(&self, group_id: u64, milestone_id: u64, payload: &UpdateMilestonePayload) -> Result<Milestone, GitLabError> {
        let path = format!("groups/{}/milestones/{}", group_id, milestone_id);
        self.http.put(&path, &payload, "milestones.update_group")
    }

    /// Remove um milestone de grupo.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    /// - `milestone_id`: ID do milestone.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn delete_group_milestone(&self, group_id: u64, milestone_id: u64) -> Result<(), GitLabError> {
        let path = format!("groups/{}/milestones/{}", group_id, milestone_id);
        self.http.delete(&path, &[], "milestones.delete_group")
    }

    /// Lista issues associadas a um milestone de grupo.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    /// - `milestone_id`: ID do milestone.
    ///
    /// ## Returns
    /// `Result<Vec<Issue>, GitLabError>` — lista de issues do milestone.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn list_group_milestone_issues(&self, group_id: u64, milestone_id: u64) -> Result<Vec<Issue>, GitLabError> {
        let path = format!("groups/{}/milestones/{}/issues", group_id, milestone_id);
        self.http.get(&path, &[], "milestones.list_group_issues")
    }

    /// Lista merge requests associados a um milestone de grupo.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    /// - `milestone_id`: ID do milestone.
    ///
    /// ## Returns
    /// `Result<Vec<MergeRequest>, GitLabError>` — lista de merge requests do milestone.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn list_group_milestone_merge_requests(&self, group_id: u64, milestone_id: u64) -> Result<Vec<MergeRequest>, GitLabError> {
        let path = format!("groups/{}/milestones/{}/merge_requests", group_id, milestone_id);
        self.http.get(&path, &[], "milestones.list_group_merge_requests")
    }
}
