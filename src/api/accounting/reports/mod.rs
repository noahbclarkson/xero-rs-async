use super::AccountingApi;
use crate::error::XeroError;
use crate::models::accounting::report;
use reqwest::Method;

mod base;
mod typed_api;
mod typed_reports;

/// Resource accessor for Reports.
#[derive(Debug, Clone, Copy)]
pub struct ReportsResource<'a> {
    api: &'a AccountingApi,
}

impl<'a> ReportsResource<'a> {
    pub(crate) fn new(api: &'a AccountingApi) -> Self {
        Self { api }
    }

    /// Retrieves a specific report by name.
    pub async fn get(
        &self,
        report_name: &str,
        params: Vec<(String, String)>,
    ) -> Result<report::Report, XeroError> {
        let path = format!("/Reports/{report_name}");
        let resp: report::ReportsResponse = self
            .api
            .client
            .send_request(Method::GET, &path, Some(&params), None::<()>)
            .await?;
        resp.reports
            .into_iter()
            .next()
            .ok_or_else(|| XeroError::Api {
                status: reqwest::StatusCode::NOT_FOUND,
                message: "Report not found in response".to_string(),
            })
    }
}
