use super::ProjectsApi;
use crate::error::XeroError;
use crate::models::projects::project;
use reqwest::Method;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct ProjectsResource<'a> {
    api: &'a ProjectsApi,
}

impl<'a> ProjectsResource<'a> {
    pub(crate) fn new(api: &'a ProjectsApi) -> Self {
        Self { api }
    }

    pub fn list(&self) -> ProjectsListRequest<'a> {
        ProjectsListRequest::new(self.api)
    }

    pub async fn get(&self, project_id: Uuid) -> Result<project::Project, XeroError> {
        let path = format!("/projects/{project_id}");
        self.api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await
    }

    pub async fn create(&self, project: project::Project) -> Result<project::Project, XeroError> {
        self.api
            .client
            .send_request(Method::POST, "/projects", None, Some(project))
            .await
    }

    pub async fn update(
        &self,
        project_id: Uuid,
        project: project::Project,
    ) -> Result<project::Project, XeroError> {
        let path = format!("/projects/{project_id}");
        self.api
            .client
            .send_request(Method::PUT, &path, None, Some(project))
            .await
    }

    pub async fn patch(
        &self,
        project_id: Uuid,
        project: project::Project,
    ) -> Result<project::Project, XeroError> {
        let path = format!("/projects/{project_id}");
        self.api
            .client
            .send_request(Method::PATCH, &path, None, Some(project))
            .await
    }
}

#[derive(Debug, Clone)]
pub struct ProjectsListRequest<'a> {
    api: &'a ProjectsApi,
    project_ids: Option<Vec<Uuid>>,
    contact_id: Option<Uuid>,
    states: Option<Vec<String>>,
    page: Option<u32>,
    page_size: Option<u32>,
}

impl<'a> ProjectsListRequest<'a> {
    fn new(api: &'a ProjectsApi) -> Self {
        Self {
            api,
            project_ids: None,
            contact_id: None,
            states: None,
            page: None,
            page_size: None,
        }
    }

    pub fn project_ids<I>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = Uuid>,
    {
        self.project_ids = Some(ids.into_iter().collect());
        self
    }

    pub fn contact_id(mut self, contact_id: Uuid) -> Self {
        self.contact_id = Some(contact_id);
        self
    }

    pub fn states<I, S>(mut self, states: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.states = Some(states.into_iter().map(Into::into).collect());
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

    pub async fn send(self) -> Result<Vec<project::Project>, XeroError> {
        let mut query = Vec::new();
        if let Some(ids) = self.project_ids {
            let ids = ids
                .into_iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(",");
            query.push(("projectIds".to_string(), ids));
        }
        if let Some(contact_id) = self.contact_id {
            query.push(("contactId".to_string(), contact_id.to_string()));
        }
        if let Some(states) = self.states {
            query.push(("states".to_string(), states.join(",")));
        }
        if let Some(page) = self.page {
            query.push(("page".to_string(), page.to_string()));
        }
        if let Some(page_size) = self.page_size {
            query.push(("pageSize".to_string(), page_size.to_string()));
        }
        let resp: project::ProjectsResponse = self
            .api
            .client
            .send_request(Method::GET, "/projects", Some(&query), None::<()>)
            .await?;
        Ok(resp.items.unwrap_or_default())
    }
}
