use super::PayrollNzApi;
use crate::error::XeroError;
use crate::models::payroll_nz::pay_slip;
use reqwest::Method;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct PaySlipsResource<'a> {
    api: &'a PayrollNzApi,
}

impl<'a> PaySlipsResource<'a> {
    pub(crate) fn new(api: &'a PayrollNzApi) -> Self {
        Self { api }
    }

    pub fn list(&self) -> PaySlipsListRequest<'a> {
        PaySlipsListRequest::new(self.api)
    }

    pub async fn get(&self, pay_slip_id: Uuid) -> Result<Vec<pay_slip::PaySlip>, XeroError> {
        let path = format!("/paySlips/{pay_slip_id}");
        let resp: pay_slip::PaySlipsResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        Ok(resp.pay_slips.unwrap_or_default())
    }

    pub async fn update(
        &self,
        pay_slip_id: Uuid,
        pay_slip: pay_slip::PaySlip,
    ) -> Result<Vec<pay_slip::PaySlip>, XeroError> {
        let path = format!("/paySlips/{pay_slip_id}");
        let resp: pay_slip::PaySlipsResponse = self
            .api
            .client
            .send_request(Method::PUT, &path, None, Some(pay_slip))
            .await?;
        Ok(resp.pay_slips.unwrap_or_default())
    }
}

#[derive(Debug, Clone)]
pub struct PaySlipsListRequest<'a> {
    api: &'a PayrollNzApi,
    pay_run_id: Option<Uuid>,
    page: Option<u32>,
}

impl<'a> PaySlipsListRequest<'a> {
    fn new(api: &'a PayrollNzApi) -> Self {
        Self {
            api,
            pay_run_id: None,
            page: None,
        }
    }

    pub fn pay_run_id(mut self, pay_run_id: Uuid) -> Self {
        self.pay_run_id = Some(pay_run_id);
        self
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    pub async fn send(self) -> Result<Vec<pay_slip::PaySlip>, XeroError> {
        let mut query = Vec::new();
        if let Some(pay_run_id) = self.pay_run_id {
            query.push(("payrunId".to_string(), pay_run_id.to_string()));
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
