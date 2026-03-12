use super::PayrollNzApi;
use crate::error::XeroError;
use crate::models::payroll_nz::deduction;
use reqwest::Method;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct DeductionsResource<'a> {
    api: &'a PayrollNzApi,
}

impl<'a> DeductionsResource<'a> {
    pub(crate) fn new(api: &'a PayrollNzApi) -> Self {
        Self { api }
    }

    pub fn list(&self) -> DeductionsListRequest<'a> {
        DeductionsListRequest::new(self.api)
    }

    pub async fn get(&self, deduction_id: Uuid) -> Result<Vec<deduction::Deduction>, XeroError> {
        let path = format!("/deductions/{deduction_id}");
        let resp: deduction::DeductionResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        Ok(resp.deductions.unwrap_or_default())
    }

    pub async fn create(
        &self,
        item: deduction::Deduction,
    ) -> Result<Vec<deduction::Deduction>, XeroError> {
        let body = deduction::DeductionRequest {
            deductions: vec![item],
        };
        let resp: deduction::DeductionResponse = self
            .api
            .client
            .send_request(Method::POST, "/deductions", None, Some(body))
            .await?;
        Ok(resp.deductions.unwrap_or_default())
    }
}

#[derive(Debug, Clone)]
pub struct DeductionsListRequest<'a> {
    api: &'a PayrollNzApi,
    page: Option<u32>,
}

impl<'a> DeductionsListRequest<'a> {
    fn new(api: &'a PayrollNzApi) -> Self {
        Self { api, page: None }
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    pub async fn send(self) -> Result<Vec<deduction::Deduction>, XeroError> {
        let mut query = Vec::new();
        if let Some(page) = self.page {
            query.push(("page".to_string(), page.to_string()));
        }
        let resp: deduction::DeductionResponse = self
            .api
            .client
            .send_request(Method::GET, "/deductions", Some(&query), None::<()>)
            .await?;
        Ok(resp.deductions.unwrap_or_default())
    }
}
