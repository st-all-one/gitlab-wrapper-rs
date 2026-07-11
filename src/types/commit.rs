use serde::{Deserialize, Serialize};

/// Resposta da API GitLab representando um commit.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Commit {
    /// Hash SHA-1 completo do commit.
    pub id: String,
    /// Hash SHA-1 abreviado do commit.
    pub short_id: Option<String>,
    /// Título do commit (primeira linha da mensagem).
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
    /// Estatísticas do commit (adições, deleções, total).
    pub stats: Option<CommitStats>,
    /// Status do pipeline associado ao commit.
    pub status: Option<String>,
}

/// Resposta da API GitLab representando estatísticas de um commit.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CommitStats {
    /// Número de linhas adicionadas.
    pub additions: Option<u32>,
    /// Número de linhas removidas.
    pub deletions: Option<u32>,
    /// Número total de alterações.
    pub total: Option<u32>,
}

/// Payload para criar um commit na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateCommitPayload {
    /// Nome da branch onde o commit será criado.
    pub branch: String,
    /// Mensagem do commit.
    pub commit_message: String,
    /// Ações a serem realizadas no commit (criar, atualizar, excluir arquivos).
    pub actions: Vec<CommitAction>,
}

/// Payload representando uma ação individual dentro de um commit.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CommitAction {
    /// Tipo de ação ("create", "update", "delete", "move", "chmod").
    pub action: String,
    /// Caminho do arquivo alvo da ação.
    pub file_path: String,
    /// Conteúdo do arquivo (obrigatório para create/update).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Codificação do conteúdo ("text" ou "base64").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    /// Caminho anterior do arquivo (usado em rename/move).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_path: Option<String>,
}

/// Resposta da API GitLab representando um diff de commit.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CommitDiff {
    /// Conteúdo do diff.
    pub diff: Option<String>,
    /// Caminho do novo arquivo.
    pub new_path: Option<String>,
    /// Caminho do arquivo antigo.
    pub old_path: Option<String>,
    /// Modo do arquivo antigo.
    pub a_mode: Option<String>,
    /// Modo do novo arquivo.
    pub b_mode: Option<String>,
    /// Indica se é um arquivo novo.
    pub new_file: Option<bool>,
    /// Indica se o arquivo foi renomeado.
    pub renamed_file: Option<bool>,
    /// Indica se o arquivo foi excluído.
    pub deleted_file: Option<bool>,
}

/// Filtros disponíveis para listar commits. Use `..Default::default()` para valores padrão.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CommitFilter {
    /// Nome da branch ou tag para filtrar.
    pub ref_name: Option<String>,
    /// Caminho do arquivo para filtrar commits.
    pub path: Option<String>,
    /// Data inicial (ISO 8601) para filtrar commits.
    pub since: Option<String>,
    /// Data final (ISO 8601) para filtrar commits.
    pub until: Option<String>,
    /// Nome do autor para filtrar commits.
    pub author: Option<String>,
    /// Indica se deve incluir estatísticas na resposta.
    pub with_stats: Option<bool>,
    /// Número da página para recuperação.
    pub page: Option<u32>,
    /// Número de itens por página.
    pub per_page: Option<u32>,
}
