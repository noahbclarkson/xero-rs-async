use super::PayrollUkApi;
use crate::error::XeroError;
use crate::models::payroll_uk::employee_pay_templates;
use reqwest::Method;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct EmployeePayTemplatesResource<'a> {
    api: &'a PayrollUkApi,
}

impl<'a> EmployeePayTemplatesResource<'a> {
    pub(crate) fn new(api: &'a PayrollUkApi) -> Self {
        Self { api }
    }

    pub async fn get(
        &self,
        employee_id: Uuid,
    ) -> Result<employee_pay_templates::PayTemplatesResponse, XeroError> {
        let path = format!("/employees/{employee_id}/paytemplates");
        self.api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await
    }

    pub async fn create_earning(
        &self,
        employee_id: Uuid,
        earning: employee_pay_templates::PayTemplateEarning,
    ) -> Result<employee_pay_templates::PayTemplateEarning, XeroError> {
        let path = format!("/employees/{employee_id}/paytemplates/earnings");
        let resp: employee_pay_templates::PayTemplateEarningResponse = self
            .api
            .client
            .send_request(Method::POST, &path, None, Some(earning))
            .await?;
        resp.earning_template.ok_or(XeroError::Api {
            status: reqwest::StatusCode::NOT_FOUND,
            message: "Earning template not found in response".to_string(),
        })
    }

    pub async fn create_earnings(
        &self,
        employee_id: Uuid,
        earnings: Vec<employee_pay_templates::PayTemplateEarning>,
    ) -> Result<employee_pay_templates::PayTemplatesResponse, XeroError> {
        let path = format!("/employees/{employee_id}/paytemplateearnings");
        self.api
            .client
            .send_request(Method::POST, &path, None, Some(earnings))
            .await
    }

    pub async fn update_earning(
        &self,
        employee_id: Uuid,
        pay_template_earning_id: Uuid,
        earning: employee_pay_templates::PayTemplateEarning,
    ) -> Result<employee_pay_templates::PayTemplateEarning, XeroError> {
        let path =
            format!("/employees/{employee_id}/paytemplates/earnings/{pay_template_earning_id}");
        let resp: employee_pay_templates::PayTemplateEarningResponse = self
            .api
            .client
            .send_request(Method::PUT, &path, None, Some(earning))
            .await?;
        resp.earning_template.ok_or(XeroError::Api {
            status: reqwest::StatusCode::NOT_FOUND,
            message: "Earning template not found in response".to_string(),
        })
    }

    pub async fn delete_earning(
        &self,
        employee_id: Uuid,
        pay_template_earning_id: Uuid,
    ) -> Result<(), XeroError> {
        let path =
            format!("/employees/{employee_id}/paytemplates/earnings/{pay_template_earning_id}");
        self.api
            .client
            .send_request_empty_response(Method::DELETE, &path, None::<()>)
            .await
    }
}
