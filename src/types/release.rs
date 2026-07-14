use crate::types::base::*;
use crate::types::branch::BranchCommit;
use serde::{Deserialize, Serialize};

/// Resposta da API GitLab representando uma release (versão).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Release {
    /// Nome da tag da release.
    pub tag_name: Option<String>,
    /// Caminho da tag da release.
    pub tag_path: Option<String>,
    /// Descrição da release.
    pub description: Option<String>,
    /// Descrição em HTML da release.
    pub description_html: Option<String>,
    /// Nome da release.
    pub name: Option<String>,
    /// Data de criação.
    pub created_at: Option<String>,
    /// Data de publicação da release.
    pub released_at: Option<String>,
    /// Autor da release.
    pub author: Option<AuthorInfo>,
    /// Commit associado à release.
    pub commit: Option<BranchCommit>,
    /// Ativos (assets) da release.
    pub assets: Option<ReleaseAssets>,
    /// Evidências da release.
    pub evidences: Option<Vec<ReleaseEvidence>>,
    /// Links relacionados à release.
    pub _links: Option<ReleaseLinks>,
}

/// Ativos (assets) de uma release.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReleaseAssets {
    /// Quantidade de ativos.
    pub count: Option<u32>,
    /// Fontes disponíveis para download.
    pub sources: Option<Vec<ReleaseSource>>,
    /// Links dos ativos.
    pub links: Option<Vec<ReleaseLinkItem>>,
}

/// Fonte para download de uma release.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReleaseSource {
    /// Formato do arquivo (zip, tar.gz, etc.).
    pub format: Option<String>,
    /// URL para download.
    pub url: Option<String>,
}

/// Item de link de uma release.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReleaseLinkItem {
    /// ID do link.
    pub id: Option<GitLabId>,
    /// Nome do link.
    pub name: Option<String>,
    /// URL do link.
    pub url: Option<String>,
}

/// Evidência de uma release.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReleaseEvidence {
    /// SHA da evidência.
    pub sha: Option<String>,
    /// Caminho do arquivo de evidência.
    pub filepath: Option<String>,
    /// Data de coleta da evidência.
    pub collected_at: Option<String>,
}

/// Links relacionados a uma release.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReleaseLinks {
    /// URL da própria release.
    #[serde(rename = "self")]
    pub self_: Option<String>,
    /// URL de edição da release.
    pub edit_url: Option<String>,
}

/// Payload para criar uma release na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateReleasePayload {
    /// Nome da tag da release (obrigatório).
    pub tag_name: String,
    /// Descrição da release (obrigatório).
    pub description: String,
    /// Nome da release.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Referência (branch/tag) — renomeado de `ref` para `ref_` devido a palavra reservada.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ref")]
    pub ref_: Option<String>,
    /// Data de publicação da release.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub released_at: Option<String>,
    /// Lista de marcos (milestones) associados.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milestones: Option<Vec<String>>,
}

/// Payload para atualizar uma release na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateReleasePayload {
    /// Descrição da release.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Nome da release.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Data de publicação da release.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub released_at: Option<String>,
    /// Lista de marcos (milestones) associados.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milestones: Option<Vec<String>>,
}

/// Payload para criar um link de release na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateReleaseLinkPayload {
    /// Nome do link (obrigatório).
    pub name: String,
    /// URL do link (obrigatório).
    pub url: String,
    /// Caminho do arquivo associado.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filepath: Option<String>,
    /// Tipo do link (other, runbook, image, package, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_type: Option<String>,
}
