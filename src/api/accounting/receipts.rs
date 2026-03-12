use super::query::QueryParams;
use super::AccountingApi;
use crate::error::XeroError;
use crate::models::accounting::receipt;
use chrono::{DateTime, Utc};
use reqwest::Method;
use uuid::Uuid;

/// Resource accessor for Receipts.
#[derive(Debug, Clone, Copy)]
pub struct ReceiptsResource<'a> {
    api: &'a AccountingApi,
}

impl<'a> ReceiptsResource<'a> {
    pub(crate) fn new(api: &'a AccountingApi) -> Self {
        Self { api }
    }

    /// Builds a request to list receipts.
    pub fn list(&self) -> ReceiptsListRequest<'a> {
        ReceiptsListRequest::new(self.api)
    }

    /// Retrieves a receipt by ID.
    pub async fn get(&self, receipt_id: Uuid) -> Result<Vec<receipt::Receipt>, XeroError> {
        let path = format!("/Receipts/{receipt_id}");
        let resp: receipt::ReceiptsResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        Ok(resp.receipts)
    }

    /// Creates or updates one or many receipts.
    pub async fn create_or_update(
        &self,
        receipts: Vec<receipt::Receipt>,
        summarize_errors: Option<bool>,
    ) -> Result<Vec<receipt::Receipt>, XeroError> {
        let mut query = QueryParams::default();
        query.push_opt("summarizeErrors", summarize_errors);
        let body = if receipts.len() == 1 {
            serde_json::to_value(receipts.into_iter().next().unwrap())?
        } else {
            serde_json::to_value(receipt::ReceiptsRequest { receipts })?
        };
        let resp: receipt::ReceiptsResponse = self
            .api
            .client
            .send_request(Method::POST, "/Receipts", query.as_slice(), Some(body))
            .await?;
        Ok(resp.receipts)
    }
}

/// Builder for Receipts list requests.
#[derive(Debug, Clone)]
pub struct ReceiptsListRequest<'a> {
    api: &'a AccountingApi,
    where_filter: Option<String>,
    order_by: Option<String>,
}

impl<'a> ReceiptsListRequest<'a> {
    fn new(api: &'a AccountingApi) -> Self {
        Self {
            api,
            where_filter: None,
            order_by: None,
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

    /// Executes the list request.
    pub async fn send(self) -> Result<Vec<receipt::Receipt>, XeroError> {
        let mut query = QueryParams::default();
        query.push_opt_string("where", self.where_filter);
        query.push_opt_string("order", self.order_by);

        let resp: receipt::ReceiptsResponse = self
            .api
            .client
            .send_request(Method::GET, "/Receipts", query.as_slice(), None::<()>)
            .await?;
        Ok(resp.receipts)
    }
}

impl AccountingApi {
    /// Retrieves one or many receipts.
    pub async fn get_receipts(
        &self,
        receipt_id: Option<Uuid>,
        _modified_after: Option<DateTime<Utc>>,
        where_filter: Option<String>,
        order_by: Option<String>,
    ) -> Result<Vec<receipt::Receipt>, XeroError> {
        if let Some(id) = receipt_id {
            self.receipts().get(id).await
        } else {
            let mut request = self.receipts().list();
            if let Some(filter) = where_filter {
                request = request.where_filter(filter);
            }
            if let Some(order) = order_by {
                request = request.order_by(order);
            }
            request.send().await
        }
    }

    /// Creates or updates one or many receipts.
    pub async fn create_or_update_receipts(
        &self,
        receipts: Vec<receipt::Receipt>,
        summarize_errors: Option<bool>,
    ) -> Result<Vec<receipt::Receipt>, XeroError> {
        self.receipts()
            .create_or_update(receipts, summarize_errors)
            .await
    }
}
