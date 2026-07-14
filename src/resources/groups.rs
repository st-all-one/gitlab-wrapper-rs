use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use crate::utils::encoding::encode_query_param;
use crate::utils::encoding::filter_to_query;
use std::sync::Arc;

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
    pub async fn list(&self, filter: Option<&GroupFilter>) -> Result<Vec<Group>, GitLabError> {
        let query = filter_to_query(filter);
        self.http.get("groups", &query, "groups.list").await
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
    pub async fn get(&self, group_id: u64) -> Result<Group, GitLabError> {
        let path = format!("groups/{}", group_id);
        self.http.get(&path, &[], "groups.get").await
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
    pub async fn get_by_path(&self, path: &str) -> Result<Group, GitLabError> {
        let encoded = encode_query_param(path);
        let url = format!("groups/{}", encoded);
        self.http.get(&url, &[], "groups.get_by_path").await
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
    pub async fn create(&self, payload: &CreateGroupPayload) -> Result<Group, GitLabError> {
        self.http.post("groups", &payload, "groups.create").await
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
    pub async fn update(
        &self,
        group_id: u64,
        payload: &UpdateGroupPayload,
    ) -> Result<Group, GitLabError> {
        let path = format!("groups/{}", group_id);
        self.http.put(&path, &payload, "groups.update").await
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
    pub async fn delete(&self, group_id: u64) -> Result<(), GitLabError> {
        let path = format!("groups/{}", group_id);
        self.http.delete(&path, &[], "groups.delete").await
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
    pub async fn subgroups(&self, group_id: u64) -> Result<Vec<Group>, GitLabError> {
        let path = format!("groups/{}/subgroups", group_id);
        self.http.get(&path, &[], "groups.subgroups").await
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
    pub async fn descendant_groups(&self, group_id: u64) -> Result<Vec<Group>, GitLabError> {
        let path = format!("groups/{}/descendant_groups", group_id);
        self.http.get(&path, &[], "groups.descendant_groups").await
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
    pub async fn projects(&self, group_id: u64) -> Result<Vec<Project>, GitLabError> {
        let path = format!("groups/{}/projects", group_id);
        self.http.get(&path, &[], "groups.projects").await
    }

    /// Lista projetos compartilhados com um grupo.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<Project>, GitLabError>` — lista de projetos compartilhados.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn shared_projects(&self, group_id: u64) -> Result<Vec<Project>, GitLabError> {
        let path = format!("groups/{}/projects/shared", group_id);
        self.http.get(&path, &[], "groups.shared_projects").await
    }

    /// Lista usuários SAML de um grupo.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<User>, GitLabError>` — lista de usuários SAML.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn saml_users(&self, group_id: u64) -> Result<Vec<User>, GitLabError> {
        let path = format!("groups/{}/saml_users", group_id);
        self.http.get(&path, &[], "groups.saml_users").await
    }

    /// Lista usuários provisionados de um grupo.
    ///
    /// ## Params
    /// - `group_id`: ID do grupo no GitLab.
    ///
    /// ## Returns
    /// `Result<Vec<User>, GitLabError>` — lista de usuários provisionados.
    ///
    /// ## Errors
    /// Retorna `GitLabError` em caso de falha de rede, autenticação (401),
    /// permissão (403), recurso não encontrado (404), ou validação (422).
    pub async fn provisioned_users(&self, group_id: u64) -> Result<Vec<User>, GitLabError> {
        let path = format!("groups/{}/provisioned_users", group_id);
        self.http.get(&path, &[], "groups.provisioned_users").await
    }
}
