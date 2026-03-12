use super::query::QueryParams;
use super::AccountingApi;
use crate::error::XeroError;
use crate::models::accounting::quote;
use reqwest::Method;
use uuid::Uuid;

/// Resource accessor for Quotes.
#[derive(Debug, Clone, Copy)]
pub struct QuotesResource<'a> {
    api: &'a AccountingApi,
}

impl<'a> QuotesResource<'a> {
    pub(crate) fn new(api: &'a AccountingApi) -> Self {
        Self { api }
    }

    /// Builds a request to list quotes.
    pub fn list(&self) -> QuotesListRequest<'a> {
        QuotesListRequest::new(self.api)
    }

    /// Retrieves a quote by ID.
    pub async fn get(&self, quote_id: Uuid) -> Result<Vec<quote::Quote>, XeroError> {
        let path = format!("/Quotes/{quote_id}");
        let resp: quote::QuotesResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        Ok(resp.quotes)
    }

    /// Creates or updates one or more quotes.
    pub async fn create_or_update(
        &self,
        quotes: Vec<quote::Quote>,
        summarize_errors: Option<bool>,
    ) -> Result<Vec<quote::Quote>, XeroError> {
        let mut query = QueryParams::default();
        query.push_opt("summarizeErrors", summarize_errors);
        let body = if quotes.len() == 1 {
            serde_json::to_value(quotes.into_iter().next().unwrap())?
        } else {
            serde_json::to_value(quote::QuotesRequest { quotes })?
        };
        let resp: quote::QuotesResponse = self
            .api
            .client
            .send_request(Method::POST, "/Quotes", query.as_slice(), Some(body))
            .await?;
        Ok(resp.quotes)
    }
}

/// Builder for Quotes list requests.
#[derive(Debug, Clone)]
pub struct QuotesListRequest<'a> {
    api: &'a AccountingApi,
    quote_number: Option<String>,
    contact_id: Option<Uuid>,
    status: Option<String>,
    date_from: Option<String>,
    date_to: Option<String>,
    expiry_date_from: Option<String>,
    expiry_date_to: Option<String>,
    order_by: Option<String>,
    page: Option<u32>,
    page_size: Option<u32>,
}

impl<'a> QuotesListRequest<'a> {
    fn new(api: &'a AccountingApi) -> Self {
        Self {
            api,
            quote_number: None,
            contact_id: None,
            status: None,
            date_from: None,
            date_to: None,
            expiry_date_from: None,
            expiry_date_to: None,
            order_by: None,
            page: None,
            page_size: None,
        }
    }

    /// Filter by quote number.
    pub fn quote_number(mut self, quote_number: impl Into<String>) -> Self {
        self.quote_number = Some(quote_number.into());
        self
    }

    /// Filter by contact ID.
    pub fn contact_id(mut self, contact_id: Uuid) -> Self {
        self.contact_id = Some(contact_id);
        self
    }

    /// Filter by status.
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    /// Filter by start date.
    pub fn date_from(mut self, date_from: impl Into<String>) -> Self {
        self.date_from = Some(date_from.into());
        self
    }

    /// Filter by end date.
    pub fn date_to(mut self, date_to: impl Into<String>) -> Self {
        self.date_to = Some(date_to.into());
        self
    }

    /// Filter by expiry date start.
    pub fn expiry_date_from(mut self, expiry_date_from: impl Into<String>) -> Self {
        self.expiry_date_from = Some(expiry_date_from.into());
        self
    }

    /// Filter by expiry date end.
    pub fn expiry_date_to(mut self, expiry_date_to: impl Into<String>) -> Self {
        self.expiry_date_to = Some(expiry_date_to.into());
        self
    }

    /// Order by a field.
    pub fn order_by(mut self, order_by: impl Into<String>) -> Self {
        self.order_by = Some(order_by.into());
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
    pub async fn send(self) -> Result<Vec<quote::Quote>, XeroError> {
        let mut query = QueryParams::default();
        query.push_opt_string("QuoteNumber", self.quote_number);
        query.push_opt("ContactID", self.contact_id);
        query.push_opt_string("Status", self.status);
        query.push_opt_string("DateFrom", self.date_from);
        query.push_opt_string("DateTo", self.date_to);
        query.push_opt_string("ExpiryDateFrom", self.expiry_date_from);
        query.push_opt_string("ExpiryDateTo", self.expiry_date_to);
        query.push_opt_string("order", self.order_by);
        query.push_opt("page", self.page);
        query.push_opt("pageSize", self.page_size);

        let resp: quote::QuotesResponse = self
            .api
            .client
            .send_request(Method::GET, "/Quotes", query.as_slice(), None::<()>)
            .await?;
        Ok(resp.quotes)
    }
}

impl AccountingApi {
    /// Retrieves one or many quotes.
    #[allow(clippy::too_many_arguments)]
    pub async fn get_quotes(
        &self,
        quote_id: Option<Uuid>,
        quote_number: Option<String>,
        contact_id: Option<Uuid>,
        status: Option<String>,
        date_from: Option<String>,
        date_to: Option<String>,
        expiry_date_from: Option<String>,
        expiry_date_to: Option<String>,
        order_by: Option<String>,
        page: Option<u32>,
        page_size: Option<u32>,
    ) -> Result<Vec<quote::Quote>, XeroError> {
        if let Some(id) = quote_id {
            self.quotes().get(id).await
        } else {
            let mut request = self.quotes().list();
            if let Some(quote_number) = quote_number {
                request = request.quote_number(quote_number);
            }
            if let Some(contact_id) = contact_id {
                request = request.contact_id(contact_id);
            }
            if let Some(status) = status {
                request = request.status(status);
            }
            if let Some(date_from) = date_from {
                request = request.date_from(date_from);
            }
            if let Some(date_to) = date_to {
                request = request.date_to(date_to);
            }
            if let Some(expiry_date_from) = expiry_date_from {
                request = request.expiry_date_from(expiry_date_from);
            }
            if let Some(expiry_date_to) = expiry_date_to {
                request = request.expiry_date_to(expiry_date_to);
            }
            if let Some(order_by) = order_by {
                request = request.order_by(order_by);
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

    /// Creates or updates one or more quotes.
    pub async fn create_or_update_quotes(
        &self,
        quotes: Vec<quote::Quote>,
        summarize_errors: Option<bool>,
    ) -> Result<Vec<quote::Quote>, XeroError> {
        self.quotes()
            .create_or_update(quotes, summarize_errors)
            .await
    }
}
