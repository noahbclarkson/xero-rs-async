use super::PayrollUkApi;
use crate::error::XeroError;
use crate::models::payroll_uk::employee_statutory_leave_balance;
use reqwest::Method;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct EmployeeStatutoryLeaveBalanceResource<'a> {
    api: &'a PayrollUkApi,
}

impl<'a> EmployeeStatutoryLeaveBalanceResource<'a> {
    pub(crate) fn new(api: &'a PayrollUkApi) -> Self {
        Self { api }
    }

    pub async fn get(
        &self,
        employee_id: Uuid,
        leave_type: Option<String>,
        as_of_date: Option<String>,
    ) -> Result<employee_statutory_leave_balance::StatutoryLeaveBalance, XeroError> {
        let path = format!("/employees/{employee_id}/statutoryleavebalance");
        let mut query = Vec::new();
        if let Some(leave_type) = leave_type {
            query.push(("leaveType".to_string(), leave_type));
        }
        if let Some(as_of_date) = as_of_date {
            query.push(("asOfDate".to_string(), as_of_date));
        }
        let resp: employee_statutory_leave_balance::StatutoryLeaveBalanceResponse = self
            .api
            .client
            .send_request(Method::GET, &path, Some(&query), None::<()>)
            .await?;
        resp.leave_balance.ok_or(XeroError::Api {
            status: reqwest::StatusCode::NOT_FOUND,
            message: "Leave balance not found in response".to_string(),
        })
    }
}
