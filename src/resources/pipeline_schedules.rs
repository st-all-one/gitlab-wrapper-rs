use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use std::sync::Arc;
use crate::types::*;

/// Recurso de API para operações com agendamentos de pipeline no GitLab.
#[derive(Debug)]
pub struct PipelineSchedulesResource {
    http: Arc<HttpClient>,
}

impl PipelineSchedulesResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista todos os agendamentos de pipeline de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<PipelineSchedule>, GitLabError>` — lista de agendamentos.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn list(&self, project_id: u64) -> Result<Vec<PipelineSchedule>, GitLabError> {
        let path = format!("projects/{}/pipeline_schedules", project_id);
        self.http.get(&path, &[], "pipeline_schedules.list")
    }

    /// Obtém um agendamento de pipeline pelo ID.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `schedule_id`: ID do agendamento no GitLab.
    ///
    /// ## Returns
    /// `Result<PipelineSchedule, GitLabError>` — dados do agendamento solicitado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn get(&self, project_id: u64, schedule_id: u64) -> Result<PipelineSchedule, GitLabError> {
        let path = format!("projects/{}/pipeline_schedules/{}", project_id, schedule_id);
        self.http.get(&path, &[], "pipeline_schedules.get")
    }

    /// Cria um novo agendamento de pipeline.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `payload`: Dados para criar o agendamento.
    ///
    /// ## Returns
    /// `Result<PipelineSchedule, GitLabError>` — dados do agendamento criado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn create(&self, project_id: u64, payload: &CreatePipelineSchedulePayload) -> Result<PipelineSchedule, GitLabError> {
        let path = format!("projects/{}/pipeline_schedules", project_id);
        self.http.post(&path, &payload, "pipeline_schedules.create")
    }

    /// Atualiza um agendamento de pipeline existente.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `schedule_id`: ID do agendamento no GitLab.
    /// - `payload`: Dados para atualizar o agendamento.
    ///
    /// ## Returns
    /// `Result<PipelineSchedule, GitLabError>` — dados do agendamento atualizado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn update(&self, project_id: u64, schedule_id: u64, payload: &UpdatePipelineSchedulePayload) -> Result<PipelineSchedule, GitLabError> {
        let path = format!("projects/{}/pipeline_schedules/{}", project_id, schedule_id);
        self.http.put(&path, &payload, "pipeline_schedules.update")
    }

    /// Remove um agendamento de pipeline.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `schedule_id`: ID do agendamento no GitLab.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn delete(&self, project_id: u64, schedule_id: u64) -> Result<(), GitLabError> {
        let path = format!("projects/{}/pipeline_schedules/{}", project_id, schedule_id);
        self.http.delete(&path, &[], "pipeline_schedules.delete")
    }

    /// Assume a propriedade de um agendamento de pipeline.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `schedule_id`: ID do agendamento no GitLab.
    ///
    /// ## Returns
    /// `Result<PipelineSchedule, GitLabError>` — dados do agendamento com nova propriedade.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn take_ownership(&self, project_id: u64, schedule_id: u64) -> Result<PipelineSchedule, GitLabError> {
        let path = format!("projects/{}/pipeline_schedules/{}/take_ownership", project_id, schedule_id);
        self.http.post(&path, &serde_json::Value::Null, "pipeline_schedules.take_ownership")
    }

    /// Cria uma variável para um agendamento de pipeline.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `schedule_id`: ID do agendamento no GitLab.
    /// - `key`: Nome da variável.
    /// - `value`: Valor da variável.
    ///
    /// ## Returns
    /// `Result<PipelineScheduleVariable, GitLabError>` — dados da variável criada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn create_variable(&self, project_id: u64, schedule_id: u64, key: &str, value: &str) -> Result<PipelineScheduleVariable, GitLabError> {
        let path = format!("projects/{}/pipeline_schedules/{}/variables", project_id, schedule_id);
        let body = serde_json::json!({ "key": key, "value": value });
        self.http.post(&path, &body, "pipeline_schedules.create_variable")
    }

    /// Atualiza uma variável de um agendamento de pipeline.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `schedule_id`: ID do agendamento no GitLab.
    /// - `variable_id`: ID da variável no GitLab.
    /// - `value`: Novo valor da variável.
    ///
    /// ## Returns
    /// `Result<PipelineScheduleVariable, GitLabError>` — dados da variável atualizada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn update_variable(&self, project_id: u64, schedule_id: u64, variable_id: u64, value: &str) -> Result<PipelineScheduleVariable, GitLabError> {
        let path = format!("projects/{}/pipeline_schedules/{}/variables/{}", project_id, schedule_id, variable_id);
        let body = serde_json::json!({ "value": value });
        self.http.put(&path, &body, "pipeline_schedules.update_variable")
    }

    /// Remove uma variável de um agendamento de pipeline.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `schedule_id`: ID do agendamento no GitLab.
    /// - `variable_id`: ID da variável no GitLab.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn delete_variable(&self, project_id: u64, schedule_id: u64, variable_id: u64) -> Result<(), GitLabError> {
        let path = format!("projects/{}/pipeline_schedules/{}/variables/{}", project_id, schedule_id, variable_id);
        self.http.delete(&path, &[], "pipeline_schedules.delete_variable")
    }
}
