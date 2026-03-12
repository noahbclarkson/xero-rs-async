use super::query::QueryParams;
use super::AccountingApi;
use crate::error::XeroError;
use crate::models::accounting::payment;
use chrono::{DateTime, Utc};
use reqwest::Method;
use serde::Serialize;
use uuid::Uuid;

/// Resource accessor for Payments.
#[derive(Debug, Clone, Copy)]
pub struct PaymentsResource<'a> {
    api: &'a AccountingApi,
}

impl<'a> PaymentsResource<'a> {
    pub(crate) fn new(api: &'a AccountingApi) -> Self {
        Self { api }
    }

    /// Builds a request to list payments.
    pub fn list(&self) -> PaymentsListRequest<'a> {
        PaymentsListRequest::new(self.api)
    }

    /// Retrieves a payment by ID.
    pub async fn get(&self, payment_id: Uuid) -> Result<Vec<payment::Payment>, XeroError> {
        let path = format!("/Payments/{payment_id}");
        let resp: payment::PaymentsResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        Ok(resp.payments.unwrap_or_default())
    }

    /// Creates one or more new payments.
    pub async fn create(
        &self,
        payments: Vec<payment::Payment>,
        summarize_errors: Option<bool>,
    ) -> Result<Vec<payment::Payment>, XeroError> {
        let mut query = QueryParams::default();
        query.push_opt("summarizeErrors", summarize_errors);
        let body = if payments.len() == 1 {
            serde_json::to_value(payments.into_iter().next().unwrap())?
        } else {
            serde_json::to_value(payment::PaymentsRequest {
                payments: Some(payments),
            })?
        };
        let resp: payment::PaymentsResponse = self
            .api
            .client
            .send_request(Method::PUT, "/Payments", query.as_slice(), Some(body))
            .await?;
        Ok(resp.payments.unwrap_or_default())
    }

    /// Deletes (reverses) a payment.
    pub async fn delete(&self, payment_id: Uuid) -> Result<Vec<payment::Payment>, XeroError> {
        #[derive(Serialize)]
        #[serde(rename_all = "PascalCase")]
        struct DeleteRequest {
            status: payment::PaymentStatus,
        }
        let path = format!("/Payments/{payment_id}");
        let body = DeleteRequest {
            status: payment::PaymentStatus::Deleted,
        };
        let resp: payment::PaymentsResponse = self
            .api
            .client
            .send_request(Method::POST, &path, None, Some(body))
            .await?;
        Ok(resp.payments.unwrap_or_default())
    }
}

/// Builder for Payments list requests.
#[derive(Debug, Clone)]
pub struct PaymentsListRequest<'a> {
    api: &'a AccountingApi,
    where_filter: Option<String>,
    order_by: Option<String>,
    page: Option<u32>,
    page_size: Option<u32>,
}

impl<'a> PaymentsListRequest<'a> {
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
    pub async fn send(self) -> Result<Vec<payment::Payment>, XeroError> {
        let mut query = QueryParams::default();
        query.push_opt_string("where", self.where_filter);
        query.push_opt_string("order", self.order_by);
        query.push_opt("page", self.page);
        query.push_opt("pageSize", self.page_size);

        let resp: payment::PaymentsResponse = self
            .api
            .client
            .send_request(Method::GET, "/Payments", query.as_slice(), None::<()>)
            .await?;
        Ok(resp.payments.unwrap_or_default())
    }
}

impl AccountingApi {
    /// Retrieves one or many payments.
    #[allow(clippy::too_many_arguments)]
    pub async fn get_payments(
        &self,
        payment_id: Option<Uuid>,
        _modified_after: Option<DateTime<Utc>>,
        where_filter: Option<String>,
        order_by: Option<String>,
        page: Option<u32>,
        page_size: Option<u32>,
    ) -> Result<Vec<payment::Payment>, XeroError> {
        if let Some(id) = payment_id {
            self.payments().get(id).await
        } else {
            let mut request = self.payments().list();
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

    /// Creates one or more new payments.
    pub async fn create_payments(
        &self,
        payments: Vec<payment::Payment>,
        summarize_errors: Option<bool>,
    ) -> Result<Vec<payment::Payment>, XeroError> {
        self.payments().create(payments, summarize_errors).await
    }

    /// Deletes (reverses) a payment.
    pub async fn delete_payment(
        &self,
        payment_id: Uuid,
    ) -> Result<Vec<payment::Payment>, XeroError> {
        self.payments().delete(payment_id).await
    }
}
