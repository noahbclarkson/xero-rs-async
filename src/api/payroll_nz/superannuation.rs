use super::PayrollNzApi;
use crate::error::XeroError;
use crate::models::payroll_nz::superannuation;
use reqwest::Method;

#[derive(Debug, Clone, Copy)]
pub struct SuperannuationResource<'a> {
    api: &'a PayrollNzApi,
}

impl<'a> SuperannuationResource<'a> {
    pub(crate) fn new(api: &'a PayrollNzApi) -> Self {
        Self { api }
    }

    pub async fn list(&self) -> Result<Vec<superannuation::Superannuation>, XeroError> {
        let resp: superannuation::SuperannuationResponse = self
            .api
            .client
            .send_request(Method::GET, "/superannuation", None, None::<()>)
            .await?;
        Ok(resp.superannuations.unwrap_or_default())
    }
}
