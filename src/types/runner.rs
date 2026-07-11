use serde::{Deserialize, Serialize};
use crate::types::base::GitLabId;

/// Resposta da API GitLab representando um runner de CI/CD.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Runner {
    /// ID do runner.
    pub id: GitLabId,
    /// Descrição do runner.
    pub description: Option<String>,
    /// Endereço IP do runner.
    pub ip_address: Option<String>,
    /// Indica se o runner está ativo.
    pub active: Option<bool>,
    /// Indica se o runner está pausado.
    pub paused: Option<bool>,
    /// Indica se o runner é compartilhado entre projetos.
    pub is_shared: Option<bool>,
    /// Tipo do runner (instance_type, group_type, project_type).
    pub runner_type: Option<String>,
    /// Status do runner (online, offline, paused, etc.).
    pub status: Option<String>,
    /// Indica se o runner está online.
    pub online: Option<bool>,
    /// Arquitetura do runner.
    pub architecture: Option<String>,
    /// Plataforma do runner.
    pub platform: Option<String>,
    /// Indica se o runner está bloqueado.
    pub locked: Option<bool>,
    /// Nível de acesso do runner.
    pub access_level: Option<String>,
    /// Versão do runner.
    pub version: Option<String>,
    /// Revisão do runner.
    pub revision: Option<String>,
    /// Lista de tags do runner.
    pub tag_list: Option<Vec<String>>,
    /// Indica se o runner pode executar jobs sem tags.
    pub run_untagged: Option<bool>,
    /// Tempo máximo de execução em segundos.
    pub maximum_timeout: Option<u32>,
    /// Projetos associados ao runner.
    pub projects: Option<Vec<RunnerProject>>,
}

/// Projeto associado a um runner.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RunnerProject {
    /// ID do projeto.
    pub id: GitLabId,
    /// Nome do projeto.
    pub name: Option<String>,
    /// Caminho completo do projeto.
    pub full_path: Option<String>,
    /// URL da página web do projeto.
    pub web_url: Option<String>,
}

/// Payload para criar um runner na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateRunnerPayload {
    /// Tipo do runner (instance_type, group_type, project_type).
    pub runner_type: String,
    /// Descrição do runner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Lista de tags do runner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<String>>,
    /// Indica se o runner pode executar jobs sem tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_untagged: Option<bool>,
    /// Indica se o runner está bloqueado.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    /// Nível de acesso do runner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_level: Option<String>,
    /// Tempo máximo de execução em segundos.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_timeout: Option<u32>,
}

/// Payload para atualizar um runner na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateRunnerPayload {
    /// Descrição do runner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Lista de tags do runner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<String>>,
    /// Indica se o runner pode executar jobs sem tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_untagged: Option<bool>,
    /// Indica se o runner está bloqueado.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    /// Nível de acesso do runner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_level: Option<String>,
    /// Tempo máximo de execução em segundos.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_timeout: Option<u32>,
    /// Indica se o runner está ativo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}
