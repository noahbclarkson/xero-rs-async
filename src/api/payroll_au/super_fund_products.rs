use super::PayrollAuApi;
use crate::error::XeroError;
use crate::models::payroll_au::super_fund_product;
use reqwest::Method;

#[derive(Debug, Clone, Copy)]
pub struct SuperFundProductsResource<'a> {
    api: &'a PayrollAuApi,
}

impl<'a> SuperFundProductsResource<'a> {
    pub(crate) fn new(api: &'a PayrollAuApi) -> Self {
        Self { api }
    }

    /// Searches for super fund products by ABN and/or USI.
    pub async fn search(
        &self,
        abn: Option<String>,
        usi: Option<String>,
    ) -> Result<Vec<super_fund_product::SuperFundProduct>, XeroError> {
        let mut query = Vec::new();
        if let Some(abn) = abn {
            query.push(("ABN".to_string(), abn));
        }
        if let Some(usi) = usi {
            query.push(("USI".to_string(), usi));
        }
        let resp: super_fund_product::SuperFundProductsResponse = self
            .api
            .client_v1
            .send_request(Method::GET, "/SuperFundProducts", Some(&query), None::<()>)
            .await?;
        Ok(resp.super_fund_products)
    }
}
