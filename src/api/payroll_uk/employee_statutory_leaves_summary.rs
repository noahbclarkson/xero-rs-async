use super::PayrollUkApi;
use crate::error::XeroError;
use crate::models::payroll_uk::employee_statutory_leaves_summary;
use reqwest::Method;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct EmployeeStatutoryLeavesSummaryResource<'a> {
    api: &'a PayrollUkApi,
}

impl<'a> EmployeeStatutoryLeavesSummaryResource<'a> {
    pub(crate) fn new(api: &'a PayrollUkApi) -> Self {
        Self { api }
    }

    pub async fn get(
        &self,
        employee_id: Uuid,
        active_only: Option<bool>,
    ) -> Result<Vec<employee_statutory_leaves_summary::StatutoryLeaveSummary>, XeroError> {
        let path = format!("/statutoryleaves/summary/{employee_id}");
        let mut query = Vec::new();
        if let Some(active_only) = active_only {
            query.push(("activeOnly".to_string(), active_only.to_string()));
        }
        let resp: employee_statutory_leaves_summary::StatutoryLeavesSummaryResponse = self
            .api
            .client
            .send_request(Method::GET, &path, Some(&query), None::<()>)
            .await?;
        Ok(resp.statutory_leaves.unwrap_or_default())
    }
}
