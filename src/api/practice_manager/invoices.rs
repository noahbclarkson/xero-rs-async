//! Invoices resource for the XPM Practice Manager API v3.1.

use super::PracticeManagerApi;
use crate::error::XeroError;
use crate::models::practice_manager::invoice::{
    InvoiceResponse, InvoicesResponse, PaymentsResponse,
};
use reqwest::Method;

/// Resource accessor for XPM Invoices.
#[derive(Debug, Clone, Copy)]
pub struct InvoicesResource<'a> {
    api: &'a PracticeManagerApi,
}

impl<'a> InvoicesResource<'a> {
    pub(crate) fn new(api: &'a PracticeManagerApi) -> Self {
        Self { api }
    }

    /// Returns a list of current invoices.
    ///
    /// When `detailed` is `true`, each invoice includes full detail (jobs,
    /// tasks, costs, etc.).
    pub async fn current(&self, detailed: bool) -> Result<InvoicesResponse, XeroError> {
        let query: Vec<(String, String)> = if detailed {
            vec![("detailed".into(), "true".into())]
        } else {
            vec![]
        };
        let q = if query.is_empty() {
            None
        } else {
            Some(query.as_slice())
        };
        self.api
            .client
            .send_request_xml(Method::GET, "/invoice.api/current", q)
            .await
    }

    /// Retrieves detailed information for a specific invoice by its invoice number.
    pub async fn get(&self, invoice_number: &str) -> Result<InvoiceResponse, XeroError> {
        let path = format!("/invoice.api/get/{invoice_number}");
        self.api
            .client
            .send_request_xml(Method::GET, &path, None)
            .await
    }

    /// Returns a list of draft invoices.
    ///
    /// When `detailed` is `true`, each invoice includes full detail.
    pub async fn draft(&self, detailed: bool) -> Result<InvoicesResponse, XeroError> {
        let query: Vec<(String, String)> = if detailed {
            vec![("detailed".into(), "true".into())]
        } else {
            vec![]
        };
        let q = if query.is_empty() {
            None
        } else {
            Some(query.as_slice())
        };
        self.api
            .client
            .send_request_xml(Method::GET, "/invoice.api/draft", q)
            .await
    }

    /// Returns a list of invoices for a specific job.
    pub async fn by_job(&self, job_number: &str) -> Result<InvoicesResponse, XeroError> {
        let path = format!("/invoice.api/job/{job_number}");
        self.api
            .client
            .send_request_xml(Method::GET, &path, None)
            .await
    }

    /// Returns a list of current and archived invoices within a date range.
    ///
    /// `from` and `to` are in `YYYYMMDD` format. The maximum range is one year.
    /// When `detailed` is `true`, each invoice includes full detail.
    pub async fn list(
        &self,
        from: &str,
        to: &str,
        detailed: bool,
    ) -> Result<InvoicesResponse, XeroError> {
        let mut query = vec![
            ("from".into(), from.to_string()),
            ("to".into(), to.to_string()),
        ];
        if detailed {
            query.push(("detailed".into(), "true".into()));
        }
        self.api
            .client
            .send_request_xml(Method::GET, "/invoice.api/list", Some(query.as_slice()))
            .await
    }

    /// Returns a list of payments for an invoice.
    pub async fn payments(&self, invoice_number: &str) -> Result<PaymentsResponse, XeroError> {
        let path = format!("/invoice.api/payments/{invoice_number}");
        self.api
            .client
            .send_request_xml(Method::GET, &path, None)
            .await
    }
}
