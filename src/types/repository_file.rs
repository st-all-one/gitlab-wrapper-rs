use serde::{Deserialize, Serialize};

/// Resposta da API GitLab representando um arquivo do repositório.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RepositoryFile {
    /// ID do arquivo.
    pub id: Option<String>,
    /// Nome do arquivo.
    pub file_name: Option<String>,
    /// Caminho completo do arquivo no repositório.
    pub file_path: Option<String>,
    /// Tamanho do arquivo em bytes.
    pub size: Option<u64>,
    /// Codificação do conteúdo ("base64" ou "text").
    pub encoding: Option<String>,
    /// Conteúdo do arquivo (codificado conforme `encoding`).
    pub content: Option<String>,
    /// Nome da branch ou tag de referência do arquivo.
    #[serde(rename = "ref")]
    pub ref_: Option<String>,
    /// ID do blob do arquivo.
    pub blob_id: Option<String>,
    /// ID do commit associado ao arquivo.
    pub commit_id: Option<String>,
    /// ID do último commit que modificou o arquivo.
    pub last_commit_id: Option<String>,
    /// Indica se o arquivo tem permissão de execução.
    pub execute_filemode: Option<bool>,
}

/// Payload para criar um arquivo na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateFilePayload {
    /// Nome da branch onde o arquivo será criado.
    pub branch: String,
    /// Conteúdo do arquivo.
    pub content: String,
    /// Mensagem do commit.
    pub commit_message: String,
    /// Codificação do conteúdo ("text" ou "base64").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    /// E-mail do autor do commit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_email: Option<String>,
    /// Nome do autor do commit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_name: Option<String>,
}

/// Payload para atualizar um arquivo na API GitLab.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateFilePayload {
    /// Nome da branch onde o arquivo será atualizado.
    pub branch: String,
    /// Novo conteúdo do arquivo.
    pub content: String,
    /// Mensagem do commit.
    pub commit_message: String,
    /// Codificação do conteúdo ("text" ou "base64").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    /// E-mail do autor do commit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_email: Option<String>,
    /// Nome do autor do commit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_name: Option<String>,
    /// ID do último commit (usado para evitar conflitos).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_commit_id: Option<String>,
}
