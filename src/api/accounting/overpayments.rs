use super::query::QueryParams;
use super::AccountingApi;
use crate::error::XeroError;
use crate::models::accounting::common::Allocation;
use crate::models::accounting::{credit_note, overpayment};
use chrono::{DateTime, Utc};
use reqwest::Method;
use uuid::Uuid;

/// Resource accessor for Overpayments.
#[derive(Debug, Clone, Copy)]
pub struct OverpaymentsResource<'a> {
    api: &'a AccountingApi,
}

impl<'a> OverpaymentsResource<'a> {
    pub(crate) fn new(api: &'a AccountingApi) -> Self {
        Self { api }
    }

    /// Builds a request to list overpayments.
    pub fn list(&self) -> OverpaymentsListRequest<'a> {
        OverpaymentsListRequest::new(self.api)
    }

    /// Retrieves an overpayment by ID.
    pub async fn get(
        &self,
        overpayment_id: Uuid,
    ) -> Result<Vec<overpayment::Overpayment>, XeroError> {
        let path = format!("/Overpayments/{overpayment_id}");
        let resp: overpayment::OverpaymentsResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        Ok(resp.overpayments)
    }

    /// Allocates an overpayment.
    pub async fn allocate(
        &self,
        overpayment_id: Uuid,
        allocation: Allocation,
    ) -> Result<Vec<Allocation>, XeroError> {
        let path = format!("/Overpayments/{overpayment_id}/Allocations");
        let resp: credit_note::AllocationsResponse = self
            .api
            .client
            .send_request(Method::PUT, &path, None, Some(allocation))
            .await?;
        Ok(resp.allocations)
    }

    /// Deletes an overpayment allocation.
    pub async fn delete_allocation(
        &self,
        overpayment_id: Uuid,
        allocation_id: Uuid,
    ) -> Result<(), XeroError> {
        let path = format!("/Overpayments/{overpayment_id}/Allocations/{allocation_id}");
        self.api
            .client
            .send_request_empty_response(Method::DELETE, &path, None::<()>)
            .await
    }
}

/// Builder for Overpayments list requests.
#[derive(Debug, Clone)]
pub struct OverpaymentsListRequest<'a> {
    api: &'a AccountingApi,
    where_filter: Option<String>,
    order_by: Option<String>,
    page: Option<u32>,
}

impl<'a> OverpaymentsListRequest<'a> {
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
    pub async fn send(self) -> Result<Vec<overpayment::Overpayment>, XeroError> {
        let mut query = QueryParams::default();
        query.push_opt_string("where", self.where_filter);
        query.push_opt_string("order", self.order_by);
        query.push_opt("page", self.page);

        let resp: overpayment::OverpaymentsResponse = self
            .api
            .client
            .send_request(Method::GET, "/Overpayments", query.as_slice(), None::<()>)
            .await?;
        Ok(resp.overpayments)
    }
}

impl AccountingApi {
    /// Retrieves one or many overpayments.
    pub async fn get_overpayments(
        &self,
        overpayment_id: Option<Uuid>,
        _modified_after: Option<DateTime<Utc>>,
        where_filter: Option<String>,
        order_by: Option<String>,
        page: Option<u32>,
    ) -> Result<Vec<overpayment::Overpayment>, XeroError> {
        if let Some(id) = overpayment_id {
            self.overpayments().get(id).await
        } else {
            let mut request = self.overpayments().list();
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

    /// Allocates an overpayment.
    pub async fn allocate_overpayment(
        &self,
        overpayment_id: Uuid,
        allocation: Allocation,
    ) -> Result<Vec<Allocation>, XeroError> {
        self.overpayments()
            .allocate(overpayment_id, allocation)
            .await
    }

    /// Deletes an overpayment allocation.
    pub async fn delete_overpayment_allocation(
        &self,
        overpayment_id: Uuid,
        allocation_id: Uuid,
    ) -> Result<(), XeroError> {
        self.overpayments()
            .delete_allocation(overpayment_id, allocation_id)
            .await
    }
}
