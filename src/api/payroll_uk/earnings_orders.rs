use super::PayrollUkApi;
use crate::error::XeroError;
use crate::models::payroll_uk::earnings_order;
use reqwest::Method;

#[derive(Debug, Clone, Copy)]
pub struct EarningsOrdersResource<'a> {
    api: &'a PayrollUkApi,
}

impl<'a> EarningsOrdersResource<'a> {
    pub(crate) fn new(api: &'a PayrollUkApi) -> Self {
        Self { api }
    }

    pub fn list(&self) -> EarningsOrdersListRequest<'a> {
        EarningsOrdersListRequest::new(self.api)
    }
}

#[derive(Debug, Clone)]
pub struct EarningsOrdersListRequest<'a> {
    api: &'a PayrollUkApi,
    page: Option<u32>,
}

impl<'a> EarningsOrdersListRequest<'a> {
    fn new(api: &'a PayrollUkApi) -> Self {
        Self { api, page: None }
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    pub async fn send(self) -> Result<Vec<earnings_order::EarningsOrder>, XeroError> {
        let mut query = Vec::new();
        if let Some(page) = self.page {
            query.push(("page".to_string(), page.to_string()));
        }
        let resp: earnings_order::EarningsOrderResponse = self
            .api
            .client
            .send_request(Method::GET, "/earningsOrders", Some(&query), None::<()>)
            .await?;
        Ok(resp.earnings_orders.unwrap_or_default())
    }
}
