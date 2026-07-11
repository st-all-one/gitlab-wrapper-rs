use serde::{Deserialize, Serialize};
use crate::types::base::*;

/// Resposta da API GitLab representando um pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Pipeline {
    /// ID do pipeline.
    pub id: GitLabId,
    /// ID do projeto ao qual o pipeline pertence.
    pub project_id: Option<GitLabId>,
    /// Referência (branch/tag) — renomeado de `ref` para `ref_` devido a palavra reservada.
    #[serde(rename = "ref")]
    pub ref_: Option<String>,
    /// SHA do commit do pipeline.
    pub sha: Option<String>,
    /// SHA do commit anterior.
    pub before_sha: Option<String>,
    /// Status atual (pending, running, success, failed).
    pub status: Option<String>,
    /// Status detalhado do pipeline.
    pub detailed_status: Option<String>,
    /// Lista de estágios do pipeline.
    pub stages: Option<Vec<String>>,
    /// Data de criação.
    pub created_at: Option<String>,
    /// Data de atualização.
    pub updated_at: Option<String>,
    /// Data de início.
    pub started_at: Option<String>,
    /// Data de conclusão.
    pub finished_at: Option<String>,
    /// Data do commit.
    pub committed_at: Option<String>,
    /// Duração do pipeline em segundos.
    pub duration: Option<f64>,
    /// Tempo na fila em segundos.
    pub queued_duration: Option<f64>,
    /// Usuário que disparou o pipeline.
    pub user: Option<AuthorInfo>,
    /// Origem do pipeline (push, web, schedule, etc.).
    pub source: Option<String>,
    /// URL da página web do pipeline.
    pub web_url: Option<String>,
    /// Erros de YAML do pipeline.
    pub yaml_errors: Option<String>,
    /// Indica se o pipeline foi disparado por uma tag.
    pub tag: Option<bool>,
    /// Nome do pipeline.
    pub name: Option<String>,
}

/// Variável de um pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PipelineVariable {
    /// Chave da variável.
    pub key: Option<String>,
    /// Valor da variável.
    pub value: Option<String>,
    /// Tipo da variável (env_var, file).
    pub variable_type: Option<String>,
}

/// Payload para criar um pipeline na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreatePipelinePayload {
    /// Referência (branch/tag) para o pipeline — renomeado de `ref` para `ref_` devido a palavra reservada.
    #[serde(rename = "ref")]
    pub ref_: String,
    /// Variáveis do pipeline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<Vec<PipelineVariable>>,
}

/// Filtros para listar pipelines. Use `..Default::default()`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PipelineFilter {
    /// Filtro por escopo (running, pending, finished, etc.).
    pub scope: Option<String>,
    /// Filtro por status (pending, running, success, failed, etc.).
    pub status: Option<String>,
    /// Filtro por referência (branch/tag) — renomeado de `ref` para `ref_` devido a palavra reservada.
    #[serde(rename = "ref")]
    pub ref_: Option<String>,
    /// Filtro por SHA do commit.
    pub sha: Option<String>,
    /// Filtro por origem (push, web, schedule, etc.).
    pub source: Option<String>,
    /// Filtro por nome de usuário.
    pub username: Option<String>,
    /// Filtro por data de atualização inicial (ISO 8601).
    pub updated_after: Option<String>,
    /// Filtro por data de atualização final (ISO 8601).
    pub updated_before: Option<String>,
    /// Número da página para paginação.
    pub page: Option<u32>,
    /// Número de itens por página.
    pub per_page: Option<u32>,
}
