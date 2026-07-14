use crate::types::base::*;
use serde::{Deserialize, Serialize};

/// Resposta da API GitLab representando uma regra de push de grupo.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GroupPushRule {
    /// ID único da regra de push.
    pub id: GitLabId,
    /// ID do grupo ao qual a regra pertence.
    pub group_id: Option<GitLabId>,
    /// Impede a exclusão de tags.
    pub deny_delete_tag: Option<bool>,
    /// Exige verificação de associação ao grupo.
    pub member_check: Option<bool>,
    /// Impede o push de secrets no repositório.
    pub prevent_secrets: Option<bool>,
    /// Exige verificação do autor do commit.
    pub commit_committer_check: Option<bool>,
    /// Rejeita commits não assinados.
    pub reject_unsigned_commits: Option<bool>,
    /// Regex para validação da mensagem do commit.
    pub commit_message_regex: Option<String>,
    /// Regex para validação do nome da branch.
    pub branch_name_regex: Option<String>,
    /// Regex para validação do email do autor.
    pub author_email_regex: Option<String>,
    /// Regex para validação do nome do arquivo.
    pub file_name_regex: Option<String>,
    /// Tamanho máximo de arquivo em bytes.
    pub max_file_size: Option<u64>,
}

/// Payload para criar uma regra de push de grupo.
///
/// Todos os campos são opcionais — campos não informados usarão
/// o valor padrão do GitLab.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateGroupPushRulePayload {
    /// Impede a exclusão de tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deny_delete_tag: Option<bool>,
    /// Exige verificação de associação ao grupo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_check: Option<bool>,
    /// Impede o push de secrets no repositório.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prevent_secrets: Option<bool>,
    /// Exige verificação do autor do commit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_committer_check: Option<bool>,
    /// Rejeita commits não assinados.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_unsigned_commits: Option<bool>,
    /// Regex para validação da mensagem do commit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_message_regex: Option<String>,
    /// Regex para validação do nome da branch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_name_regex: Option<String>,
    /// Regex para validação do email do autor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_email_regex: Option<String>,
    /// Regex para validação do nome do arquivo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name_regex: Option<String>,
    /// Tamanho máximo de arquivo em bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_file_size: Option<u64>,
}

/// Payload para atualizar uma regra de push de grupo.
///
/// Apenas campos informados serão alterados; campos `None` mantêm o valor atual.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateGroupPushRulePayload {
    /// Impede a exclusão de tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deny_delete_tag: Option<bool>,
    /// Exige verificação de associação ao grupo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_check: Option<bool>,
    /// Impede o push de secrets no repositório.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prevent_secrets: Option<bool>,
    /// Exige verificação do autor do commit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_committer_check: Option<bool>,
    /// Rejeita commits não assinados.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_unsigned_commits: Option<bool>,
    /// Regex para validação da mensagem do commit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_message_regex: Option<String>,
    /// Regex para validação do nome da branch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_name_regex: Option<String>,
    /// Regex para validação do email do autor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_email_regex: Option<String>,
    /// Regex para validação do nome do arquivo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name_regex: Option<String>,
    /// Tamanho máximo de arquivo em bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_file_size: Option<u64>,
}
