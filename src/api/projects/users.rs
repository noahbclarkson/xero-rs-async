use super::ProjectsApi;
use crate::error::XeroError;
use crate::models::projects::user;
use reqwest::Method;

#[derive(Debug, Clone, Copy)]
pub struct UsersResource<'a> {
    api: &'a ProjectsApi,
}

impl<'a> UsersResource<'a> {
    pub(crate) fn new(api: &'a ProjectsApi) -> Self {
        Self { api }
    }

    pub fn list(&self) -> UsersListRequest<'a> {
        UsersListRequest::new(self.api)
    }
}

#[derive(Debug, Clone)]
pub struct UsersListRequest<'a> {
    api: &'a ProjectsApi,
    page: Option<u32>,
    page_size: Option<u32>,
}

impl<'a> UsersListRequest<'a> {
    fn new(api: &'a ProjectsApi) -> Self {
        Self {
            api,
            page: None,
            page_size: None,
        }
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub async fn send(self) -> Result<Vec<user::ProjectUser>, XeroError> {
        let mut query = Vec::new();
        if let Some(page) = self.page {
            query.push(("page".to_string(), page.to_string()));
        }
        if let Some(page_size) = self.page_size {
            query.push(("pageSize".to_string(), page_size.to_string()));
        }
        let resp: user::ProjectUsersResponse = self
            .api
            .client
            .send_request(Method::GET, "/projectsusers", Some(&query), None::<()>)
            .await?;
        Ok(resp.items.unwrap_or_default())
    }
}
