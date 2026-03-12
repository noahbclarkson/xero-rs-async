use super::PayrollUkApi;
use crate::error::XeroError;
use crate::models::payroll_uk::employee;
use reqwest::Method;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct EmployeesResource<'a> {
    api: &'a PayrollUkApi,
}

impl<'a> EmployeesResource<'a> {
    pub(crate) fn new(api: &'a PayrollUkApi) -> Self {
        Self { api }
    }

    pub fn list(&self) -> EmployeesListRequest<'a> {
        EmployeesListRequest::new(self.api)
    }

    pub async fn get(&self, employee_id: Uuid) -> Result<Vec<employee::Employee>, XeroError> {
        let path = format!("/employees/{employee_id}");
        let resp: employee::EmployeeResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        Ok(resp.employees.unwrap_or_default())
    }

    pub async fn create(
        &self,
        employee: employee::Employee,
    ) -> Result<Vec<employee::Employee>, XeroError> {
        let body = employee::EmployeeRequest {
            employees: vec![employee],
        };
        let resp: employee::EmployeeResponse = self
            .api
            .client
            .send_request(Method::POST, "/employees", None, Some(body))
            .await?;
        Ok(resp.employees.unwrap_or_default())
    }

    pub async fn update(
        &self,
        employee_id: Uuid,
        employee: employee::Employee,
    ) -> Result<Vec<employee::Employee>, XeroError> {
        let path = format!("/employees/{employee_id}");
        let body = employee::EmployeeRequest {
            employees: vec![employee],
        };
        let resp: employee::EmployeeResponse = self
            .api
            .client
            .send_request(Method::PUT, &path, None, Some(body))
            .await?;
        Ok(resp.employees.unwrap_or_default())
    }
}

#[derive(Debug, Clone)]
pub struct EmployeesListRequest<'a> {
    api: &'a PayrollUkApi,
    page: Option<u32>,
    filter: Option<String>,
}

impl<'a> EmployeesListRequest<'a> {
    fn new(api: &'a PayrollUkApi) -> Self {
        Self {
            api,
            page: None,
            filter: None,
        }
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    pub fn filter(mut self, filter: impl Into<String>) -> Self {
        self.filter = Some(filter.into());
        self
    }

    pub async fn send(self) -> Result<Vec<employee::Employee>, XeroError> {
        let mut query = Vec::new();
        if let Some(page) = self.page {
            query.push(("page".to_string(), page.to_string()));
        }
        if let Some(filter) = self.filter {
            query.push(("filter".to_string(), filter));
        }
        let resp: employee::EmployeeResponse = self
            .api
            .client
            .send_request(Method::GET, "/employees", Some(&query), None::<()>)
            .await?;
        Ok(resp.employees.unwrap_or_default())
    }
}
