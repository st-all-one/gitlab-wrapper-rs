use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use std::sync::Arc;
use crate::types::*;
use crate::utils::encoding::encode_query_param;

/// Recurso de API para operações com labels no GitLab.
#[derive(Debug)]
pub struct LabelsResource {
    http: Arc<HttpClient>,
}

impl LabelsResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista labels de um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<Label>, GitLabError>` — lista de labels do projeto.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn list_project_labels(&self, project_id: u64) -> Result<Vec<Label>, GitLabError> {
        let path = format!("projects/{}/labels", project_id);
        self.http.get(&path, &[], "labels.list_project")
    }

    /// Obtém uma label pelo ID.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `label_id`: ID da label.
    ///
    /// ## Returns
    /// `Result<Label, GitLabError>` — dados da label solicitada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn get_project_label(&self, project_id: u64, label_id: u64) -> Result<Label, GitLabError> {
        let path = format!("projects/{}/labels/{}", project_id, label_id);
        self.http.get(&path, &[], "labels.get_project")
    }

    /// Cria uma nova label em um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `payload`: Dados para criar a label.
    ///
    /// ## Returns
    /// `Result<Label, GitLabError>` — dados da label criada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn create_project_label(&self, project_id: u64, payload: &CreateLabelPayload) -> Result<Label, GitLabError> {
        let path = format!("projects/{}/labels", project_id);
        self.http.post(&path, &payload, "labels.create_project")
    }

    /// Atualiza uma label de projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `payload`: Dados para atualizar a label.
    ///
    /// ## Returns
    /// `Result<Label, GitLabError>` — dados da label atualizada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn update_project_label(&self, project_id: u64, payload: &UpdateLabelPayload) -> Result<Label, GitLabError> {
        let path = format!("projects/{}/labels", project_id);
        self.http.put(&path, &payload, "labels.update_project")
    }

    /// Remove uma label de projeto pelo nome.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `name`: Nome da label a ser removida.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn delete_project_label(&self, project_id: u64, name: &str) -> Result<(), GitLabError> {
        let path = format!("projects/{}/labels/{}", project_id, encode_query_param(name));
        self.http.delete(&path, &[], "labels.delete_project")
    }

    /// Promove uma label de projeto a label de grupo.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `name`: Nome da label a ser promovida.
    ///
    /// ## Returns
    /// `Result<GroupLabel, GitLabError>` — dados da label de grupo promovida.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn promote_project_label(&self, project_id: u64, name: &str) -> Result<GroupLabel, GitLabError> {
        let path = format!("projects/{}/labels/{}/promote", project_id, encode_query_param(name));
        self.http.put(&path, &serde_json::Value::Null, "labels.promote_project")
    }

    /// Lista labels de um grupo.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<GroupLabel>, GitLabError>` — lista de labels do grupo.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn list_group_labels(&self, group_id: u64) -> Result<Vec<GroupLabel>, GitLabError> {
        let path = format!("groups/{}/labels", group_id);
        self.http.get(&path, &[], "labels.list_group")
    }

    /// Obtém uma label de grupo pelo ID.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    /// - `label_id`: ID da label.
    ///
    /// ## Returns
    /// `Result<GroupLabel, GitLabError>` — dados da label de grupo solicitada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn get_group_label(&self, group_id: u64, label_id: u64) -> Result<GroupLabel, GitLabError> {
        let path = format!("groups/{}/labels/{}", group_id, label_id);
        self.http.get(&path, &[], "labels.get_group")
    }

    /// Cria uma nova label em um grupo.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    /// - `payload`: Dados para criar a label.
    ///
    /// ## Returns
    /// `Result<GroupLabel, GitLabError>` — dados da label de grupo criada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn create_group_label(&self, group_id: u64, payload: &CreateLabelPayload) -> Result<GroupLabel, GitLabError> {
        let path = format!("groups/{}/labels", group_id);
        self.http.post(&path, &payload, "labels.create_group")
    }

    /// Atualiza uma label de grupo.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    /// - `payload`: Dados para atualizar a label.
    ///
    /// ## Returns
    /// `Result<GroupLabel, GitLabError>` — dados da label de grupo atualizada.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn update_group_label(&self, group_id: u64, payload: &UpdateLabelPayload) -> Result<GroupLabel, GitLabError> {
        let path = format!("groups/{}/labels", group_id);
        self.http.put(&path, &payload, "labels.update_group")
    }

    /// Remove uma label de grupo pelo nome.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    /// - `name`: Nome da label a ser removida.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn delete_group_label(&self, group_id: u64, name: &str) -> Result<(), GitLabError> {
        let path = format!("groups/{}/labels/{}", group_id, encode_query_param(name));
        self.http.delete(&path, &[], "labels.delete_group")
    }
}
