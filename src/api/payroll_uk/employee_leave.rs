use super::PayrollUkApi;
use crate::error::XeroError;
use crate::models::payroll_uk::employee_leave;
use reqwest::Method;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct EmployeeLeaveResource<'a> {
    api: &'a PayrollUkApi,
}

impl<'a> EmployeeLeaveResource<'a> {
    pub(crate) fn new(api: &'a PayrollUkApi) -> Self {
        Self { api }
    }

    pub fn list(&self, employee_id: Uuid) -> EmployeeLeaveListRequest<'a> {
        EmployeeLeaveListRequest::new(self.api, employee_id)
    }

    pub async fn get(
        &self,
        employee_id: Uuid,
        leave_id: Uuid,
    ) -> Result<employee_leave::EmployeeLeave, XeroError> {
        let path = format!("/employees/{employee_id}/leave/{leave_id}");
        let resp: employee_leave::EmployeeLeaveResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        resp.leave.ok_or(XeroError::Api {
            status: reqwest::StatusCode::NOT_FOUND,
            message: "Leave not found in response".to_string(),
        })
    }

    pub async fn create(
        &self,
        employee_id: Uuid,
        leave: employee_leave::EmployeeLeave,
    ) -> Result<employee_leave::EmployeeLeave, XeroError> {
        let path = format!("/employees/{employee_id}/leave");
        let resp: employee_leave::EmployeeLeaveResponse = self
            .api
            .client
            .send_request(Method::POST, &path, None, Some(leave))
            .await?;
        resp.leave.ok_or(XeroError::Api {
            status: reqwest::StatusCode::NOT_FOUND,
            message: "Leave not found in response".to_string(),
        })
    }

    pub async fn update(
        &self,
        employee_id: Uuid,
        leave_id: Uuid,
        leave: employee_leave::EmployeeLeave,
    ) -> Result<employee_leave::EmployeeLeave, XeroError> {
        let path = format!("/employees/{employee_id}/leave/{leave_id}");
        let resp: employee_leave::EmployeeLeaveResponse = self
            .api
            .client
            .send_request(Method::PUT, &path, None, Some(leave))
            .await?;
        resp.leave.ok_or(XeroError::Api {
            status: reqwest::StatusCode::NOT_FOUND,
            message: "Leave not found in response".to_string(),
        })
    }

    pub async fn delete(&self, employee_id: Uuid, leave_id: Uuid) -> Result<(), XeroError> {
        let path = format!("/employees/{employee_id}/leave/{leave_id}");
        self.api
            .client
            .send_request_empty_response(Method::DELETE, &path, None::<()>)
            .await
    }
}

#[derive(Debug, Clone)]
pub struct EmployeeLeaveListRequest<'a> {
    api: &'a PayrollUkApi,
    employee_id: Uuid,
    page: Option<u32>,
}

impl<'a> EmployeeLeaveListRequest<'a> {
    fn new(api: &'a PayrollUkApi, employee_id: Uuid) -> Self {
        Self {
            api,
            employee_id,
            page: None,
        }
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    pub async fn send(self) -> Result<Vec<employee_leave::EmployeeLeave>, XeroError> {
        let path = format!("/employees/{}/leave", self.employee_id);
        let mut query = Vec::new();
        if let Some(page) = self.page {
            query.push(("page".to_string(), page.to_string()));
        }
        let resp: employee_leave::EmployeeLeavesResponse = self
            .api
            .client
            .send_request(Method::GET, &path, Some(&query), None::<()>)
            .await?;
        Ok(resp.leave.unwrap_or_default())
    }
}
