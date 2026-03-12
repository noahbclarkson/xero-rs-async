use super::super::AccountingApi;
use crate::error::XeroError;
use crate::models::accounting::invoice;
use chrono::{DateTime, Utc};
use uuid::Uuid;

impl AccountingApi {
    /// Retrieves one or many invoices.
    #[allow(clippy::too_many_arguments)]
    pub async fn get_invoices(
        &self,
        invoice_id: Option<Uuid>,
        invoice_numbers: Option<Vec<String>>,
        contact_ids: Option<Vec<Uuid>>,
        statuses: Option<Vec<String>>,
        _modified_after: Option<DateTime<Utc>>,
        where_filter: Option<String>,
        order_by: Option<String>,
        page: Option<u32>,
        page_size: Option<u32>,
        summary_only: Option<bool>,
        search_term: Option<String>,
    ) -> Result<Vec<invoice::Invoice>, XeroError> {
        if let Some(id) = invoice_id {
            self.invoices().get(id).await
        } else {
            let mut request = self.invoices().list();
            if let Some(invoice_numbers) = invoice_numbers {
                request = request.invoice_numbers(invoice_numbers);
            }
            if let Some(contact_ids) = contact_ids {
                request = request.contact_ids(contact_ids);
            }
            if let Some(statuses) = statuses {
                request = request.statuses(statuses);
            }
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
            if let Some(summary_only) = summary_only {
                request = request.summary_only(summary_only);
            }
            if let Some(search_term) = search_term {
                request = request.search_term(search_term);
            }
            request.send().await
        }
    }

    /// Creates one or more new invoices.
    pub async fn create_invoices(
        &self,
        invoices: Vec<invoice::Invoice>,
        summarize_errors: Option<bool>,
    ) -> Result<Vec<invoice::Invoice>, XeroError> {
        self.invoices().create(invoices, summarize_errors).await
    }

    /// Updates an existing invoice.
    pub async fn update_invoice(
        &self,
        invoice_id: Uuid,
        invoice_data: invoice::Invoice,
    ) -> Result<Vec<invoice::Invoice>, XeroError> {
        self.invoices().update(invoice_id, invoice_data).await
    }

    /// Retrieves the online invoice URL for a sales invoice.
    pub async fn get_online_invoice_url(
        &self,
        invoice_id: Uuid,
    ) -> Result<invoice::OnlineInvoice, XeroError> {
        self.invoices().online_invoice_url(invoice_id).await
    }

    /// Emails a sales invoice from Xero.
    pub async fn email_invoice(&self, invoice_id: Uuid) -> Result<(), XeroError> {
        self.invoices().email(invoice_id).await
    }
}
