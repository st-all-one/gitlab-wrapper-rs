use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use crate::utils::encoding::filter_to_query;
use std::sync::Arc;

/// Recurso de API para operações com iterações de grupo no GitLab.
#[derive(Debug)]
pub struct GroupIterationsResource {
    http: Arc<HttpClient>,
}

impl GroupIterationsResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista todas as iterações de um grupo com filtros opcionais.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    /// - `filter`: Filtros opcionais para a consulta.
    ///
    /// ## Returns
    /// `Result<Vec<GroupIteration>, GitLabError>` — lista de iterações do grupo.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn list(
        &self,
        group_id: u64,
        filter: Option<&GroupIterationFilter>,
    ) -> Result<Vec<GroupIteration>, GitLabError> {
        let path = format!("groups/{}/iterations", group_id);
        let query = filter_to_query(filter);
        self.http.get(&path, &query, "group_iterations.list").await
    }
}
