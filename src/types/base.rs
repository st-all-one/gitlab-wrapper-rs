use serde::{Deserialize, Serialize};

/// Identificador numérico de recurso no GitLab.
pub type GitLabId = u64;

/// Resposta da API GitLab representando informações de um autor
/// (usuário que criou ou modificou um recurso).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuthorInfo {
    /// ID único do autor.
    pub id: GitLabId,
    /// Nome de usuário do autor.
    pub username: String,
    /// Nome completo do autor.
    pub name: String,
    /// Estado atual do autor (ex.: "active", "blocked").
    pub state: Option<String>,
    /// URL do avatar do autor.
    pub avatar_url: Option<String>,
    /// URL do perfil do autor no GitLab.
    pub web_url: Option<String>,
}

/// Resposta da API GitLab representando links relacionados a um recurso
/// (ex.: links para notas, emojis, projeto).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Links {
    /// Link para o próprio recurso.
    #[serde(rename = "self")]
    pub self_: Option<String>,
    /// Link para as notas do recurso.
    pub notes: Option<String>,
    /// Link para os emojis de premiação do recurso.
    pub award_emoji: Option<String>,
    /// Link para o projeto associado.
    pub project: Option<String>,
}

/// Resposta da API GitLab representando estatísticas de tempo
/// (estimativa e tempo gasto) de um recurso.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TimeStats {
    /// Estimativa de tempo em segundos.
    pub time_estimate: Option<i64>,
    /// Tempo total gasto em segundos.
    pub total_time_spent: Option<i64>,
    /// Estimativa de tempo em formato legível (ex.: "3d 4h").
    pub human_time_estimate: Option<String>,
    /// Tempo total gasto em formato legível (ex.: "1w 2d").
    pub human_total_time_spent: Option<String>,
}

/// Resposta da API GitLab representando o status de conclusão de tarefas
/// em um recurso (ex.: lista de verificação em uma lista).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TaskCompletionStatus {
    /// Número total de tarefas.
    pub count: Option<u32>,
    /// Número de tarefas concluídas.
    pub completed_count: Option<u32>,
}

/// Resposta da API GitLab para upload genérico de arquivos
/// (`POST /projects/:id/uploads`).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UploadResult {
    /// Texto alternativo para o arquivo.
    pub alt: Option<String>,
    /// URL relativa do arquivo carregado.
    pub url: Option<String>,
    /// Código markdown para referenciar o arquivo.
    pub markdown: Option<String>,
    /// Nome original do arquivo.
    pub file_name: Option<String>,
}

/// Resposta da API GitLab para upload de anexo em wiki
/// (`POST /projects/:id/wikis/attachments`).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WikiAttachmentResult {
    /// Nome do arquivo carregado.
    pub file_name: Option<String>,
    /// Caminho do arquivo no repositório.
    pub file_path: Option<String>,
    /// Branch onde o arquivo foi carregado.
    pub branch: Option<String>,
    /// Link para o arquivo.
    pub link: Option<String>,
}

/// Parâmetros de paginação para listagem de recursos na API GitLab.
/// Use `..Default::default()` para valores padrão.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PaginationParams {
    /// Número da página para recuperação.
    pub page: Option<u32>,
    /// Número de itens por página (máximo 100).
    pub per_page: Option<u32>,
    /// Tipo de paginação ("offset" ou "keyset").
    pub pagination: Option<String>,
    /// Campo pelo qual ordenar os resultados.
    pub order_by: Option<String>,
    /// Direção da ordenação ("asc" ou "desc").
    pub sort: Option<String>,
}
