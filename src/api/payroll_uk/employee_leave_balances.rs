use super::PayrollUkApi;
use crate::error::XeroError;
use crate::models::payroll_uk::employee_leave_balance;
use reqwest::Method;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct EmployeeLeaveBalancesResource<'a> {
    api: &'a PayrollUkApi,
}

impl<'a> EmployeeLeaveBalancesResource<'a> {
    pub(crate) fn new(api: &'a PayrollUkApi) -> Self {
        Self { api }
    }

    pub async fn list(
        &self,
        employee_id: Uuid,
    ) -> Result<Vec<employee_leave_balance::EmployeeLeaveBalance>, XeroError> {
        let path = format!("/employees/{employee_id}/leaveBalances");
        let resp: employee_leave_balance::LeaveBalanceResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        Ok(resp.leave_balances.unwrap_or_default())
    }
}
