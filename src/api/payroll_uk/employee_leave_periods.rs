use super::PayrollUkApi;
use crate::error::XeroError;
use crate::models::payroll_uk::employee_leave_period;
use reqwest::Method;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct EmployeeLeavePeriodsResource<'a> {
    api: &'a PayrollUkApi,
}

impl<'a> EmployeeLeavePeriodsResource<'a> {
    pub(crate) fn new(api: &'a PayrollUkApi) -> Self {
        Self { api }
    }

    pub async fn list(
        &self,
        employee_id: Uuid,
        start_date: Option<String>,
        end_date: Option<String>,
    ) -> Result<Vec<employee_leave_period::EmployeeLeavePeriod>, XeroError> {
        let path = format!("/employees/{employee_id}/leavePeriods");
        let mut query = Vec::new();
        if let Some(start_date) = start_date {
            query.push(("startDate".to_string(), start_date));
        }
        if let Some(end_date) = end_date {
            query.push(("endDate".to_string(), end_date));
        }
        let resp: employee_leave_period::LeavePeriodResponse = self
            .api
            .client
            .send_request(Method::GET, &path, Some(&query), None::<()>)
            .await?;
        Ok(resp.leave_periods.unwrap_or_default())
    }
}
