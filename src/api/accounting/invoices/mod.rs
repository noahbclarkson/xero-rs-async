use super::AccountingApi;
use crate::error::XeroError;
use crate::models::accounting::invoice;
use reqwest::Method;
use uuid::Uuid;

mod legacy;
mod list;

pub use list::InvoicesListRequest;

/// Resource accessor for Invoices.
#[derive(Debug, Clone, Copy)]
pub struct InvoicesResource<'a> {
    api: &'a AccountingApi,
}

impl<'a> InvoicesResource<'a> {
    pub(crate) fn new(api: &'a AccountingApi) -> Self {
        Self { api }
    }

    /// Builds a request to list invoices.
    pub fn list(&self) -> InvoicesListRequest<'a> {
        InvoicesListRequest::new(self.api)
    }

    /// Retrieves a single invoice by ID.
    pub async fn get(&self, invoice_id: Uuid) -> Result<Vec<invoice::Invoice>, XeroError> {
        let path = format!("/Invoices/{invoice_id}");
        let resp: invoice::InvoicesResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        Ok(resp.invoices)
    }

    /// Creates one or more new invoices.
    pub async fn create(
        &self,
        invoices: Vec<invoice::Invoice>,
        summarize_errors: Option<bool>,
    ) -> Result<Vec<invoice::Invoice>, XeroError> {
        let mut query = super::query::QueryParams::default();
        query.push_opt("summarizeErrors", summarize_errors);
        let body = if invoices.len() == 1 {
            serde_json::to_value(invoices.into_iter().next().unwrap())?
        } else {
            serde_json::to_value(invoice::InvoicesRequest { invoices })?
        };
        let resp: invoice::InvoicesResponse = self
            .api
            .client
            .send_request(Method::PUT, "/Invoices", query.as_slice(), Some(body))
            .await?;
        Ok(resp.invoices)
    }

    /// Updates an existing invoice.
    pub async fn update(
        &self,
        invoice_id: Uuid,
        invoice_data: invoice::Invoice,
    ) -> Result<Vec<invoice::Invoice>, XeroError> {
        let path = format!("/Invoices/{invoice_id}");
        let resp: invoice::InvoicesResponse = self
            .api
            .client
            .send_request(Method::POST, &path, None, Some(invoice_data))
            .await?;
        Ok(resp.invoices)
    }

    /// Retrieves the online invoice URL for a sales invoice.
    pub async fn online_invoice_url(
        &self,
        invoice_id: Uuid,
    ) -> Result<invoice::OnlineInvoice, XeroError> {
        let path = format!("/Invoices/{invoice_id}/OnlineInvoice");
        let mut resp: invoice::OnlineInvoicesResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        resp.online_invoices.pop().ok_or(XeroError::Api {
            status: reqwest::StatusCode::NOT_FOUND,
            message: "OnlineInvoice not found in response".to_string(),
        })
    }

    /// Emails a sales invoice from Xero.
    pub async fn email(&self, invoice_id: Uuid) -> Result<(), XeroError> {
        let path = format!("/Invoices/{invoice_id}/Email");
        self.api
            .client
            .send_request_empty_response(Method::POST, &path, None::<()>)
            .await
    }
}
