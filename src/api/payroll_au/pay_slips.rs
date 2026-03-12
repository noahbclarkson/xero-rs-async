use super::PayrollAuApi;
use crate::error::XeroError;
use crate::models::payroll_au::pay_slip;
use reqwest::Method;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct PaySlipsResource<'a> {
    api: &'a PayrollAuApi,
}

impl<'a> PaySlipsResource<'a> {
    pub(crate) fn new(api: &'a PayrollAuApi) -> Self {
        Self { api }
    }

    /// Retrieves a payslip by ID.
    pub async fn get(&self, payslip_id: Uuid) -> Result<pay_slip::Payslip, XeroError> {
        let path = format!("/Payslip/{payslip_id}");
        let resp: pay_slip::PayslipResponse = self
            .api
            .client_v1
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        resp.payslip.ok_or(XeroError::Api {
            status: reqwest::StatusCode::NOT_FOUND,
            message: "Payslip not found in response".to_string(),
        })
    }

    /// Updates payslip line items.
    pub async fn update(
        &self,
        payslip_id: Uuid,
        payslips: Vec<pay_slip::Payslip>,
    ) -> Result<pay_slip::Payslip, XeroError> {
        let path = format!("/Payslip/{payslip_id}");
        let resp: pay_slip::PayslipResponse = self
            .api
            .client_v1
            .send_request(Method::POST, &path, None, Some(payslips))
            .await?;
        resp.payslip.ok_or(XeroError::Api {
            status: reqwest::StatusCode::NOT_FOUND,
            message: "Payslip not found in response".to_string(),
        })
    }
}
