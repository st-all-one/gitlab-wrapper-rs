use serde::{Deserialize, Serialize};
use crate::types::base::GitLabId;
use crate::types::commit::Commit;

/// Resposta da API GitLab representando um job (trabalho) de CI/CD.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Job {
    /// ID do job.
    pub id: GitLabId,
    /// Pipeline ao qual o job pertence.
    pub pipeline: Option<JobPipeline>,
    /// Referência (branch/tag) — renomeado de `ref` para `ref_` devido a palavra reservada.
    #[serde(rename = "ref")]
    pub ref_: Option<String>,
    /// Estágio do job no pipeline.
    pub stage: Option<String>,
    /// Nome do job.
    pub name: Option<String>,
    /// Status atual (pending, running, success, failed, etc.).
    pub status: Option<String>,
    /// Data de criação.
    pub created_at: Option<String>,
    /// Data de início.
    pub started_at: Option<String>,
    /// Data de conclusão.
    pub finished_at: Option<String>,
    /// Duração do job em segundos.
    pub duration: Option<f64>,
    /// Tempo na fila em segundos.
    pub queued_duration: Option<f64>,
    /// Usuário que disparou o job.
    pub user: Option<serde_json::Value>,
    /// Runner que executou o job.
    pub runner: Option<JobRunner>,
    /// Artefatos do job.
    pub artifacts: Option<Vec<JobArtifact>>,
    /// Commit associado ao job.
    pub commit: Option<Commit>,
    /// URL da página web do job.
    pub web_url: Option<String>,
    /// Indica se o job foi disparado por uma tag.
    pub tag: Option<bool>,
    /// Indica se o job permite falha sem falhar o pipeline.
    pub allow_failure: Option<bool>,
    /// Indica se o job foi retentado.
    pub retried: Option<bool>,
    /// Indica se o job pode ser executado novamente.
    pub playable: Option<bool>,
    /// Indica se o job pode ser retentado.
    pub retryable: Option<bool>,
    /// Indica se o job pode ser cancelado.
    pub cancelable: Option<bool>,
    /// Data de apagamento do job.
    pub erased_at: Option<String>,
    /// Data de expiração dos artefatos.
    pub artifacts_expire_at: Option<String>,
}

/// Informações resumidas do pipeline associado a um job.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct JobPipeline {
    /// ID do pipeline.
    pub id: GitLabId,
    /// Referência (branch/tag) — renomeado de `ref` para `ref_` devido a palavra reservada.
    #[serde(rename = "ref")]
    pub ref_: Option<String>,
    /// SHA do commit do pipeline.
    pub sha: Option<String>,
    /// Status do pipeline.
    pub status: Option<String>,
}

/// Runner que executou um job.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct JobRunner {
    /// ID do runner.
    pub id: Option<GitLabId>,
    /// Descrição do runner.
    pub description: Option<String>,
    /// Indica se o runner está ativo.
    pub active: Option<bool>,
    /// Indica se o runner é compartilhado.
    pub is_shared: Option<bool>,
    /// Tipo do runner (instance_type, group_type, project_type).
    pub runner_type: Option<String>,
    /// Status do runner.
    pub status: Option<String>,
}

/// Artefato gerado por um job.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct JobArtifact {
    /// Tipo do arquivo do artefato.
    pub file_type: Option<String>,
    /// Tamanho do artefato em bytes.
    pub size: Option<u64>,
    /// Nome do arquivo do artefato.
    pub filename: Option<String>,
    /// Formato do arquivo do artefato.
    pub file_format: Option<String>,
}

/// Filtros para listar jobs. Use `..Default::default()`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct JobFilter {
    /// Escopo dos jobs (created, pending, running, failed, success, etc.).
    pub scope: Option<Vec<String>>,
    /// Número da página para paginação.
    pub page: Option<u32>,
    /// Número de itens por página.
    pub per_page: Option<u32>,
}
