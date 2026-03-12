use super::PayrollNzApi;
use crate::error::XeroError;
use crate::models::payroll_nz::statutory_deduction;
use reqwest::Method;

#[derive(Debug, Clone, Copy)]
pub struct StatutoryDeductionsResource<'a> {
    api: &'a PayrollNzApi,
}

impl<'a> StatutoryDeductionsResource<'a> {
    pub(crate) fn new(api: &'a PayrollNzApi) -> Self {
        Self { api }
    }

    pub async fn list(&self) -> Result<Vec<statutory_deduction::StatutoryDeduction>, XeroError> {
        let resp: statutory_deduction::StatutoryDeductionsResponse = self
            .api
            .client
            .send_request(Method::GET, "/statutoryDeductions", None, None::<()>)
            .await?;
        Ok(resp.statutory_deductions.unwrap_or_default())
    }
}
