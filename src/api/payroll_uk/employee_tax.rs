use super::PayrollUkApi;
use crate::error::XeroError;
use crate::models::payroll_uk::employee_tax::EmployeeTaxResponse;
use reqwest::Method;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct EmployeeTaxResource<'a> {
    api: &'a PayrollUkApi,
}

impl<'a> EmployeeTaxResource<'a> {
    pub(crate) fn new(api: &'a PayrollUkApi) -> Self {
        Self { api }
    }

    pub async fn get(&self, employee_id: Uuid) -> Result<EmployeeTaxResponse, XeroError> {
        let path = format!("/employees/{employee_id}/tax");
        self.api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await
    }
}
