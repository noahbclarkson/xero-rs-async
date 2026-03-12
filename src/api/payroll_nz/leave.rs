use super::PayrollNzApi;
use crate::error::XeroError;
use crate::models::payroll_nz::leave;
use reqwest::Method;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct LeaveResource<'a> {
    api: &'a PayrollNzApi,
}

impl<'a> LeaveResource<'a> {
    pub(crate) fn new(api: &'a PayrollNzApi) -> Self {
        Self { api }
    }

    pub fn list(&self, employee_id: Uuid) -> LeaveListRequest<'a> {
        LeaveListRequest::new(self.api, employee_id)
    }

    pub async fn get(&self, employee_id: Uuid, leave_id: Uuid) -> Result<leave::Leave, XeroError> {
        let path = format!("/employees/{employee_id}/leave/{leave_id}");
        let resp: leave::LeaveResponse = self
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
        leave: leave::Leave,
    ) -> Result<leave::Leave, XeroError> {
        let path = format!("/employees/{employee_id}/leave");
        let resp: leave::LeaveResponse = self
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
        leave: leave::Leave,
    ) -> Result<leave::Leave, XeroError> {
        let path = format!("/employees/{employee_id}/leave/{leave_id}");
        let resp: leave::LeaveResponse = self
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
pub struct LeaveListRequest<'a> {
    api: &'a PayrollNzApi,
    employee_id: Uuid,
    page: Option<u32>,
}

impl<'a> LeaveListRequest<'a> {
    fn new(api: &'a PayrollNzApi, employee_id: Uuid) -> Self {
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

    pub async fn send(self) -> Result<Vec<leave::Leave>, XeroError> {
        let path = format!("/employees/{}/leave", self.employee_id);
        let mut query = Vec::new();
        if let Some(page) = self.page {
            query.push(("page".to_string(), page.to_string()));
        }
        let resp: leave::LeavesResponse = self
            .api
            .client
            .send_request(Method::GET, &path, Some(&query), None::<()>)
            .await?;
        Ok(resp.leave.unwrap_or_default())
    }
}
