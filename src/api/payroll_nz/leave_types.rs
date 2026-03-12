use super::PayrollNzApi;
use crate::error::XeroError;
use crate::models::payroll_nz::leave_type;
use reqwest::Method;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct LeaveTypesResource<'a> {
    api: &'a PayrollNzApi,
}

impl<'a> LeaveTypesResource<'a> {
    pub(crate) fn new(api: &'a PayrollNzApi) -> Self {
        Self { api }
    }

    pub fn list(&self) -> LeaveTypesListRequest<'a> {
        LeaveTypesListRequest::new(self.api)
    }

    pub async fn get(&self, leave_type_id: Uuid) -> Result<Vec<leave_type::LeaveType>, XeroError> {
        let path = format!("/leaveTypes/{leave_type_id}");
        let resp: leave_type::LeaveTypeResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        Ok(resp.leave_types.unwrap_or_default())
    }

    pub async fn create(
        &self,
        item: leave_type::LeaveType,
    ) -> Result<Vec<leave_type::LeaveType>, XeroError> {
        let body = leave_type::LeaveTypeRequest {
            leave_types: vec![item],
        };
        let resp: leave_type::LeaveTypeResponse = self
            .api
            .client
            .send_request(Method::POST, "/leaveTypes", None, Some(body))
            .await?;
        Ok(resp.leave_types.unwrap_or_default())
    }
}

#[derive(Debug, Clone)]
pub struct LeaveTypesListRequest<'a> {
    api: &'a PayrollNzApi,
    page: Option<u32>,
}

impl<'a> LeaveTypesListRequest<'a> {
    fn new(api: &'a PayrollNzApi) -> Self {
        Self { api, page: None }
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    pub async fn send(self) -> Result<Vec<leave_type::LeaveType>, XeroError> {
        let mut query = Vec::new();
        if let Some(page) = self.page {
            query.push(("page".to_string(), page.to_string()));
        }
        let resp: leave_type::LeaveTypeResponse = self
            .api
            .client
            .send_request(Method::GET, "/leaveTypes", Some(&query), None::<()>)
            .await?;
        Ok(resp.leave_types.unwrap_or_default())
    }
}
