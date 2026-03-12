use super::PayrollAuApi;
use crate::error::XeroError;
use crate::models::payroll_au::leave_application;
use reqwest::Method;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct LeaveApplicationsResource<'a> {
    api: &'a PayrollAuApi,
}

impl<'a> LeaveApplicationsResource<'a> {
    pub(crate) fn new(api: &'a PayrollAuApi) -> Self {
        Self { api }
    }

    pub fn list(&self) -> LeaveApplicationsListRequest<'a> {
        LeaveApplicationsListRequest::new(self.api)
    }

    pub async fn get(
        &self,
        leave_application_id: Uuid,
    ) -> Result<Vec<leave_application::LeaveApplication>, XeroError> {
        let path = format!("/LeaveApplications/{leave_application_id}");
        let resp: leave_application::LeaveApplicationsResponse = self
            .api
            .client_v1
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        Ok(resp.leave_applications)
    }

    pub async fn create_or_update(
        &self,
        leave_applications: Vec<leave_application::LeaveApplication>,
    ) -> Result<Vec<leave_application::LeaveApplication>, XeroError> {
        let body = if leave_applications.len() == 1 {
            serde_json::to_value(leave_applications.into_iter().next().unwrap())?
        } else {
            serde_json::to_value(leave_application::LeaveApplicationsRequest {
                leave_applications,
            })?
        };
        let resp: leave_application::LeaveApplicationsResponse = self
            .api
            .client_v1
            .send_request(Method::POST, "/LeaveApplications", None, Some(body))
            .await?;
        Ok(resp.leave_applications)
    }

    pub async fn list_v2(&self) -> Result<Vec<leave_application::LeaveApplication>, XeroError> {
        let resp: leave_application::LeaveApplicationsResponse = self
            .api
            .client_v1
            .send_request(Method::GET, "/LeaveApplications/v2", None, None::<()>)
            .await?;
        Ok(resp.leave_applications)
    }
}

#[derive(Debug, Clone)]
pub struct LeaveApplicationsListRequest<'a> {
    api: &'a PayrollAuApi,
    page: Option<u32>,
    where_filter: Option<String>,
    order_by: Option<String>,
}

impl<'a> LeaveApplicationsListRequest<'a> {
    fn new(api: &'a PayrollAuApi) -> Self {
        Self {
            api,
            page: None,
            where_filter: None,
            order_by: None,
        }
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    pub fn where_filter(mut self, filter: impl Into<String>) -> Self {
        self.where_filter = Some(filter.into());
        self
    }

    pub fn order_by(mut self, order: impl Into<String>) -> Self {
        self.order_by = Some(order.into());
        self
    }

    pub async fn send(self) -> Result<Vec<leave_application::LeaveApplication>, XeroError> {
        let mut query = Vec::new();
        if let Some(page) = self.page {
            query.push(("page".to_string(), page.to_string()));
        }
        if let Some(filter) = self.where_filter {
            query.push(("where".to_string(), filter));
        }
        if let Some(order) = self.order_by {
            query.push(("order".to_string(), order));
        }
        let resp: leave_application::LeaveApplicationsResponse = self
            .api
            .client_v1
            .send_request(Method::GET, "/LeaveApplications", Some(&query), None::<()>)
            .await?;
        Ok(resp.leave_applications)
    }
}
