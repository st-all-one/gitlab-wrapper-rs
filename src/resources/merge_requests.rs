use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::{
    AuthorInfo, Commit, CreateMergeRequestPayload, MergePayload, MergeRequest, MergeRequestFilter,
    Pipeline, UpdateMergeRequestPayload,
};
use crate::utils::encoding::filter_to_query;
use std::sync::Arc;

/// Recurso de API para operações com merge requests no GitLab.
#[derive(Debug)]
pub struct MergeRequestsResource {
    http: Arc<HttpClient>,
}

impl MergeRequestsResource {
    /// Cria uma nova instância do recurso.
    pub(crate) const fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista merge requests com filtros opcionais (escopo global).
    ///
    /// ## Params
    /// - `filter`: Filtros opcionais para a consulta.
    ///
    /// ## Returns
    /// `Result<Vec<MergeRequest>, GitLabError>` — lista de merge requests.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list(
        &self,
        filter: Option<&MergeRequestFilter>,
    ) -> Result<Vec<MergeRequest>, GitLabError> {
        let query = filter_to_query(filter);
        self.http
            .get("merge_requests", &query, "merge_requests.list")
            .await
    }

    /// Lista merge requests de um projeto específico.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `filter`: Filtros opcionais para a consulta.
    ///
    /// ## Returns
    /// `Result<Vec<MergeRequest>, GitLabError>` — lista de merge requests do projeto.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list_for_project(
        &self,
        project_id: u64,
        filter: Option<&MergeRequestFilter>,
    ) -> Result<Vec<MergeRequest>, GitLabError> {
        let path = format!("projects/{project_id}/merge_requests");
        let query = filter_to_query(filter);
        self.http
            .get(&path, &query, "merge_requests.list_for_project")
            .await
    }

    /// Obtém um merge request pelo ID do projeto e IID do MR.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `mr_iid`: IID do merge request no projeto.
    ///
    /// ## Returns
    /// `Result<MergeRequest, GitLabError>` — dados do merge request solicitado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get(&self, project_id: u64, mr_iid: u32) -> Result<MergeRequest, GitLabError> {
        let path = format!("projects/{project_id}/merge_requests/{mr_iid}");
        self.http.get(&path, &[], "merge_requests.get").await
    }

    /// Cria um novo merge request em um projeto.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `payload`: Dados para criar o merge request.
    ///
    /// ## Returns
    /// `Result<MergeRequest, GitLabError>` — dados do merge request criado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn create(
        &self,
        project_id: u64,
        payload: &CreateMergeRequestPayload,
    ) -> Result<MergeRequest, GitLabError> {
        let path = format!("projects/{project_id}/merge_requests");
        self.http
            .post(&path, &payload, "merge_requests.create")
            .await
    }

    /// Atualiza um merge request existente.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `mr_iid`: IID do merge request no projeto.
    /// - `payload`: Dados para atualizar o merge request.
    ///
    /// ## Returns
    /// `Result<MergeRequest, GitLabError>` — dados do merge request atualizado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn update(
        &self,
        project_id: u64,
        mr_iid: u32,
        payload: &UpdateMergeRequestPayload,
    ) -> Result<MergeRequest, GitLabError> {
        let path = format!("projects/{project_id}/merge_requests/{mr_iid}");
        self.http
            .put(&path, &payload, "merge_requests.update")
            .await
    }

    /// Remove um merge request.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `mr_iid`: IID do merge request no projeto.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn delete(&self, project_id: u64, mr_iid: u32) -> Result<(), GitLabError> {
        let path = format!("projects/{project_id}/merge_requests/{mr_iid}");
        self.http.delete(&path, &[], "merge_requests.delete").await
    }

    /// Realiza o merge de um merge request.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `mr_iid`: IID do merge request no projeto.
    /// - `payload`: Parâmetros opcionais para o merge.
    ///
    /// ## Returns
    /// `Result<MergeRequest, GitLabError>` — dados do merge request mesclado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn merge(
        &self,
        project_id: u64,
        mr_iid: u32,
        payload: Option<&MergePayload>,
    ) -> Result<MergeRequest, GitLabError> {
        let path = format!("projects/{project_id}/merge_requests/{mr_iid}/merge");
        let body = payload.unwrap_or(&MergePayload {
            merge_commit_message: None,
            squash_commit_message: None,
            should_remove_source_branch: None,
        });
        self.http.put(&path, &body, "merge_requests.merge").await
    }

    /// Cancela o merge automático quando a pipeline for bem-sucedida.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `mr_iid`: IID do merge request no projeto.
    ///
    /// ## Returns
    /// `Result<MergeRequest, GitLabError>` — dados do merge request atualizado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn cancel_merge_when_pipeline_succeeds(
        &self,
        project_id: u64,
        mr_iid: u32,
    ) -> Result<MergeRequest, GitLabError> {
        let path = format!(
            "projects/{project_id}/merge_requests/{mr_iid}/cancel_merge_when_pipeline_succeeds"
        );
        self.http
            .post(
                &path,
                &serde_json::json!({}),
                "merge_requests.cancel_merge_when_pipeline_succeeds",
            )
            .await
    }

    /// Aprova um merge request.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `mr_iid`: IID do merge request no projeto.
    ///
    /// ## Returns
    /// `Result<MergeRequest, GitLabError>` — dados do merge request aprovado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn approve(
        &self,
        project_id: u64,
        mr_iid: u32,
    ) -> Result<serde_json::Value, GitLabError> {
        let path = format!("projects/{project_id}/merge_requests/{mr_iid}/approve");
        self.http
            .post(&path, &serde_json::json!({}), "merge_requests.approve")
            .await
    }

    /// Remove a aprovação de um merge request.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `mr_iid`: IID do merge request no projeto.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn unapprove(
        &self,
        project_id: u64,
        mr_iid: u32,
    ) -> Result<serde_json::Value, GitLabError> {
        let path = format!("projects/{project_id}/merge_requests/{mr_iid}/unapprove");
        self.http
            .post(&path, &serde_json::json!({}), "merge_requests.unapprove")
            .await
    }

    /// Faz rebase de um merge request.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `mr_iid`: IID do merge request no projeto.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn rebase(&self, project_id: u64, mr_iid: u32) -> Result<(), GitLabError> {
        let path = format!("projects/{project_id}/merge_requests/{mr_iid}/rebase");
        self.http
            .put(&path, &serde_json::json!({}), "merge_requests.rebase")
            .await
    }

    /// Lista commits associados a um merge request.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `mr_iid`: IID do merge request no projeto.
    ///
    /// ## Returns
    /// `Result<Vec<Commit>, GitLabError>` — lista de commits do MR.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn commits(&self, project_id: u64, mr_iid: u32) -> Result<Vec<Commit>, GitLabError> {
        let path = format!("projects/{project_id}/merge_requests/{mr_iid}/commits");
        self.http.get(&path, &[], "merge_requests.commits").await
    }

    /// Lista as alterações (diff) de um merge request.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `mr_iid`: IID do merge request no projeto.
    ///
    /// ## Returns
    /// `Result<serde_json::Value, GitLabError>` — dados das alterações do MR.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn changes(
        &self,
        project_id: u64,
        mr_iid: u32,
    ) -> Result<serde_json::Value, GitLabError> {
        let path = format!("projects/{project_id}/merge_requests/{mr_iid}/changes");
        self.http.get(&path, &[], "merge_requests.changes").await
    }

    /// Lista pipelines associados a um merge request.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `mr_iid`: IID do merge request no projeto.
    ///
    /// ## Returns
    /// `Result<Vec<Pipeline>, GitLabError>` — lista de pipelines.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn pipelines(
        &self,
        project_id: u64,
        mr_iid: u32,
    ) -> Result<Vec<Pipeline>, GitLabError> {
        let path = format!("projects/{project_id}/merge_requests/{mr_iid}/pipelines");
        self.http.get(&path, &[], "merge_requests.pipelines").await
    }

    /// Cria um pipeline para um merge request.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `mr_iid`: IID do merge request no projeto.
    ///
    /// ## Returns
    /// `Result<Pipeline, GitLabError>` — dados do pipeline criado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn create_pipeline(
        &self,
        project_id: u64,
        mr_iid: u32,
    ) -> Result<Pipeline, GitLabError> {
        let path = format!("projects/{project_id}/merge_requests/{mr_iid}/pipelines");
        self.http
            .post(
                &path,
                &serde_json::json!({}),
                "merge_requests.create_pipeline",
            )
            .await
    }

    /// Lista participantes de um merge request.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `mr_iid`: IID do merge request no projeto.
    ///
    /// ## Returns
    /// `Result<Vec<AuthorInfo>, GitLabError>` — lista de participantes.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn participants(
        &self,
        project_id: u64,
        mr_iid: u32,
    ) -> Result<Vec<AuthorInfo>, GitLabError> {
        let path = format!("projects/{project_id}/merge_requests/{mr_iid}/participants");
        self.http
            .get(&path, &[], "merge_requests.participants")
            .await
    }

    /// Obtém o status de inscrição em um merge request.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `mr_iid`: IID do merge request no projeto.
    ///
    /// ## Returns
    /// `Result<serde_json::Value, GitLabError>` — status de inscrição.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn subscription(
        &self,
        project_id: u64,
        mr_iid: u32,
    ) -> Result<serde_json::Value, GitLabError> {
        let path = format!("projects/{project_id}/merge_requests/{mr_iid}/subscription");
        self.http
            .get(&path, &[], "merge_requests.subscription")
            .await
    }

    /// Inscreve-se em um merge request.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `mr_iid`: IID do merge request no projeto.
    ///
    /// ## Returns
    /// `Result<serde_json::Value, GitLabError>` — status de inscrição atualizado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn subscribe(
        &self,
        project_id: u64,
        mr_iid: u32,
    ) -> Result<serde_json::Value, GitLabError> {
        let path = format!("projects/{project_id}/merge_requests/{mr_iid}/subscribe");
        self.http
            .post(&path, &serde_json::json!({}), "merge_requests.subscribe")
            .await
    }

    /// Cancela a inscrição em um merge request.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `mr_iid`: IID do merge request no projeto.
    ///
    /// ## Returns
    /// `Result<serde_json::Value, GitLabError>` — status de inscrição atualizado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn unsubscribe(
        &self,
        project_id: u64,
        mr_iid: u32,
    ) -> Result<serde_json::Value, GitLabError> {
        let path = format!("projects/{project_id}/merge_requests/{mr_iid}/unsubscribe");
        self.http
            .post(&path, &serde_json::json!({}), "merge_requests.unsubscribe")
            .await
    }

    /// Define uma estimativa de tempo para um merge request.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `mr_iid`: IID do merge request no projeto.
    /// - `duration`: Duração no formato "3h30m".
    ///
    /// ## Returns
    /// `Result<serde_json::Value, GitLabError>` — dados atualizados de tempo.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn set_time_estimate(
        &self,
        project_id: u64,
        mr_iid: u32,
        duration: &str,
    ) -> Result<serde_json::Value, GitLabError> {
        let path = format!("projects/{project_id}/merge_requests/{mr_iid}/time_estimate");
        let body = serde_json::json!({ "duration": duration });
        self.http
            .post(&path, &body, "merge_requests.set_time_estimate")
            .await
    }

    /// Adiciona tempo gasto a um merge request.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `mr_iid`: IID do merge request no projeto.
    /// - `duration`: Duração no formato "3h30m".
    ///
    /// ## Returns
    /// `Result<serde_json::Value, GitLabError>` — dados atualizados de tempo.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn add_spent_time(
        &self,
        project_id: u64,
        mr_iid: u32,
        duration: &str,
    ) -> Result<serde_json::Value, GitLabError> {
        let path = format!("projects/{project_id}/merge_requests/{mr_iid}/add_spent_time");
        let body = serde_json::json!({ "duration": duration });
        self.http
            .post(&path, &body, "merge_requests.add_spent_time")
            .await
    }

    /// Redefine a estimativa de tempo de um merge request.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `mr_iid`: IID do merge request no projeto.
    ///
    /// ## Returns
    /// `Result<serde_json::Value, GitLabError>` — dados atualizados de tempo.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn reset_time_estimate(
        &self,
        project_id: u64,
        mr_iid: u32,
    ) -> Result<serde_json::Value, GitLabError> {
        let path = format!("projects/{project_id}/merge_requests/{mr_iid}/reset_time_estimate");
        self.http
            .post(
                &path,
                &serde_json::json!({}),
                "merge_requests.reset_time_estimate",
            )
            .await
    }

    /// Redefine o tempo gasto de um merge request.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `mr_iid`: IID do merge request no projeto.
    ///
    /// ## Returns
    /// `Result<serde_json::Value, GitLabError>` — dados atualizados de tempo.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn reset_spent_time(
        &self,
        project_id: u64,
        mr_iid: u32,
    ) -> Result<serde_json::Value, GitLabError> {
        let path = format!("projects/{project_id}/merge_requests/{mr_iid}/reset_spent_time");
        self.http
            .post(
                &path,
                &serde_json::json!({}),
                "merge_requests.reset_spent_time",
            )
            .await
    }

    /// Lista merge requests de um grupo.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    /// - `filter`: Filtros opcionais para a consulta.
    ///
    /// ## Returns
    /// `Result<Vec<MergeRequest>, GitLabError>` — lista de merge requests do grupo.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list_by_group(
        &self,
        group_id: u64,
        filter: Option<&MergeRequestFilter>,
    ) -> Result<Vec<MergeRequest>, GitLabError> {
        let path = format!("groups/{group_id}/merge_requests");
        let query = filter_to_query(filter);
        self.http
            .get(&path, &query, "merge_requests.list_by_group")
            .await
    }
}
