use super::PayrollAuApi;
use crate::error::XeroError;
use crate::models::payroll_au::earnings_rate;
use reqwest::Method;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct EarningsRatesResource<'a> {
    api: &'a PayrollAuApi,
}

impl<'a> EarningsRatesResource<'a> {
    pub(crate) fn new(api: &'a PayrollAuApi) -> Self {
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
        let resp: earnings_rate::EarningsRatesResponse = self
            .api
            .client_v2
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        Ok(resp.earnings_rates.unwrap_or_default())
    }

    pub async fn create(
        &self,
        earnings_rate: earnings_rate::EarningsRate,
    ) -> Result<Vec<earnings_rate::EarningsRate>, XeroError> {
        self.create_many(vec![earnings_rate]).await
    }

    pub async fn create_many(
        &self,
        earnings_rates: Vec<earnings_rate::EarningsRate>,
    ) -> Result<Vec<earnings_rate::EarningsRate>, XeroError> {
        let body = earnings_rate::EarningsRatesRequest { earnings_rates };
        let resp: earnings_rate::EarningsRatesResponse = self
            .api
            .client_v2
            .send_request(Method::POST, "/earningsRates", None, Some(body))
            .await?;
        Ok(resp.earnings_rates.unwrap_or_default())
    }
}

#[derive(Debug, Clone)]
pub struct EarningsRatesListRequest<'a> {
    api: &'a PayrollAuApi,
    page: Option<u32>,
}

impl<'a> EarningsRatesListRequest<'a> {
    fn new(api: &'a PayrollAuApi) -> Self {
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
        let resp: earnings_rate::EarningsRatesResponse = self
            .api
            .client_v2
            .send_request(Method::GET, "/earningsRates", Some(&query), None::<()>)
            .await?;
        Ok(resp.earnings_rates.unwrap_or_default())
    }
}
