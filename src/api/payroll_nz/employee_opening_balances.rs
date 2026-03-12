use super::PayrollNzApi;
use crate::error::XeroError;
use crate::models::payroll_nz::employee_opening_balances;
use reqwest::Method;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct EmployeeOpeningBalancesResource<'a> {
    api: &'a PayrollNzApi,
}

impl<'a> EmployeeOpeningBalancesResource<'a> {
    pub(crate) fn new(api: &'a PayrollNzApi) -> Self {
        Self { api }
    }

    pub async fn get(
        &self,
        employee_id: Uuid,
    ) -> Result<employee_opening_balances::EmployeeOpeningBalances, XeroError> {
        let path = format!("/employees/{employee_id}/openingBalances");
        let resp: employee_opening_balances::EmployeeOpeningBalancesResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        resp.opening_balances.ok_or(XeroError::Api {
            status: reqwest::StatusCode::NOT_FOUND,
            message: "Opening balances not found in response".to_string(),
        })
    }

    pub async fn create(
        &self,
        employee_id: Uuid,
        balances: employee_opening_balances::EmployeeOpeningBalances,
    ) -> Result<employee_opening_balances::EmployeeOpeningBalances, XeroError> {
        let path = format!("/employees/{employee_id}/openingBalances");
        let body = employee_opening_balances::EmployeeOpeningBalancesRequest {
            opening_balances: balances,
        };
        let resp: employee_opening_balances::EmployeeOpeningBalancesResponse = self
            .api
            .client
            .send_request(Method::POST, &path, None, Some(body))
            .await?;
        resp.opening_balances.ok_or(XeroError::Api {
            status: reqwest::StatusCode::NOT_FOUND,
            message: "Opening balances not found in response".to_string(),
        })
    }
}
