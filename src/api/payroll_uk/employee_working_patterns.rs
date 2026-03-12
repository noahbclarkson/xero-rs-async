use super::PayrollUkApi;
use crate::error::XeroError;
use crate::models::payroll_uk::working_pattern;
use reqwest::Method;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct EmployeeWorkingPatternsResource<'a> {
    api: &'a PayrollUkApi,
}

impl<'a> EmployeeWorkingPatternsResource<'a> {
    pub(crate) fn new(api: &'a PayrollUkApi) -> Self {
        Self { api }
    }

    pub async fn list_for_employee(
        &self,
        employee_id: Uuid,
    ) -> Result<Vec<working_pattern::WorkingPattern>, XeroError> {
        let path = format!("/employees/{employee_id}/working-patterns");
        let resp: working_pattern::WorkingPatternResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        Ok(resp.working_patterns.unwrap_or_default())
    }

    pub async fn get_for_employee(
        &self,
        employee_id: Uuid,
        working_pattern_id: Uuid,
    ) -> Result<Vec<working_pattern::WorkingPattern>, XeroError> {
        let path = format!("/employees/{employee_id}/working-patterns/{working_pattern_id}");
        let resp: working_pattern::WorkingPatternResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        Ok(resp.working_patterns.unwrap_or_default())
    }

    pub async fn create_for_employee(
        &self,
        employee_id: Uuid,
        working_pattern: working_pattern::WorkingPattern,
    ) -> Result<Vec<working_pattern::WorkingPattern>, XeroError> {
        let path = format!("/employees/{employee_id}/working-patterns");
        let body = working_pattern::WorkingPatternRequest {
            working_patterns: vec![working_pattern],
        };
        let resp: working_pattern::WorkingPatternResponse = self
            .api
            .client
            .send_request(Method::POST, &path, None, Some(body))
            .await?;
        Ok(resp.working_patterns.unwrap_or_default())
    }

    pub async fn delete_for_employee(
        &self,
        employee_id: Uuid,
        working_pattern_id: Uuid,
    ) -> Result<(), XeroError> {
        let path = format!("/employees/{employee_id}/working-patterns/{working_pattern_id}");
        self.api
            .client
            .send_request_empty_response(Method::DELETE, &path, None::<()>)
            .await
    }

    pub async fn list_global(&self) -> Result<Vec<working_pattern::WorkingPattern>, XeroError> {
        let resp: working_pattern::WorkingPatternResponse = self
            .api
            .client
            .send_request(Method::GET, "/working-patterns", None, None::<()>)
            .await?;
        Ok(resp.working_patterns.unwrap_or_default())
    }

    pub async fn create_global(
        &self,
        working_pattern: working_pattern::WorkingPattern,
    ) -> Result<Vec<working_pattern::WorkingPattern>, XeroError> {
        let body = working_pattern::WorkingPatternRequest {
            working_patterns: vec![working_pattern],
        };
        let resp: working_pattern::WorkingPatternResponse = self
            .api
            .client
            .send_request(Method::POST, "/working-patterns", None, Some(body))
            .await?;
        Ok(resp.working_patterns.unwrap_or_default())
    }

    pub async fn delete_global(&self, working_pattern_id: Uuid) -> Result<(), XeroError> {
        let path = format!("/working-patterns/{working_pattern_id}");
        self.api
            .client
            .send_request_empty_response(Method::DELETE, &path, None::<()>)
            .await
    }
}
