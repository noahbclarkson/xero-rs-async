use super::PayrollUkApi;
use crate::error::XeroError;
use crate::models::payroll_uk::earnings_rate;
use reqwest::Method;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct EarningsRatesResource<'a> {
    api: &'a PayrollUkApi,
}

impl<'a> EarningsRatesResource<'a> {
    pub(crate) fn new(api: &'a PayrollUkApi) -> Self {
        Self { api }
    }

    pub fn list(&self) -> EarningsRatesListRequest<'a> {
        EarningsRatesListRequest::new(self.api)
    }

    pub async fn get(
        &self,
        earnings_rate_id: Uuid,
    ) -> Result<Vec<earnings_rate::EarningsRate>, XeroError> {
        let path = format!("/earningsRates/{earnings_rate_id}");
        let resp: earnings_rate::EarningsRateResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        Ok(resp.earnings_rates.unwrap_or_default())
    }

    pub async fn create(
        &self,
        item: earnings_rate::EarningsRate,
    ) -> Result<Vec<earnings_rate::EarningsRate>, XeroError> {
        let body = earnings_rate::EarningsRateRequest {
            earnings_rates: vec![item],
        };
        let resp: earnings_rate::EarningsRateResponse = self
            .api
            .client
            .send_request(Method::POST, "/earningsRates", None, Some(body))
            .await?;
        Ok(resp.earnings_rates.unwrap_or_default())
    }
}

#[derive(Debug, Clone)]
pub struct EarningsRatesListRequest<'a> {
    api: &'a PayrollUkApi,
    page: Option<u32>,
}

impl<'a> EarningsRatesListRequest<'a> {
    fn new(api: &'a PayrollUkApi) -> Self {
        Self { api, page: None }
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    pub async fn send(self) -> Result<Vec<earnings_rate::EarningsRate>, XeroError> {
        let mut query = Vec::new();
        if let Some(page) = self.page {
            query.push(("page".to_string(), page.to_string()));
        }
        let resp: earnings_rate::EarningsRateResponse = self
            .api
            .client
            .send_request(Method::GET, "/earningsRates", Some(&query), None::<()>)
            .await?;
        Ok(resp.earnings_rates.unwrap_or_default())
    }
}
