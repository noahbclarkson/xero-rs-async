use super::PayrollNzApi;
use crate::error::XeroError;
use crate::models::payroll_nz::employee_tax;
use reqwest::Method;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct EmployeeTaxResource<'a> {
    api: &'a PayrollNzApi,
}

impl<'a> EmployeeTaxResource<'a> {
    pub(crate) fn new(api: &'a PayrollNzApi) -> Self {
        Self { api }
    }

    pub async fn get(&self, employee_id: Uuid) -> Result<employee_tax::EmployeeTax, XeroError> {
        let path = format!("/employees/{employee_id}/tax");
        let resp: employee_tax::EmployeeTaxResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        resp.tax.ok_or(XeroError::Api {
            status: reqwest::StatusCode::NOT_FOUND,
            message: "Employee tax not found in response".to_string(),
        })
    }

    pub async fn update(
        &self,
        employee_id: Uuid,
        tax: employee_tax::EmployeeTax,
    ) -> Result<employee_tax::EmployeeTax, XeroError> {
        let path = format!("/employees/{employee_id}/tax");
        let resp: employee_tax::EmployeeTaxResponse = self
            .api
            .client
            .send_request(Method::POST, &path, None, Some(tax))
            .await?;
        resp.tax.ok_or(XeroError::Api {
            status: reqwest::StatusCode::NOT_FOUND,
            message: "Employee tax not found in response".to_string(),
        })
    }
}
