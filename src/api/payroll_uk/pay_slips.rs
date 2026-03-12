use super::PayrollUkApi;
use crate::error::XeroError;
use crate::models::payroll_uk::pay_slip;
use reqwest::Method;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct PaySlipsResource<'a> {
    api: &'a PayrollUkApi,
}

impl<'a> PaySlipsResource<'a> {
    pub(crate) fn new(api: &'a PayrollUkApi) -> Self {
        Self { api }
    }

    pub fn list(&self) -> PaySlipsListRequest<'a> {
        PaySlipsListRequest::new(self.api)
    }

    pub async fn get(&self, payslip_id: Uuid) -> Result<pay_slip::PaySlip, XeroError> {
        let path = format!("/paySlips/{payslip_id}");
        let resp: pay_slip::PaySlipResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        resp.pay_slip.ok_or(XeroError::Api {
            status: reqwest::StatusCode::NOT_FOUND,
            message: "Payslip not found in response".to_string(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct PaySlipsListRequest<'a> {
    api: &'a PayrollUkApi,
    payrun_id: Option<Uuid>,
    page: Option<u32>,
}

impl<'a> PaySlipsListRequest<'a> {
    fn new(api: &'a PayrollUkApi) -> Self {
        Self {
            api,
            payrun_id: None,
            page: None,
        }
    }

    pub fn payrun_id(mut self, payrun_id: Uuid) -> Self {
        self.payrun_id = Some(payrun_id);
        self
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    pub async fn send(self) -> Result<Vec<pay_slip::PaySlip>, XeroError> {
        let mut query = Vec::new();
        if let Some(payrun_id) = self.payrun_id {
            query.push(("payrunId".to_string(), payrun_id.to_string()));
        }
        if let Some(page) = self.page {
            query.push(("page".to_string(), page.to_string()));
        }
        let resp: pay_slip::PaySlipsResponse = self
            .api
            .client
            .send_request(Method::GET, "/paySlips", Some(&query), None::<()>)
            .await?;
        Ok(resp.pay_slips.unwrap_or_default())
    }
}
