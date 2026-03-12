use super::PayrollUkApi;
use crate::error::XeroError;
use crate::models::payroll_uk::employee_statutory_sick_leave;
use reqwest::Method;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct EmployeeStatutorySickLeaveResource<'a> {
    api: &'a PayrollUkApi,
}

impl<'a> EmployeeStatutorySickLeaveResource<'a> {
    pub(crate) fn new(api: &'a PayrollUkApi) -> Self {
        Self { api }
    }

    pub async fn get(
        &self,
        statutory_sick_leave_id: Uuid,
    ) -> Result<employee_statutory_sick_leave::StatutorySickLeave, XeroError> {
        let path = format!("/statutoryleaves/sick/{statutory_sick_leave_id}");
        let resp: employee_statutory_sick_leave::StatutorySickLeaveResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        resp.statutory_sick_leave.ok_or(XeroError::Api {
            status: reqwest::StatusCode::NOT_FOUND,
            message: "Statutory sick leave not found in response".to_string(),
        })
    }

    pub async fn create(
        &self,
        statutory_sick_leave: employee_statutory_sick_leave::StatutorySickLeave,
    ) -> Result<employee_statutory_sick_leave::StatutorySickLeave, XeroError> {
        let resp: employee_statutory_sick_leave::StatutorySickLeaveResponse = self
            .api
            .client
            .send_request(
                Method::POST,
                "/statutoryleaves/sick",
                None,
                Some(statutory_sick_leave),
            )
            .await?;
        resp.statutory_sick_leave.ok_or(XeroError::Api {
            status: reqwest::StatusCode::NOT_FOUND,
            message: "Statutory sick leave not found in response".to_string(),
        })
    }
}
