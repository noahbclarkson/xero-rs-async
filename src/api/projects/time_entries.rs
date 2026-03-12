use super::ProjectsApi;
use crate::error::XeroError;
use crate::models::projects::time_entry;
use reqwest::Method;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct TimeEntriesResource<'a> {
    api: &'a ProjectsApi,
}

impl<'a> TimeEntriesResource<'a> {
    pub(crate) fn new(api: &'a ProjectsApi) -> Self {
        Self { api }
    }

    pub fn list(&self, project_id: Uuid) -> TimeEntriesListRequest<'a> {
        TimeEntriesListRequest::new(self.api, project_id)
    }

    pub async fn get(
        &self,
        project_id: Uuid,
        time_entry_id: Uuid,
    ) -> Result<time_entry::TimeEntry, XeroError> {
        let path = format!("/projects/{project_id}/time/{time_entry_id}");
        self.api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await
    }

    pub async fn create(
        &self,
        project_id: Uuid,
        entry: time_entry::TimeEntry,
    ) -> Result<time_entry::TimeEntry, XeroError> {
        let path = format!("/projects/{project_id}/time");
        self.api
            .client
            .send_request(Method::POST, &path, None, Some(entry))
            .await
    }

    pub async fn update(
        &self,
        project_id: Uuid,
        time_entry_id: Uuid,
        entry: time_entry::TimeEntry,
    ) -> Result<time_entry::TimeEntry, XeroError> {
        let path = format!("/projects/{project_id}/time/{time_entry_id}");
        self.api
            .client
            .send_request(Method::PUT, &path, None, Some(entry))
            .await
    }

    pub async fn delete(&self, project_id: Uuid, time_entry_id: Uuid) -> Result<(), XeroError> {
        let path = format!("/projects/{project_id}/time/{time_entry_id}");
        self.api
            .client
            .send_request_empty_response(Method::DELETE, &path, None::<()>)
            .await
    }
}

#[derive(Debug, Clone)]
pub struct TimeEntriesListRequest<'a> {
    api: &'a ProjectsApi,
    project_id: Uuid,
    user_id: Option<Uuid>,
    task_id: Option<Uuid>,
    date_after_utc: Option<String>,
    date_before_utc: Option<String>,
    is_chargeable: Option<bool>,
    invoice_id: Option<Uuid>,
    contact_id: Option<Uuid>,
    states: Option<Vec<String>>,
    page: Option<u32>,
    page_size: Option<u32>,
}

impl<'a> TimeEntriesListRequest<'a> {
    fn new(api: &'a ProjectsApi, project_id: Uuid) -> Self {
        Self {
            api,
            project_id,
            user_id: None,
            task_id: None,
            date_after_utc: None,
            date_before_utc: None,
            is_chargeable: None,
            invoice_id: None,
            contact_id: None,
            states: None,
            page: None,
            page_size: None,
        }
    }

    pub fn user_id(mut self, user_id: Uuid) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn task_id(mut self, task_id: Uuid) -> Self {
        self.task_id = Some(task_id);
        self
    }

    pub fn date_after_utc(mut self, date: impl Into<String>) -> Self {
        self.date_after_utc = Some(date.into());
        self
    }

    pub fn date_before_utc(mut self, date: impl Into<String>) -> Self {
        self.date_before_utc = Some(date.into());
        self
    }

    pub fn is_chargeable(mut self, is_chargeable: bool) -> Self {
        self.is_chargeable = Some(is_chargeable);
        self
    }

    pub fn invoice_id(mut self, invoice_id: Uuid) -> Self {
        self.invoice_id = Some(invoice_id);
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

    pub async fn send(self) -> Result<Vec<time_entry::TimeEntry>, XeroError> {
        let mut query = Vec::new();
        if let Some(user_id) = self.user_id {
            query.push(("userId".to_string(), user_id.to_string()));
        }
        if let Some(task_id) = self.task_id {
            query.push(("taskId".to_string(), task_id.to_string()));
        }
        if let Some(date_after_utc) = self.date_after_utc {
            query.push(("dateAfterUtc".to_string(), date_after_utc));
        }
        if let Some(date_before_utc) = self.date_before_utc {
            query.push(("dateBeforeUtc".to_string(), date_before_utc));
        }
        if let Some(is_chargeable) = self.is_chargeable {
            query.push(("isChargeable".to_string(), is_chargeable.to_string()));
        }
        if let Some(invoice_id) = self.invoice_id {
            query.push(("invoiceId".to_string(), invoice_id.to_string()));
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
        let path = format!("/projects/{}/time", self.project_id);
        let resp: time_entry::TimeEntriesResponse = self
            .api
            .client
            .send_request(Method::GET, &path, Some(&query), None::<()>)
            .await?;
        Ok(resp.items.unwrap_or_default())
    }
}
