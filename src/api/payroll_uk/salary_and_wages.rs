use super::PayrollUkApi;
use crate::error::XeroError;
use crate::models::payroll_uk::salary_and_wages;
use reqwest::Method;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct SalaryAndWagesResource<'a> {
    api: &'a PayrollUkApi,
}

impl<'a> SalaryAndWagesResource<'a> {
    pub(crate) fn new(api: &'a PayrollUkApi) -> Self {
        Self { api }
    }

    pub fn list(&self, employee_id: Uuid) -> SalaryAndWagesListRequest<'a> {
        SalaryAndWagesListRequest::new(self.api, employee_id)
    }

    pub async fn get(
        &self,
        employee_id: Uuid,
        salary_and_wages_id: Uuid,
    ) -> Result<salary_and_wages::SalaryAndWages, XeroError> {
        let path = format!("/employees/{employee_id}/salaryAndWages/{salary_and_wages_id}");
        let resp: salary_and_wages::SalaryAndWagesItemResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        resp.salary_and_wages.ok_or(XeroError::Api {
            status: reqwest::StatusCode::NOT_FOUND,
            message: "Salary and wages not found in response".to_string(),
        })
    }

    pub async fn create(
        &self,
        employee_id: Uuid,
        salary_and_wages: salary_and_wages::SalaryAndWages,
    ) -> Result<salary_and_wages::SalaryAndWages, XeroError> {
        let path = format!("/employees/{employee_id}/salaryAndWages");
        let resp: salary_and_wages::SalaryAndWagesItemResponse = self
            .api
            .client
            .send_request(Method::POST, &path, None, Some(salary_and_wages))
            .await?;
        resp.salary_and_wages.ok_or(XeroError::Api {
            status: reqwest::StatusCode::NOT_FOUND,
            message: "Salary and wages not found in response".to_string(),
        })
    }

    pub async fn update(
        &self,
        employee_id: Uuid,
        salary_and_wages: salary_and_wages::SalaryAndWages,
    ) -> Result<salary_and_wages::SalaryAndWages, XeroError> {
        let path = format!("/employees/{employee_id}/salaryAndWages");
        let resp: salary_and_wages::SalaryAndWagesItemResponse = self
            .api
            .client
            .send_request(Method::PUT, &path, None, Some(salary_and_wages))
            .await?;
        resp.salary_and_wages.ok_or(XeroError::Api {
            status: reqwest::StatusCode::NOT_FOUND,
            message: "Salary and wages not found in response".to_string(),
        })
    }

    pub async fn delete(
        &self,
        employee_id: Uuid,
        salary_and_wages_id: Uuid,
    ) -> Result<(), XeroError> {
        let path = format!("/employees/{employee_id}/salaryAndWages/{salary_and_wages_id}");
        self.api
            .client
            .send_request_empty_response(Method::DELETE, &path, None::<()>)
            .await
    }
}

#[derive(Debug, Clone)]
pub struct SalaryAndWagesListRequest<'a> {
    api: &'a PayrollUkApi,
    employee_id: Uuid,
    page: Option<u32>,
}

impl<'a> SalaryAndWagesListRequest<'a> {
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

    pub async fn send(self) -> Result<Vec<salary_and_wages::SalaryAndWages>, XeroError> {
        let path = format!("/employees/{}/salaryAndWages", self.employee_id);
        let mut query = Vec::new();
        if let Some(page) = self.page {
            query.push(("page".to_string(), page.to_string()));
        }
        let resp: salary_and_wages::SalaryAndWagesResponse = self
            .api
            .client
            .send_request(Method::GET, &path, Some(&query), None::<()>)
            .await?;
        Ok(resp.salary_and_wages.unwrap_or_default())
    }
}
