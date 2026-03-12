use super::PayrollNzApi;
use crate::error::XeroError;
use crate::models::payroll_nz::employment;
use reqwest::Method;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct EmploymentResource<'a> {
    api: &'a PayrollNzApi,
}

impl<'a> EmploymentResource<'a> {
    pub(crate) fn new(api: &'a PayrollNzApi) -> Self {
        Self { api }
    }

    pub async fn update(
        &self,
        employee_id: Uuid,
        employment: employment::Employment,
    ) -> Result<employment::Employment, XeroError> {
        let path = format!("/employees/{employee_id}/employment");
        let resp: employment::EmploymentResponse = self
            .api
            .client
            .send_request(Method::POST, &path, None, Some(employment))
            .await?;
        resp.employment.ok_or(XeroError::Api {
            status: reqwest::StatusCode::NOT_FOUND,
            message: "Employment not found in response".to_string(),
        })
    }
}
