use super::PayrollNzApi;
use crate::error::XeroError;
use crate::models::payroll_nz::employee_pay_templates;
use reqwest::Method;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct EmployeePayTemplatesResource<'a> {
    api: &'a PayrollNzApi,
}

impl<'a> EmployeePayTemplatesResource<'a> {
    pub(crate) fn new(api: &'a PayrollNzApi) -> Self {
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

    pub async fn update_deduction(
        &self,
        employee_id: Uuid,
        pay_template_deduction_id: Uuid,
        deduction: employee_pay_templates::PayTemplateDeduction,
    ) -> Result<(), XeroError> {
        let path =
            format!("/employees/{employee_id}/paytemplates/deductions/{pay_template_deduction_id}");
        self.api
            .client
            .send_request_empty_response(Method::PUT, &path, Some(deduction))
            .await
    }

    pub async fn delete_deduction(
        &self,
        employee_id: Uuid,
        pay_template_deduction_id: Uuid,
    ) -> Result<(), XeroError> {
        let path =
            format!("/employees/{employee_id}/paytemplates/deductions/{pay_template_deduction_id}");
        self.api
            .client
            .send_request_empty_response(Method::DELETE, &path, None::<()>)
            .await
    }

    pub async fn update_benefit(
        &self,
        employee_id: Uuid,
        pay_template_benefit_id: Uuid,
        benefit: employee_pay_templates::PayTemplateBenefit,
    ) -> Result<(), XeroError> {
        let path =
            format!("/employees/{employee_id}/paytemplates/benefits/{pay_template_benefit_id}");
        self.api
            .client
            .send_request_empty_response(Method::PUT, &path, Some(benefit))
            .await
    }

    pub async fn delete_benefit(
        &self,
        employee_id: Uuid,
        pay_template_benefit_id: Uuid,
    ) -> Result<(), XeroError> {
        let path =
            format!("/employees/{employee_id}/paytemplates/benefits/{pay_template_benefit_id}");
        self.api
            .client
            .send_request_empty_response(Method::DELETE, &path, None::<()>)
            .await
    }

    pub async fn update_reimbursement(
        &self,
        employee_id: Uuid,
        pay_template_reimbursement_id: Uuid,
        reimbursement: employee_pay_templates::PayTemplateReimbursement,
    ) -> Result<(), XeroError> {
        let path = format!(
            "/employees/{employee_id}/paytemplates/reimbursements/{pay_template_reimbursement_id}"
        );
        self.api
            .client
            .send_request_empty_response(Method::PUT, &path, Some(reimbursement))
            .await
    }

    pub async fn delete_reimbursement(
        &self,
        employee_id: Uuid,
        pay_template_reimbursement_id: Uuid,
    ) -> Result<(), XeroError> {
        let path = format!(
            "/employees/{employee_id}/paytemplates/reimbursements/{pay_template_reimbursement_id}"
        );
        self.api
            .client
            .send_request_empty_response(Method::DELETE, &path, None::<()>)
            .await
    }
}
