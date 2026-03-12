//! Tasks resource for the XPM Practice Manager API v3.1.
//!
//! These are global task definitions (not job-specific tasks).

use super::PracticeManagerApi;
use crate::error::XeroError;
use crate::models::practice_manager::task::{TaskListResponse, TaskResponse};
use reqwest::Method;
use uuid::Uuid;

/// Resource accessor for XPM Task definitions.
#[derive(Debug, Clone, Copy)]
pub struct TasksResource<'a> {
    api: &'a PracticeManagerApi,
}

impl<'a> TasksResource<'a> {
    pub(crate) fn new(api: &'a PracticeManagerApi) -> Self {
        Self { api }
    }

    /// Returns a list of all task definitions.
    pub async fn list(&self) -> Result<TaskListResponse, XeroError> {
        self.api
            .client
            .send_request_xml(Method::GET, "/task.api/list", None)
            .await
    }

    /// Retrieves detailed information for a specific task definition.
    pub async fn get(&self, uuid: Uuid) -> Result<TaskResponse, XeroError> {
        let path = format!("/task.api/get/{uuid}");
        self.api
            .client
            .send_request_xml(Method::GET, &path, None)
            .await
    }
}
