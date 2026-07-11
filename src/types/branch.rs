use serde::{Deserialize, Serialize};

/// Resposta da API GitLab representando uma branch de um repositório.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Branch {
    /// Nome da branch.
    pub name: String,
    /// Indica se a branch já foi merged.
    pub merged: Option<bool>,
    /// Indica se a branch é protegida.
    pub protected: Option<bool>,
    /// Indica se é a branch padrão do repositório.
    pub default: Option<bool>,
    /// Indica se o usuário atual pode fazer push.
    pub can_push: Option<bool>,
    /// Indica se desenvolvedores podem fazer push.
    pub developers_can_push: Option<bool>,
    /// Indica se desenvolvedores podem fazer merge.
    pub developers_can_merge: Option<bool>,
    /// Último commit da branch.
    pub commit: Option<BranchCommit>,
    /// URL da branch no GitLab.
    pub web_url: Option<String>,
}

/// Resposta da API GitLab representando o commit associado a uma branch.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BranchCommit {
    /// Hash SHA-1 completo do commit.
    pub id: Option<String>,
    /// Hash SHA-1 abreviado do commit.
    pub short_id: Option<String>,
    /// Título do commit.
    pub title: Option<String>,
    /// Mensagem completa do commit.
    pub message: Option<String>,
    /// Nome do autor do commit.
    pub author_name: Option<String>,
    /// E-mail do autor do commit.
    pub author_email: Option<String>,
    /// Data de autoria no formato ISO 8601.
    pub authored_date: Option<String>,
    /// Nome do committer.
    pub committer_name: Option<String>,
    /// E-mail do committer.
    pub committer_email: Option<String>,
    /// Data do commit no formato ISO 8601.
    pub committed_date: Option<String>,
    /// URL do commit no GitLab.
    pub web_url: Option<String>,
}

/// Payload para criar uma branch na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateBranchPayload {
    /// Nome da nova branch.
    pub branch: String,
    /// Nome da branch de origem, tag ou SHA do commit de referência.
    #[serde(rename = "ref")]
    pub ref_: String,
}
