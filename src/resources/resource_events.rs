use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use std::sync::Arc;

/// Recurso de API para operações com eventos de recursos
/// (estado, label, milestone, peso e iteração) no GitLab.
#[derive(Debug)]
pub struct ResourceEventsResource {
    http: Arc<HttpClient>,
}

impl ResourceEventsResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    fn issue_state_events_base(project_id: u64, issue_iid: u32) -> String {
        format!("projects/{}/issues/{}/resource_state_events", project_id, issue_iid)
    }

    fn mr_state_events_base(project_id: u64, mr_iid: u32) -> String {
        format!("projects/{}/merge_requests/{}/resource_state_events", project_id, mr_iid)
    }

    fn issue_label_events_base(project_id: u64, issue_iid: u32) -> String {
        format!("projects/{}/issues/{}/resource_label_events", project_id, issue_iid)
    }

    fn mr_label_events_base(project_id: u64, mr_iid: u32) -> String {
        format!("projects/{}/merge_requests/{}/resource_label_events", project_id, mr_iid)
    }

    fn issue_milestone_events_base(project_id: u64, issue_iid: u32) -> String {
        format!("projects/{}/issues/{}/resource_milestone_events", project_id, issue_iid)
    }

    fn mr_milestone_events_base(project_id: u64, mr_iid: u32) -> String {
        format!("projects/{}/merge_requests/{}/resource_milestone_events", project_id, mr_iid)
    }

    fn issue_weight_events_base(project_id: u64, issue_iid: u32) -> String {
        format!("projects/{}/issues/{}/resource_weight_events", project_id, issue_iid)
    }

    fn issue_iteration_events_base(project_id: u64, issue_iid: u32) -> String {
        format!("projects/{}/issues/{}/resource_iteration_events", project_id, issue_iid)
    }

    // -- State events --

    /// Lista eventos de estado de uma issue.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `issue_iid`: IID da issue no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<ResourceStateEvent>, GitLabError>` — lista de eventos de estado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list_issue_state_events(
        &self,
        project_id: u64,
        issue_iid: u32,
    ) -> Result<Vec<ResourceStateEvent>, GitLabError> {
        let path = Self::issue_state_events_base(project_id, issue_iid);
        self.http.get(&path, &[], "resource_events.list_issue_state").await
    }

    /// Lista eventos de estado de um merge request.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `mr_iid`: IID do merge request no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<ResourceStateEvent>, GitLabError>` — lista de eventos de estado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list_mr_state_events(
        &self,
        project_id: u64,
        mr_iid: u32,
    ) -> Result<Vec<ResourceStateEvent>, GitLabError> {
        let path = Self::mr_state_events_base(project_id, mr_iid);
        self.http.get(&path, &[], "resource_events.list_mr_state").await
    }

    // -- Label events --

    /// Lista eventos de label de uma issue.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `issue_iid`: IID da issue no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<ResourceLabelEvent>, GitLabError>` — lista de eventos de label.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list_issue_label_events(
        &self,
        project_id: u64,
        issue_iid: u32,
    ) -> Result<Vec<ResourceLabelEvent>, GitLabError> {
        let path = Self::issue_label_events_base(project_id, issue_iid);
        self.http.get(&path, &[], "resource_events.list_issue_label").await
    }

    /// Obtém um evento de label específico de uma issue.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `issue_iid`: IID da issue no GitLab.
    /// - `event_id`: ID do evento de label no GitLab.
    ///
    /// ## Returns
    /// `Result<ResourceLabelEvent, GitLabError>` — dados do evento de label.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get_issue_label_event(
        &self,
        project_id: u64,
        issue_iid: u32,
        event_id: u64,
    ) -> Result<ResourceLabelEvent, GitLabError> {
        let path = format!("{}/{}", Self::issue_label_events_base(project_id, issue_iid), event_id);
        self.http.get(&path, &[], "resource_events.get_issue_label").await
    }

    /// Lista eventos de label de um merge request.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `mr_iid`: IID do merge request no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<ResourceLabelEvent>, GitLabError>` — lista de eventos de label.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list_mr_label_events(
        &self,
        project_id: u64,
        mr_iid: u32,
    ) -> Result<Vec<ResourceLabelEvent>, GitLabError> {
        let path = Self::mr_label_events_base(project_id, mr_iid);
        self.http.get(&path, &[], "resource_events.list_mr_label").await
    }

    // -- Milestone events --

    /// Lista eventos de milestone de uma issue.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `issue_iid`: IID da issue no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<ResourceMilestoneEvent>, GitLabError>` — lista de eventos de milestone.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list_issue_milestone_events(
        &self,
        project_id: u64,
        issue_iid: u32,
    ) -> Result<Vec<ResourceMilestoneEvent>, GitLabError> {
        let path = Self::issue_milestone_events_base(project_id, issue_iid);
        self.http.get(&path, &[], "resource_events.list_issue_milestone").await
    }

    /// Lista eventos de milestone de um merge request.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `mr_iid`: IID do merge request no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<ResourceMilestoneEvent>, GitLabError>` — lista de eventos de milestone.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list_mr_milestone_events(
        &self,
        project_id: u64,
        mr_iid: u32,
    ) -> Result<Vec<ResourceMilestoneEvent>, GitLabError> {
        let path = Self::mr_milestone_events_base(project_id, mr_iid);
        self.http.get(&path, &[], "resource_events.list_mr_milestone").await
    }

    // -- Weight events --

    /// Lista eventos de peso de uma issue.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `issue_iid`: IID da issue no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<ResourceWeightEvent>, GitLabError>` — lista de eventos de peso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list_issue_weight_events(
        &self,
        project_id: u64,
        issue_iid: u32,
    ) -> Result<Vec<ResourceWeightEvent>, GitLabError> {
        let path = Self::issue_weight_events_base(project_id, issue_iid);
        self.http.get(&path, &[], "resource_events.list_issue_weight").await
    }

    // -- Iteration events --

    /// Lista eventos de iteração de uma issue.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `issue_iid`: IID da issue no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<ResourceIterationEvent>, GitLabError>` — lista de eventos de iteração.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list_issue_iteration_events(
        &self,
        project_id: u64,
        issue_iid: u32,
    ) -> Result<Vec<ResourceIterationEvent>, GitLabError> {
        let path = Self::issue_iteration_events_base(project_id, issue_iid);
        self.http.get(&path, &[], "resource_events.list_issue_iteration").await
    }
}
