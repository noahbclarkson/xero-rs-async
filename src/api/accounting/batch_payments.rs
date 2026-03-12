use super::query::QueryParams;
use super::AccountingApi;
use crate::error::XeroError;
use crate::models::accounting::batch_payment;
use chrono::{DateTime, Utc};
use reqwest::Method;
use serde::Serialize;
use uuid::Uuid;

/// Resource accessor for Batch Payments.
#[derive(Debug, Clone, Copy)]
pub struct BatchPaymentsResource<'a> {
    api: &'a AccountingApi,
}

impl<'a> BatchPaymentsResource<'a> {
    pub(crate) fn new(api: &'a AccountingApi) -> Self {
        Self { api }
    }

    /// Builds a request to list batch payments.
    pub fn list(&self) -> BatchPaymentsListRequest<'a> {
        BatchPaymentsListRequest::new(self.api)
    }

    /// Retrieves a batch payment by ID.
    pub async fn get(
        &self,
        batch_payment_id: Uuid,
    ) -> Result<Vec<batch_payment::BatchPayment>, XeroError> {
        let path = format!("/BatchPayments/{batch_payment_id}");
        let resp: batch_payment::BatchPaymentsResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        Ok(resp.batch_payments)
    }

    /// Creates a new batch payment.
    pub async fn create(
        &self,
        batch_payment: batch_payment::BatchPayment,
    ) -> Result<Vec<batch_payment::BatchPayment>, XeroError> {
        let resp: batch_payment::BatchPaymentsResponse = self
            .api
            .client
            .send_request(Method::PUT, "/BatchPayments", None, Some(batch_payment))
            .await?;
        Ok(resp.batch_payments)
    }

    /// Updates a batch payment status to DELETED.
    pub async fn delete(
        &self,
        batch_payment_id: Uuid,
    ) -> Result<Vec<batch_payment::BatchPayment>, XeroError> {
        #[derive(Serialize)]
        #[serde(rename_all = "PascalCase")]
        struct DeleteRequest {
            status: batch_payment::BatchPaymentStatus,
        }
        let path = format!("/BatchPayments/{batch_payment_id}");
        let body = DeleteRequest {
            status: batch_payment::BatchPaymentStatus::Deleted,
        };
        let resp: batch_payment::BatchPaymentsResponse = self
            .api
            .client
            .send_request(Method::POST, &path, None, Some(body))
            .await?;
        Ok(resp.batch_payments)
    }
}

/// Builder for Batch Payments list requests.
#[derive(Debug, Clone)]
pub struct BatchPaymentsListRequest<'a> {
    api: &'a AccountingApi,
    where_filter: Option<String>,
    order_by: Option<String>,
}

impl<'a> BatchPaymentsListRequest<'a> {
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
    pub async fn send(self) -> Result<Vec<batch_payment::BatchPayment>, XeroError> {
        let mut query = QueryParams::default();
        query.push_opt_string("where", self.where_filter);
        query.push_opt_string("order", self.order_by);

        let resp: batch_payment::BatchPaymentsResponse = self
            .api
            .client
            .send_request(Method::GET, "/BatchPayments", query.as_slice(), None::<()>)
            .await?;
        Ok(resp.batch_payments)
    }
}

impl AccountingApi {
    /// Retrieves one or many batch payments.
    pub async fn get_batch_payments(
        &self,
        batch_payment_id: Option<Uuid>,
        _modified_after: Option<DateTime<Utc>>,
        where_filter: Option<String>,
        order_by: Option<String>,
    ) -> Result<Vec<batch_payment::BatchPayment>, XeroError> {
        if let Some(id) = batch_payment_id {
            self.batch_payments().get(id).await
        } else {
            let mut request = self.batch_payments().list();
            if let Some(filter) = where_filter {
                request = request.where_filter(filter);
            }
            if let Some(order) = order_by {
                request = request.order_by(order);
            }
            request.send().await
        }
    }

    /// Creates a new batch payment.
    pub async fn create_batch_payment(
        &self,
        batch_payment: batch_payment::BatchPayment,
    ) -> Result<Vec<batch_payment::BatchPayment>, XeroError> {
        self.batch_payments().create(batch_payment).await
    }

    /// Updates a batch payment status to DELETED.
    pub async fn delete_batch_payment(
        &self,
        batch_payment_id: Uuid,
    ) -> Result<Vec<batch_payment::BatchPayment>, XeroError> {
        self.batch_payments().delete(batch_payment_id).await
    }
}
