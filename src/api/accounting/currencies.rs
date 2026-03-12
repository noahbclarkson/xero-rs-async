use super::AccountingApi;
use crate::error::XeroError;
use crate::models::accounting::currency;
use reqwest::Method;

/// Resource accessor for Currencies.
#[derive(Debug, Clone, Copy)]
pub struct CurrenciesResource<'a> {
    api: &'a AccountingApi,
}

impl<'a> CurrenciesResource<'a> {
    pub(crate) fn new(api: &'a AccountingApi) -> Self {
        Self { api }
    }

    /// Retrieves all currencies.
    pub async fn list(&self) -> Result<Vec<currency::Currency>, XeroError> {
        let resp: currency::CurrenciesResponse = self
            .api
            .client
            .send_request(Method::GET, "/Currencies", None, None::<()>)
            .await?;
        Ok(resp.currencies)
    }

    /// Creates a new currency.
    pub async fn create(
        &self,
        currency: currency::Currency,
    ) -> Result<Vec<currency::Currency>, XeroError> {
        let resp: currency::CurrenciesResponse = self
            .api
            .client
            .send_request(Method::PUT, "/Currencies", None, Some(currency))
            .await?;
        Ok(resp.currencies)
    }
}

impl AccountingApi {
    /// Retrieves all currencies.
    pub async fn get_currencies(
        &self,
        _where_filter: Option<String>,
        _order_by: Option<String>,
    ) -> Result<Vec<currency::Currency>, XeroError> {
        self.currencies().list().await
    }

    /// Creates a new currency.
    pub async fn create_currency(
        &self,
        currency: currency::Currency,
    ) -> Result<Vec<currency::Currency>, XeroError> {
        self.currencies().create(currency).await
    }
}
