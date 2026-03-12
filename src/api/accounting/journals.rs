use super::query::QueryParams;
use super::AccountingApi;
use crate::error::XeroError;
use crate::models::accounting::journal;
use chrono::{DateTime, Utc};
use reqwest::Method;

/// Resource accessor for Journals.
#[derive(Debug, Clone, Copy)]
pub struct JournalsResource<'a> {
    api: &'a AccountingApi,
}

impl<'a> JournalsResource<'a> {
    pub(crate) fn new(api: &'a AccountingApi) -> Self {
        Self { api }
    }

    /// Builds a request to list journals.
    pub fn list(&self) -> JournalsListRequest<'a> {
        JournalsListRequest::new(self.api)
    }
}

/// Builder for Journals list requests.
#[derive(Debug, Clone)]
pub struct JournalsListRequest<'a> {
    api: &'a AccountingApi,
    offset: Option<u32>,
    payments_only: Option<bool>,
    if_modified_since: Option<DateTime<Utc>>,
}

impl<'a> JournalsListRequest<'a> {
    fn new(api: &'a AccountingApi) -> Self {
        Self {
            api,
            offset: None,
            payments_only: None,
            if_modified_since: None,
        }
    }

    /// Sets the offset (journal number to start from).
    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Restrict to payment journals only.
    pub fn payments_only(mut self, payments_only: bool) -> Self {
        self.payments_only = Some(payments_only);
        self
    }

    /// Only return journals created or modified after this UTC datetime.
    ///
    /// Maps to the HTTP `If-Modified-Since` header. Useful for fetching
    /// only recent journals instead of the entire history.
    pub fn if_modified_since(mut self, since: DateTime<Utc>) -> Self {
        self.if_modified_since = Some(since);
        self
    }

    /// Executes the list request.
    pub async fn send(self) -> Result<Vec<journal::Journal>, XeroError> {
        let mut query = QueryParams::default();
        query.push_opt("offset", self.offset);
        query.push_opt("paymentsOnly", self.payments_only);

        if let Some(since) = &self.if_modified_since {
            let resp: journal::JournalsResponse = self
                .api
                .client
                .send_request_modified_since(Method::GET, "/Journals", query.as_slice(), since)
                .await?;
            Ok(resp.journals)
        } else {
            let resp: journal::JournalsResponse = self
                .api
                .client
                .send_request(Method::GET, "/Journals", query.as_slice(), None::<()>)
                .await?;
            Ok(resp.journals)
        }
    }
}

impl AccountingApi {
    /// Retrieves journals.
    pub async fn get_journals(
        &self,
        offset: Option<u32>,
        payments_only: Option<bool>,
    ) -> Result<Vec<journal::Journal>, XeroError> {
        let mut request = self.journals().list();
        if let Some(offset) = offset {
            request = request.offset(offset);
        }
        if let Some(payments_only) = payments_only {
            request = request.payments_only(payments_only);
        }
        request.send().await
    }
}
