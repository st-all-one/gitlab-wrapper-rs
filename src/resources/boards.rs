use crate::core::errors::GitLabError;
use crate::http::client::HttpClient;
use crate::types::*;
use std::sync::Arc;

/// Recurso de API para issue boards no GitLab (projeto + grupo).
#[derive(Debug)]
pub struct BoardsResource {
    http: Arc<HttpClient>,
}

impl BoardsResource {
    pub(crate) fn new(http: Arc<HttpClient>) -> Self {
        Self { http }
    }

    // ── Project Boards ──

    /// Executa a operação .
    /// Executa a operacao `list_project_boards`.
    pub async fn list_project_boards(&self, project_id: u64) -> Result<Vec<Board>, GitLabError> {
        let path = format!("projects/{}/boards", project_id);
        self.http.get(&path, &[], "boards.list_project").await
    }

    /// Executa a operação .
    /// Executa a operacao `get_project_board`.
    pub async fn get_project_board(
        &self,
        project_id: u64,
        board_id: u64,
    ) -> Result<Board, GitLabError> {
        let path = format!("projects/{}/boards/{}", project_id, board_id);
        self.http.get(&path, &[], "boards.get_project").await
    }

    /// Executa a operação .
    /// Executa a operacao `list_project_board_lists`.
    pub async fn list_project_board_lists(
        &self,
        project_id: u64,
        board_id: u64,
    ) -> Result<Vec<BoardList>, GitLabError> {
        let path = format!("projects/{}/boards/{}/lists", project_id, board_id);
        self.http.get(&path, &[], "boards.list_lists").await
    }

    /// Executa a operação .
    /// Executa a operacao `get_project_board_list`.
    pub async fn get_project_board_list(
        &self,
        project_id: u64,
        board_id: u64,
        list_id: u64,
    ) -> Result<BoardList, GitLabError> {
        let path = format!("projects/{}/boards/{}/lists/{}", project_id, board_id, list_id);
        self.http.get(&path, &[], "boards.get_list").await
    }

    /// Executa a operação .
    /// Executa a operacao `create_project_board_list`.
    pub async fn create_project_board_list(
        &self,
        project_id: u64,
        board_id: u64,
        payload: &CreateBoardListPayload,
    ) -> Result<BoardList, GitLabError> {
        let path = format!("projects/{}/boards/{}/lists", project_id, board_id);
        self.http.post(&path, payload, "boards.create_list").await
    }

    /// Executa a operação .
    /// Executa a operacao `update_project_board_list`.
    pub async fn update_project_board_list(
        &self,
        project_id: u64,
        board_id: u64,
        list_id: u64,
        payload: &UpdateBoardListPayload,
    ) -> Result<BoardList, GitLabError> {
        let path = format!("projects/{}/boards/{}/lists/{}", project_id, board_id, list_id);
        self.http.put(&path, payload, "boards.update_list").await
    }

    /// Executa a operação .
    /// Executa a operacao `delete_project_board_list`.
    pub async fn delete_project_board_list(
        &self,
        project_id: u64,
        board_id: u64,
        list_id: u64,
    ) -> Result<(), GitLabError> {
        let path = format!("projects/{}/boards/{}/lists/{}", project_id, board_id, list_id);
        self.http.delete(&path, &[], "boards.delete_list").await
    }

    // ── Group Boards ──

    /// Executa a operação .
    /// Executa a operacao `list_group_boards`.
    pub async fn list_group_boards(&self, group_id: u64) -> Result<Vec<Board>, GitLabError> {
        let path = format!("groups/{}/boards", group_id);
        self.http.get(&path, &[], "boards.list_group").await
    }

    /// Executa a operação .
    /// Executa a operacao `get_group_board`.
    pub async fn get_group_board(
        &self,
        group_id: u64,
        board_id: u64,
    ) -> Result<Board, GitLabError> {
        let path = format!("groups/{}/boards/{}", group_id, board_id);
        self.http.get(&path, &[], "boards.get_group").await
    }
}
