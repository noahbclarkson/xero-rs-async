use super::query::QueryParams;
use super::AccountingApi;
use crate::error::XeroError;
use crate::models::accounting::common::Allocation;
use crate::models::accounting::{credit_note, prepayment};
use chrono::{DateTime, Utc};
use reqwest::Method;
use uuid::Uuid;

/// Resource accessor for Prepayments.
#[derive(Debug, Clone, Copy)]
pub struct PrepaymentsResource<'a> {
    api: &'a AccountingApi,
}

impl<'a> PrepaymentsResource<'a> {
    pub(crate) fn new(api: &'a AccountingApi) -> Self {
        Self { api }
    }

    /// Builds a request to list prepayments.
    pub fn list(&self) -> PrepaymentsListRequest<'a> {
        PrepaymentsListRequest::new(self.api)
    }

    /// Retrieves a prepayment by ID.
    pub async fn get(&self, prepayment_id: Uuid) -> Result<Vec<prepayment::Prepayment>, XeroError> {
        let path = format!("/Prepayments/{prepayment_id}");
        let resp: prepayment::PrepaymentsResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        Ok(resp.prepayments)
    }

    /// Allocates a prepayment to an invoice.
    pub async fn allocate(
        &self,
        prepayment_id: Uuid,
        allocation: Allocation,
    ) -> Result<Vec<Allocation>, XeroError> {
        let path = format!("/Prepayments/{prepayment_id}/Allocations");
        let resp: credit_note::AllocationsResponse = self
            .api
            .client
            .send_request(Method::PUT, &path, None, Some(allocation))
            .await?;
        Ok(resp.allocations)
    }

    /// Deletes a prepayment allocation.
    pub async fn delete_allocation(
        &self,
        prepayment_id: Uuid,
        allocation_id: Uuid,
    ) -> Result<(), XeroError> {
        let path = format!("/Prepayments/{prepayment_id}/Allocations/{allocation_id}");
        self.api
            .client
            .send_request_empty_response(Method::DELETE, &path, None::<()>)
            .await
    }
}

/// Builder for Prepayments list requests.
#[derive(Debug, Clone)]
pub struct PrepaymentsListRequest<'a> {
    api: &'a AccountingApi,
    where_filter: Option<String>,
    order_by: Option<String>,
    page: Option<u32>,
}

impl<'a> PrepaymentsListRequest<'a> {
    fn new(api: &'a AccountingApi) -> Self {
        Self {
            api,
            where_filter: None,
            order_by: None,
            page: None,
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

    /// Executes the list request.
    pub async fn send(self) -> Result<Vec<prepayment::Prepayment>, XeroError> {
        let mut query = QueryParams::default();
        query.push_opt_string("where", self.where_filter);
        query.push_opt_string("order", self.order_by);
        query.push_opt("page", self.page);

        let resp: prepayment::PrepaymentsResponse = self
            .api
            .client
            .send_request(Method::GET, "/Prepayments", query.as_slice(), None::<()>)
            .await?;
        Ok(resp.prepayments)
    }
}

impl AccountingApi {
    /// Retrieves one or many prepayments.
    pub async fn get_prepayments(
        &self,
        prepayment_id: Option<Uuid>,
        _modified_after: Option<DateTime<Utc>>,
        where_filter: Option<String>,
        order_by: Option<String>,
        page: Option<u32>,
    ) -> Result<Vec<prepayment::Prepayment>, XeroError> {
        if let Some(id) = prepayment_id {
            self.prepayments().get(id).await
        } else {
            let mut request = self.prepayments().list();
            if let Some(filter) = where_filter {
                request = request.where_filter(filter);
            }
            if let Some(order) = order_by {
                request = request.order_by(order);
            }
            if let Some(page) = page {
                request = request.page(page);
            }
            request.send().await
        }
    }

    /// Allocates a prepayment to an invoice.
    pub async fn allocate_prepayment(
        &self,
        prepayment_id: Uuid,
        allocation: Allocation,
    ) -> Result<Vec<Allocation>, XeroError> {
        self.prepayments().allocate(prepayment_id, allocation).await
    }

    /// Deletes a prepayment allocation.
    pub async fn delete_prepayment_allocation(
        &self,
        prepayment_id: Uuid,
        allocation_id: Uuid,
    ) -> Result<(), XeroError> {
        self.prepayments()
            .delete_allocation(prepayment_id, allocation_id)
            .await
    }
}
