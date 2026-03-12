use super::query::QueryParams;
use super::AccountingApi;
use crate::error::XeroError;
use crate::models::accounting::tax_rate;
use reqwest::Method;

/// Resource accessor for Tax Rates.
#[derive(Debug, Clone, Copy)]
pub struct TaxRatesResource<'a> {
    api: &'a AccountingApi,
}

impl<'a> TaxRatesResource<'a> {
    pub(crate) fn new(api: &'a AccountingApi) -> Self {
        Self { api }
    }

    /// Builds a request to list tax rates.
    pub fn list(&self) -> TaxRatesListRequest<'a> {
        TaxRatesListRequest::new(self.api)
    }

    /// Creates or updates tax rates.
    pub async fn create_or_update(
        &self,
        tax_rates: Vec<tax_rate::TaxRate>,
    ) -> Result<Vec<tax_rate::TaxRate>, XeroError> {
        let body = tax_rate::TaxRatesRequest { tax_rates };
        let resp: tax_rate::TaxRatesResponse = self
            .api
            .client
            .send_request(Method::PUT, "/TaxRates", None, Some(body))
            .await?;
        Ok(resp.tax_rates)
    }
}

/// Builder for Tax Rates list requests.
#[derive(Debug, Clone)]
pub struct TaxRatesListRequest<'a> {
    api: &'a AccountingApi,
    where_filter: Option<String>,
    order_by: Option<String>,
}

impl<'a> TaxRatesListRequest<'a> {
    fn new(api: &'a AccountingApi) -> Self {
        Self {
            api,
            where_filter: None,
            order_by: None,
        }
    }

    /// Filter using the `where` query parameter.
    pub fn where_filter(mut self, filter: impl Into<String>) -> Self {
        self.where_filter = Some(filter.into());
        self
    }

    /// Order by a field.
    pub fn order_by(mut self, order: impl Into<String>) -> Self {
        self.order_by = Some(order.into());
        self
    }

    /// Executes the list request.
    pub async fn send(self) -> Result<Vec<tax_rate::TaxRate>, XeroError> {
        let mut query = QueryParams::default();
        query.push_opt_string("where", self.where_filter);
        query.push_opt_string("order", self.order_by);

        let resp: tax_rate::TaxRatesResponse = self
            .api
            .client
            .send_request(Method::GET, "/TaxRates", query.as_slice(), None::<()>)
            .await?;
        Ok(resp.tax_rates)
    }
}

impl AccountingApi {
    /// Retrieves tax rates.
    pub async fn get_tax_rates(
        &self,
        where_filter: Option<String>,
        order_by: Option<String>,
    ) -> Result<Vec<tax_rate::TaxRate>, XeroError> {
        let mut request = self.tax_rates().list();
        if let Some(filter) = where_filter {
            request = request.where_filter(filter);
        }
        if let Some(order) = order_by {
            request = request.order_by(order);
        }
        request.send().await
    }

    /// Creates or updates a tax rate.
    pub async fn create_or_update_tax_rate(
        &self,
        tax_rate: tax_rate::TaxRate,
    ) -> Result<Vec<tax_rate::TaxRate>, XeroError> {
        self.tax_rates().create_or_update(vec![tax_rate]).await
    }
}
