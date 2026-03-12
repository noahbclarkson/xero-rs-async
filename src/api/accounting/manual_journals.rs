use super::query::QueryParams;
use super::AccountingApi;
use crate::error::XeroError;
use crate::models::accounting::{attachment, manual_journal};
use chrono::{DateTime, Utc};
use reqwest::Method;
use uuid::Uuid;

/// Resource accessor for Manual Journals.
#[derive(Debug, Clone, Copy)]
pub struct ManualJournalsResource<'a> {
    api: &'a AccountingApi,
}

impl<'a> ManualJournalsResource<'a> {
    pub(crate) fn new(api: &'a AccountingApi) -> Self {
        Self { api }
    }

    /// Builds a request to list manual journals.
    pub fn list(&self) -> ManualJournalsListRequest<'a> {
        ManualJournalsListRequest::new(self.api)
    }

    /// Retrieves a manual journal by ID.
    pub async fn get(
        &self,
        manual_journal_id: Uuid,
    ) -> Result<Vec<manual_journal::ManualJournal>, XeroError> {
        let path = format!("/ManualJournals/{manual_journal_id}");
        let resp: manual_journal::ManualJournalsResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        Ok(resp.manual_journals)
    }

    /// Creates or updates a manual journal.
    pub async fn create_or_update(
        &self,
        journal: manual_journal::ManualJournal,
    ) -> Result<Vec<manual_journal::ManualJournal>, XeroError> {
        let path = if let Some(id) = journal.manual_journal_id {
            format!("/ManualJournals/{id}")
        } else {
            "/ManualJournals".to_string()
        };
        let method = if journal.manual_journal_id.is_some() {
            Method::POST
        } else {
            Method::PUT
        };
        let resp: manual_journal::ManualJournalsResponse = self
            .api
            .client
            .send_request(method, &path, None, Some(journal))
            .await?;
        Ok(resp.manual_journals)
    }

    /// Attaches a file to a Manual Journal.
    pub async fn attach_by_file_name(
        &self,
        manual_journal_id: Uuid,
        file_name: String,
        body: Vec<u8>,
    ) -> Result<Vec<attachment::Attachment>, XeroError> {
        let encoded_file_name = file_name.replace('[', "%5B").replace(']', "%5D");
        let path = format!("/ManualJournals/{manual_journal_id}/Attachments/{encoded_file_name}");
        let content_type = "application/octet-stream";
        let resp: attachment::AttachmentsResponse = self
            .api
            .client
            .send_request_raw_body(Method::PUT, &path, content_type, body)
            .await?;
        Ok(resp.attachments)
    }
}

/// Builder for Manual Journals list requests.
#[derive(Debug, Clone)]
pub struct ManualJournalsListRequest<'a> {
    api: &'a AccountingApi,
    where_filter: Option<String>,
    order_by: Option<String>,
    page: Option<u32>,
    page_size: Option<u32>,
}

impl<'a> ManualJournalsListRequest<'a> {
    fn new(api: &'a AccountingApi) -> Self {
        Self {
            api,
            where_filter: None,
            order_by: None,
            page: None,
            page_size: None,
        }
    }

    /// Filter using the `where` query parameter.
    pub fn where_filter(mut self, filter: impl Into<String>) -> Self {
        self.where_filter = Some(filter.into());
        self
    }

    /// Order by a field.
    pub fn order_by(mut self, order: impl Into<String>) -> Self {
        self.order_by = Some(order.into());
        self
    }

    /// Sets the page number.
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    /// Sets the page size.
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// Executes the list request.
    pub async fn send(self) -> Result<Vec<manual_journal::ManualJournal>, XeroError> {
        let mut query = QueryParams::default();
        query.push_opt_string("where", self.where_filter);
        query.push_opt_string("order", self.order_by);
        query.push_opt("page", self.page);
        query.push_opt("pageSize", self.page_size);

        let resp: manual_journal::ManualJournalsResponse = self
            .api
            .client
            .send_request(Method::GET, "/ManualJournals", query.as_slice(), None::<()>)
            .await?;
        Ok(resp.manual_journals)
    }
}

impl AccountingApi {
    /// Retrieves one or many manual journals.
    #[allow(clippy::too_many_arguments)]
    pub async fn get_manual_journals(
        &self,
        manual_journal_id: Option<Uuid>,
        _modified_after: Option<DateTime<Utc>>,
        where_filter: Option<String>,
        order_by: Option<String>,
        page: Option<u32>,
        page_size: Option<u32>,
    ) -> Result<Vec<manual_journal::ManualJournal>, XeroError> {
        if let Some(id) = manual_journal_id {
            self.manual_journals().get(id).await
        } else {
            let mut request = self.manual_journals().list();
            if let Some(filter) = where_filter {
                request = request.where_filter(filter);
            }
            if let Some(order) = order_by {
                request = request.order_by(order);
            }
            if let Some(page) = page {
                request = request.page(page);
            }
            if let Some(page_size) = page_size {
                request = request.page_size(page_size);
            }
            request.send().await
        }
    }

    /// Creates or updates a manual journal.
    pub async fn create_or_update_manual_journal(
        &self,
        journal: manual_journal::ManualJournal,
    ) -> Result<Vec<manual_journal::ManualJournal>, XeroError> {
        self.manual_journals().create_or_update(journal).await
    }

    /// Attaches a file to a Manual Journal.
    pub async fn create_manual_journal_attachment(
        &self,
        manual_journal_id: Uuid,
        file_name: String,
        body: Vec<u8>,
    ) -> Result<Vec<attachment::Attachment>, XeroError> {
        self.manual_journals()
            .attach_by_file_name(manual_journal_id, file_name, body)
            .await
    }
}
