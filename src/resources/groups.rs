use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use std::sync::Arc;
use crate::types::*;
use crate::utils::encoding::filter_to_query;

/// Recurso de API para operações com grupos no GitLab.
#[derive(Debug)]
pub struct GroupsResource {
    http: Arc<HttpClient>,
}

impl GroupsResource {
    /// Cria uma nova instância do recurso.
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    /// Lista grupos com filtros opcionais.
    ///
    /// ## Params
    /// - `filter`: Filtros opcionais para a consulta.
    ///
    /// ## Returns
    /// `Result<Vec<Group>, GitLabError>` — lista de grupos.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn list(&self, filter: Option<&GroupFilter>) -> Result<Vec<Group>, GitLabError> {
        let query = filter_to_query(filter);
        self.http.get("groups", &query, "groups.list")
    }

    /// Obtém um grupo pelo ID.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    ///
    /// ## Returns
    /// `Result<Group, GitLabError>` — dados do grupo solicitado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn get(&self, group_id: u64) -> Result<Group, GitLabError> {
        let path = format!("groups/{}", group_id);
        self.http.get(&path, &[], "groups.get")
    }

    /// Obtém um grupo pelo caminho URL-encoded.
    ///
    /// ## Params
    /// - `path`: Caminho do grupo (ex: "subgrupo/grupo").
    ///
    /// ## Returns
    /// `Result<Group, GitLabError>` — dados do grupo solicitado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn get_by_path(&self, path: &str) -> Result<Group, GitLabError> {
        let encoded = crate::utils::encoding::encode_query_param(path);
        let url = format!("groups/{}", encoded);
        self.http.get(&url, &[], "groups.get_by_path")
    }

    /// Cria um novo grupo.
    ///
    /// ## Params
    /// - `payload`: Dados para criar o grupo.
    ///
    /// ## Returns
    /// `Result<Group, GitLabError>` — dados do grupo criado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn create(&self, payload: &CreateGroupPayload) -> Result<Group, GitLabError> {
        self.http.post("groups", &payload, "groups.create")
    }

    /// Atualiza um grupo existente.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    /// - `payload`: Dados para atualizar o grupo.
    ///
    /// ## Returns
    /// `Result<Group, GitLabError>` — dados do grupo atualizado.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn update(&self, group_id: u64, payload: &UpdateGroupPayload) -> Result<Group, GitLabError> {
        let path = format!("groups/{}", group_id);
        self.http.put(&path, &payload, "groups.update")
    }

    /// Remove um grupo.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    ///
    /// ## Returns
    /// `Result<(), GitLabError>` — vazio em caso de sucesso.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn delete(&self, group_id: u64) -> Result<(), GitLabError> {
        let path = format!("groups/{}", group_id);
        self.http.delete(&path, &[], "groups.delete")
    }

    /// Lista subgrupos de um grupo.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<Group>, GitLabError>` — lista de subgrupos.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn subgroups(&self, group_id: u64) -> Result<Vec<Group>, GitLabError> {
        let path = format!("groups/{}/subgroups", group_id);
        self.http.get(&path, &[], "groups.subgroups")
    }

    /// Lista grupos descendentes de um grupo.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<Group>, GitLabError>` — lista de grupos descendentes.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn descendant_groups(&self, group_id: u64) -> Result<Vec<Group>, GitLabError> {
        let path = format!("groups/{}/descendant_groups", group_id);
        self.http.get(&path, &[], "groups.descendant_groups")
    }

    /// Lista projetos de um grupo.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<Project>, GitLabError>` — lista de projetos do grupo.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub fn projects(&self, group_id: u64) -> Result<Vec<Project>, GitLabError> {
        let path = format!("groups/{}/projects", group_id);
        self.http.get(&path, &[], "groups.projects")
    }
}
