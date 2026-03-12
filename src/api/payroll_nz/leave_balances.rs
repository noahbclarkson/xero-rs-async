use super::PayrollNzApi;
use crate::error::XeroError;
use crate::models::payroll_nz::employee_leave_balance;
use reqwest::Method;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct LeaveBalancesResource<'a> {
    api: &'a PayrollNzApi,
}

impl<'a> LeaveBalancesResource<'a> {
    pub(crate) fn new(api: &'a PayrollNzApi) -> Self {
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
