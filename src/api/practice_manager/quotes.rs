//! Quotes resource for the XPM Practice Manager API v3.1.

use super::PracticeManagerApi;
use crate::error::XeroError;
use crate::models::practice_manager::quote::{QuoteResponse, QuotesResponse};
use reqwest::Method;

/// Resource accessor for XPM Quotes.
#[derive(Debug, Clone, Copy)]
pub struct QuotesResource<'a> {
    api: &'a PracticeManagerApi,
}

impl<'a> QuotesResource<'a> {
    pub(crate) fn new(api: &'a PracticeManagerApi) -> Self {
        Self { api }
    }

    /// Returns a list of current quotes.
    ///
    /// When `detailed` is `true`, each quote includes full detail (tasks,
    /// costs, options, etc.).
    pub async fn current(&self, detailed: bool) -> Result<QuotesResponse, XeroError> {
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
            .send_request_xml(Method::GET, "/quote.api/current", q)
            .await
    }

    /// Retrieves detailed information for a specific quote by its quote number.
    pub async fn get(&self, quote_number: &str) -> Result<QuoteResponse, XeroError> {
        let path = format!("/quote.api/get/{quote_number}");
        self.api
            .client
            .send_request_xml(Method::GET, &path, None)
            .await
    }

    /// Returns a list of draft quotes.
    ///
    /// When `detailed` is `true`, each quote includes full detail.
    pub async fn draft(&self, detailed: bool) -> Result<QuotesResponse, XeroError> {
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
            .send_request_xml(Method::GET, "/quote.api/draft", q)
            .await
    }

    /// Returns a list of current and archived quotes within a date range.
    ///
    /// `from` and `to` are in `YYYYMMDD` format. The maximum range is one year.
    /// When `detailed` is `true`, each quote includes full detail.
    pub async fn list(
        &self,
        from: &str,
        to: &str,
        detailed: bool,
    ) -> Result<QuotesResponse, XeroError> {
        let mut query = vec![
            ("from".into(), from.to_string()),
            ("to".into(), to.to_string()),
        ];
        if detailed {
            query.push(("detailed".into(), "true".into()));
        }
        self.api
            .client
            .send_request_xml(Method::GET, "/quote.api/list", Some(query.as_slice()))
            .await
    }
}
