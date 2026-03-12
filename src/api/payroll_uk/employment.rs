use super::PayrollUkApi;
use crate::error::XeroError;
use crate::models::payroll_uk::employment::{Employment, EmploymentResponse};
use reqwest::Method;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct EmploymentResource<'a> {
    api: &'a PayrollUkApi,
}

impl<'a> EmploymentResource<'a> {
    pub(crate) fn new(api: &'a PayrollUkApi) -> Self {
        Self { api }
    }

    pub async fn create(
        &self,
        employee_id: Uuid,
        employment: Employment,
    ) -> Result<EmploymentResponse, XeroError> {
        let path = format!("/employees/{employee_id}/employment");
        let resp: EmploymentResponse = self
            .api
            .client
            .send_request(Method::POST, &path, None, Some(employment))
            .await?;
        Ok(resp)
    }
}
