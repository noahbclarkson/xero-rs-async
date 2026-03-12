use super::PayrollNzApi;
use crate::error::XeroError;
use crate::models::payroll_nz::employee_leave_setup;
use reqwest::Method;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct EmployeeLeaveSetupResource<'a> {
    api: &'a PayrollNzApi,
}

impl<'a> EmployeeLeaveSetupResource<'a> {
    pub(crate) fn new(api: &'a PayrollNzApi) -> Self {
        Self { api }
    }

    pub async fn create(
        &self,
        employee_id: Uuid,
        leave_setup: employee_leave_setup::EmployeeLeaveSetup,
    ) -> Result<employee_leave_setup::EmployeeLeaveSetup, XeroError> {
        let path = format!("/employees/{employee_id}/leaveSetup");
        let resp: employee_leave_setup::EmployeeLeaveSetupResponse = self
            .api
            .client
            .send_request(Method::POST, &path, None, Some(leave_setup))
            .await?;
        resp.leave_setup.ok_or(XeroError::Api {
            status: reqwest::StatusCode::NOT_FOUND,
            message: "Leave setup not found in response".to_string(),
        })
    }
}
