use super::query::QueryParams;
use super::AccountingApi;
use crate::error::XeroError;
use crate::models::accounting::repeating_invoice;
use reqwest::Method;
use uuid::Uuid;

/// Resource accessor for Repeating Invoices.
#[derive(Debug, Clone, Copy)]
pub struct RepeatingInvoicesResource<'a> {
    api: &'a AccountingApi,
}

impl<'a> RepeatingInvoicesResource<'a> {
    pub(crate) fn new(api: &'a AccountingApi) -> Self {
        Self { api }
    }

    /// Builds a request to list repeating invoices.
    pub fn list(&self) -> RepeatingInvoicesListRequest<'a> {
        RepeatingInvoicesListRequest::new(self.api)
    }

    /// Retrieves a repeating invoice template by ID.
    pub async fn get(
        &self,
        repeating_invoice_id: Uuid,
    ) -> Result<Vec<repeating_invoice::RepeatingInvoice>, XeroError> {
        let path = format!("/RepeatingInvoices/{repeating_invoice_id}");
        let resp: repeating_invoice::RepeatingInvoicesResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        Ok(resp.repeating_invoices)
    }

    /// Creates or deletes one or more repeating invoice templates.
    pub async fn create_or_delete(
        &self,
        invoices: Vec<repeating_invoice::RepeatingInvoice>,
    ) -> Result<Vec<repeating_invoice::RepeatingInvoice>, XeroError> {
        let body = if invoices.len() == 1 {
            serde_json::to_value(invoices.into_iter().next().unwrap())?
        } else {
            serde_json::to_value(repeating_invoice::RepeatingInvoicesRequest {
                repeating_invoices: invoices,
            })?
        };
        let resp: repeating_invoice::RepeatingInvoicesResponse = self
            .api
            .client
            .send_request(Method::POST, "/RepeatingInvoices", None, Some(body))
            .await?;
        Ok(resp.repeating_invoices)
    }
}

/// Builder for Repeating Invoices list requests.
#[derive(Debug, Clone)]
pub struct RepeatingInvoicesListRequest<'a> {
    api: &'a AccountingApi,
    where_filter: Option<String>,
    order_by: Option<String>,
}

impl<'a> RepeatingInvoicesListRequest<'a> {
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
    pub async fn send(self) -> Result<Vec<repeating_invoice::RepeatingInvoice>, XeroError> {
        let mut query = QueryParams::default();
        query.push_opt_string("where", self.where_filter);
        query.push_opt_string("order", self.order_by);

        let resp: repeating_invoice::RepeatingInvoicesResponse = self
            .api
            .client
            .send_request(
                Method::GET,
                "/RepeatingInvoices",
                query.as_slice(),
                None::<()>,
            )
            .await?;
        Ok(resp.repeating_invoices)
    }
}

impl AccountingApi {
    /// Retrieves one or many repeating invoice templates.
    pub async fn get_repeating_invoices(
        &self,
        repeating_invoice_id: Option<Uuid>,
        where_filter: Option<String>,
        order_by: Option<String>,
    ) -> Result<Vec<repeating_invoice::RepeatingInvoice>, XeroError> {
        if let Some(id) = repeating_invoice_id {
            self.repeating_invoices().get(id).await
        } else {
            let mut request = self.repeating_invoices().list();
            if let Some(filter) = where_filter {
                request = request.where_filter(filter);
            }
            if let Some(order) = order_by {
                request = request.order_by(order);
            }
            request.send().await
        }
    }

    /// Creates or deletes one or more repeating invoice templates.
    pub async fn create_or_delete_repeating_invoices(
        &self,
        invoices: Vec<repeating_invoice::RepeatingInvoice>,
    ) -> Result<Vec<repeating_invoice::RepeatingInvoice>, XeroError> {
        self.repeating_invoices().create_or_delete(invoices).await
    }
}
