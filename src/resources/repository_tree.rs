use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use crate::utils::encoding::filter_to_query;
use std::sync::Arc;

/// Recurso de API para operações com árvore de repositório no GitLab.
#[derive(Debug)]
pub struct RepositoryTreeResource {
    http: Arc<HttpClient>,
}

impl RepositoryTreeResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista entradas da árvore de repositório com filtros opcionais.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `filter`: Filtros opcionais (path, ref_, recursive, per_page, page).
    ///
    /// ## Returns
    /// `Result<Vec<RepositoryTreeItem>, GitLabError>` — lista de itens da árvore.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list(
        &self,
        project_id: u64,
        filter: Option<&TreeFilter>,
    ) -> Result<Vec<RepositoryTreeItem>, GitLabError> {
        let path = format!("projects/{}/repository/tree", project_id);
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "repository_tree.list").await
    }

    /// Obtém um item específico da árvore pelo caminho e referência.
    ///
    /// ## Params
    /// - `project_id`: ID do projeto no GitLab.
    /// - `file_path`: Caminho do item na árvore.
    /// - `ref_`: Nome da branch, tag ou SHA do commit.
    ///
    /// ## Returns
    /// `Result<RepositoryTreeItem, GitLabError>` — dados do item da árvore.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn get(
        &self,
        project_id: u64,
        file_path: &str,
        ref_: &str,
    ) -> Result<RepositoryTreeItem, GitLabError> {
        let url = format!("projects/{}/repository/tree", project_id);
        let query = vec![("path".into(), file_path.into()), ("ref".into(), ref_.into())];
        self.http.get(&url, &query, "repository_tree.get").await
    }
}
