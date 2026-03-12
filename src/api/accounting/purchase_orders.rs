use super::query::QueryParams;
use super::AccountingApi;
use crate::error::XeroError;
use crate::models::accounting::purchase_order;
use reqwest::Method;
use uuid::Uuid;

/// Resource accessor for Purchase Orders.
#[derive(Debug, Clone, Copy)]
pub struct PurchaseOrdersResource<'a> {
    api: &'a AccountingApi,
}

impl<'a> PurchaseOrdersResource<'a> {
    pub(crate) fn new(api: &'a AccountingApi) -> Self {
        Self { api }
    }

    /// Builds a request to list purchase orders.
    pub fn list(&self) -> PurchaseOrdersListRequest<'a> {
        PurchaseOrdersListRequest::new(self.api)
    }

    /// Retrieves a purchase order by ID.
    pub async fn get(
        &self,
        purchase_order_id: Uuid,
    ) -> Result<Vec<purchase_order::PurchaseOrder>, XeroError> {
        let path = format!("/PurchaseOrders/{purchase_order_id}");
        let resp: purchase_order::PurchaseOrdersResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        Ok(resp.purchase_orders)
    }

    /// Creates or updates one or more purchase orders.
    pub async fn create_or_update(
        &self,
        purchase_orders: Vec<purchase_order::PurchaseOrder>,
        summarize_errors: Option<bool>,
    ) -> Result<Vec<purchase_order::PurchaseOrder>, XeroError> {
        let mut query = QueryParams::default();
        query.push_opt("summarizeErrors", summarize_errors);
        let body = if purchase_orders.len() == 1 {
            serde_json::to_value(purchase_orders.into_iter().next().unwrap())?
        } else {
            serde_json::to_value(purchase_order::PurchaseOrdersRequest { purchase_orders })?
        };
        let resp: purchase_order::PurchaseOrdersResponse = self
            .api
            .client
            .send_request(
                Method::POST,
                "/PurchaseOrders",
                query.as_slice(),
                Some(body),
            )
            .await?;
        Ok(resp.purchase_orders)
    }
}

/// Builder for Purchase Orders list requests.
#[derive(Debug, Clone)]
pub struct PurchaseOrdersListRequest<'a> {
    api: &'a AccountingApi,
    status: Option<String>,
    date_from: Option<String>,
    date_to: Option<String>,
    order_by: Option<String>,
    page: Option<u32>,
    page_size: Option<u32>,
}

impl<'a> PurchaseOrdersListRequest<'a> {
    fn new(api: &'a AccountingApi) -> Self {
        Self {
            api,
            status: None,
            date_from: None,
            date_to: None,
            order_by: None,
            page: None,
            page_size: None,
        }
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
    pub async fn send(self) -> Result<Vec<purchase_order::PurchaseOrder>, XeroError> {
        let mut query = QueryParams::default();
        query.push_opt_string("status", self.status);
        query.push_opt_string("DateFrom", self.date_from);
        query.push_opt_string("DateTo", self.date_to);
        query.push_opt_string("order", self.order_by);
        query.push_opt("page", self.page);
        query.push_opt("pageSize", self.page_size);

        let resp: purchase_order::PurchaseOrdersResponse = self
            .api
            .client
            .send_request(Method::GET, "/PurchaseOrders", query.as_slice(), None::<()>)
            .await?;
        Ok(resp.purchase_orders)
    }
}

impl AccountingApi {
    /// Retrieves one or many purchase orders.
    #[allow(clippy::too_many_arguments)]
    pub async fn get_purchase_orders(
        &self,
        purchase_order_id: Option<Uuid>,
        status: Option<String>,
        date_from: Option<String>,
        date_to: Option<String>,
        order_by: Option<String>,
        page: Option<u32>,
        page_size: Option<u32>,
    ) -> Result<Vec<purchase_order::PurchaseOrder>, XeroError> {
        if let Some(id) = purchase_order_id {
            self.purchase_orders().get(id).await
        } else {
            let mut request = self.purchase_orders().list();
            if let Some(status) = status {
                request = request.status(status);
            }
            if let Some(date_from) = date_from {
                request = request.date_from(date_from);
            }
            if let Some(date_to) = date_to {
                request = request.date_to(date_to);
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

    /// Creates or updates one or more purchase orders.
    pub async fn create_or_update_purchase_orders(
        &self,
        purchase_orders: Vec<purchase_order::PurchaseOrder>,
        summarize_errors: Option<bool>,
    ) -> Result<Vec<purchase_order::PurchaseOrder>, XeroError> {
        self.purchase_orders()
            .create_or_update(purchase_orders, summarize_errors)
            .await
    }
}
