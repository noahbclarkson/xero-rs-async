use super::PayrollUkApi;
use crate::error::XeroError;
use crate::models::payroll_uk::employee_leave_type;
use reqwest::Method;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct EmployeeLeaveTypesResource<'a> {
    api: &'a PayrollUkApi,
}

impl<'a> EmployeeLeaveTypesResource<'a> {
    pub(crate) fn new(api: &'a PayrollUkApi) -> Self {
        Self { api }
    }

    pub async fn list(
        &self,
        employee_id: Uuid,
    ) -> Result<Vec<employee_leave_type::EmployeeLeaveType>, XeroError> {
        let path = format!("/employees/{employee_id}/leaveTypes");
        let resp: employee_leave_type::LeaveTypeResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        Ok(resp.leave_types.unwrap_or_default())
    }

    pub async fn create(
        &self,
        employee_id: Uuid,
        leave_type: employee_leave_type::EmployeeLeaveType,
    ) -> Result<Vec<employee_leave_type::EmployeeLeaveType>, XeroError> {
        let path = format!("/employees/{employee_id}/leaveTypes");
        let body = employee_leave_type::LeaveTypeRequest {
            leave_types: vec![leave_type],
        };
        let resp: employee_leave_type::LeaveTypeResponse = self
            .api
            .client
            .send_request(Method::POST, &path, None, Some(body))
            .await?;
        Ok(resp.leave_types.unwrap_or_default())
    }
}
