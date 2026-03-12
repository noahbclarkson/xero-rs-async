use super::query::QueryParams;
use super::AccountingApi;
use crate::error::XeroError;
use crate::models::accounting::employee;
use chrono::{DateTime, Utc};
use reqwest::Method;
use uuid::Uuid;

/// Resource accessor for Employees.
#[derive(Debug, Clone, Copy)]
pub struct EmployeesResource<'a> {
    api: &'a AccountingApi,
}

impl<'a> EmployeesResource<'a> {
    pub(crate) fn new(api: &'a AccountingApi) -> Self {
        Self { api }
    }

    /// Builds a request to list employees.
    pub fn list(&self) -> EmployeesListRequest<'a> {
        EmployeesListRequest::new(self.api)
    }

    /// Retrieves a single employee by ID.
    pub async fn get(&self, employee_id: Uuid) -> Result<Vec<employee::Employee>, XeroError> {
        let path = format!("/Employees/{employee_id}");
        let resp: employee::EmployeesResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        Ok(resp.employees)
    }

    /// Creates one or more new employees.
    pub async fn create(
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
            .client
            .send_request(Method::PUT, "/Employees", None, Some(body))
            .await?;
        Ok(resp.employees)
    }

    /// Updates an existing employee.
    pub async fn update(
        &self,
        employee_id: Uuid,
        employee_data: employee::Employee,
    ) -> Result<Vec<employee::Employee>, XeroError> {
        let path = format!("/Employees/{employee_id}");
        let resp: employee::EmployeesResponse = self
            .api
            .client
            .send_request(Method::POST, &path, None, Some(employee_data))
            .await?;
        Ok(resp.employees)
    }
}

/// Builder for Employees list requests.
#[derive(Debug, Clone)]
pub struct EmployeesListRequest<'a> {
    api: &'a AccountingApi,
    where_filter: Option<String>,
    order_by: Option<String>,
}

impl<'a> EmployeesListRequest<'a> {
    fn new(api: &'a AccountingApi) -> Self {
        Self {
            api,
            where_filter: None,
            order_by: None,
        }
    }

    /// Filter using the `where` query parameter.
    pub fn where_filter(mut self, filter: impl Into<String>) -> Self {
        self.where_filter = Some(filter.into());
        self
    }

    /// Order by a field.
    pub fn order_by(mut self, order: impl Into<String>) -> Self {
        self.order_by = Some(order.into());
        self
    }

    /// Executes the list request.
    pub async fn send(self) -> Result<Vec<employee::Employee>, XeroError> {
        let mut query = QueryParams::default();
        query.push_opt_string("where", self.where_filter);
        query.push_opt_string("order", self.order_by);

        let resp: employee::EmployeesResponse = self
            .api
            .client
            .send_request(Method::GET, "/Employees", query.as_slice(), None::<()>)
            .await?;
        Ok(resp.employees)
    }
}

impl AccountingApi {
    /// Retrieves one or many employees.
    pub async fn get_employees(
        &self,
        employee_id: Option<Uuid>,
        _modified_after: Option<DateTime<Utc>>,
        where_filter: Option<String>,
        order_by: Option<String>,
    ) -> Result<Vec<employee::Employee>, XeroError> {
        if let Some(id) = employee_id {
            self.employees().get(id).await
        } else {
            let mut request = self.employees().list();
            if let Some(filter) = where_filter {
                request = request.where_filter(filter);
            }
            if let Some(order) = order_by {
                request = request.order_by(order);
            }
            request.send().await
        }
    }

    /// Creates one or more new employees.
    pub async fn create_employees(
        &self,
        employees: Vec<employee::Employee>,
    ) -> Result<Vec<employee::Employee>, XeroError> {
        self.employees().create(employees).await
    }

    /// Updates an existing employee.
    pub async fn update_employee(
        &self,
        employee_id: Uuid,
        employee_data: employee::Employee,
    ) -> Result<Vec<employee::Employee>, XeroError> {
        self.employees().update(employee_id, employee_data).await
    }
}
