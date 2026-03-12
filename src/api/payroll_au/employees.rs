use super::PayrollAuApi;
use crate::error::XeroError;
use crate::models::payroll_au::employee;
use reqwest::Method;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct EmployeesResource<'a> {
    api: &'a PayrollAuApi,
}

impl<'a> EmployeesResource<'a> {
    pub(crate) fn new(api: &'a PayrollAuApi) -> Self {
        Self { api }
    }

    pub fn list(&self) -> EmployeesListRequest<'a> {
        EmployeesListRequest::new(self.api)
    }

    pub async fn get(&self, employee_id: Uuid) -> Result<Vec<employee::Employee>, XeroError> {
        let path = format!("/Employees/{employee_id}");
        let resp: employee::EmployeesResponse = self
            .api
            .client_v1
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        Ok(resp.employees)
    }

    pub async fn create_or_update(
        &self,
        employees: Vec<employee::Employee>,
    ) -> Result<Vec<employee::Employee>, XeroError> {
        let body = if employees.len() == 1 {
            serde_json::to_value(employees.into_iter().next().unwrap())?
        } else {
            serde_json::to_value(employee::EmployeesRequest { employees })?
        };
        let resp: employee::EmployeesResponse = self
            .api
            .client_v1
            .send_request(Method::POST, "/Employees", None, Some(body))
            .await?;
        Ok(resp.employees)
    }
}

#[derive(Debug, Clone)]
pub struct EmployeesListRequest<'a> {
    api: &'a PayrollAuApi,
    page: Option<u32>,
    where_filter: Option<String>,
    order_by: Option<String>,
}

impl<'a> EmployeesListRequest<'a> {
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

    pub async fn send(self) -> Result<Vec<employee::Employee>, XeroError> {
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
        let resp: employee::EmployeesResponse = self
            .api
            .client_v1
            .send_request(Method::GET, "/Employees", Some(&query), None::<()>)
            .await?;
        Ok(resp.employees)
    }
}
