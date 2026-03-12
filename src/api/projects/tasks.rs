use super::ProjectsApi;
use crate::error::XeroError;
use crate::models::projects::task;
use reqwest::Method;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct TasksResource<'a> {
    api: &'a ProjectsApi,
}

impl<'a> TasksResource<'a> {
    pub(crate) fn new(api: &'a ProjectsApi) -> Self {
        Self { api }
    }

    pub fn list(&self, project_id: Uuid) -> TasksListRequest<'a> {
        TasksListRequest::new(self.api, project_id)
    }

    pub async fn get(&self, project_id: Uuid, task_id: Uuid) -> Result<task::Task, XeroError> {
        let path = format!("/projects/{project_id}/tasks/{task_id}");
        self.api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await
    }

    pub async fn create(
        &self,
        project_id: Uuid,
        task: task::Task,
    ) -> Result<task::Task, XeroError> {
        let path = format!("/projects/{project_id}/tasks");
        self.api
            .client
            .send_request(Method::POST, &path, None, Some(task))
            .await
    }

    pub async fn update(
        &self,
        project_id: Uuid,
        task_id: Uuid,
        task: task::Task,
    ) -> Result<task::Task, XeroError> {
        let path = format!("/projects/{project_id}/tasks/{task_id}");
        self.api
            .client
            .send_request(Method::PUT, &path, None, Some(task))
            .await
    }

    pub async fn delete(&self, project_id: Uuid, task_id: Uuid) -> Result<(), XeroError> {
        let path = format!("/projects/{project_id}/tasks/{task_id}");
        self.api
            .client
            .send_request_empty_response(Method::DELETE, &path, None::<()>)
            .await
    }
}

#[derive(Debug, Clone)]
pub struct TasksListRequest<'a> {
    api: &'a ProjectsApi,
    project_id: Uuid,
    task_ids: Option<Vec<Uuid>>,
    charge_type: Option<String>,
    page: Option<u32>,
    page_size: Option<u32>,
}

impl<'a> TasksListRequest<'a> {
    fn new(api: &'a ProjectsApi, project_id: Uuid) -> Self {
        Self {
            api,
            project_id,
            task_ids: None,
            charge_type: None,
            page: None,
            page_size: None,
        }
    }

    pub fn task_ids<I>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = Uuid>,
    {
        self.task_ids = Some(ids.into_iter().collect());
        self
    }

    pub fn charge_type(mut self, charge_type: impl Into<String>) -> Self {
        self.charge_type = Some(charge_type.into());
        self
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub async fn send(self) -> Result<Vec<task::Task>, XeroError> {
        let mut query = Vec::new();
        if let Some(task_ids) = self.task_ids {
            let ids = task_ids
                .into_iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(",");
            query.push(("taskIds".to_string(), ids));
        }
        if let Some(charge_type) = self.charge_type {
            query.push(("chargeType".to_string(), charge_type));
        }
        if let Some(page) = self.page {
            query.push(("page".to_string(), page.to_string()));
        }
        if let Some(page_size) = self.page_size {
            query.push(("pageSize".to_string(), page_size.to_string()));
        }
        let path = format!("/projects/{}/tasks", self.project_id);
        let resp: task::TasksResponse = self
            .api
            .client
            .send_request(Method::GET, &path, Some(&query), None::<()>)
            .await?;
        Ok(resp.items.unwrap_or_default())
    }
}
