use super::PayrollAuApi;
use crate::error::XeroError;
use crate::models::payroll_au::super_fund;
use reqwest::Method;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct SuperFundsResource<'a> {
    api: &'a PayrollAuApi,
}

impl<'a> SuperFundsResource<'a> {
    pub(crate) fn new(api: &'a PayrollAuApi) -> Self {
        Self { api }
    }

    pub fn list(&self) -> SuperFundsListRequest<'a> {
        SuperFundsListRequest::new(self.api)
    }

    pub async fn get(&self, super_fund_id: Uuid) -> Result<Vec<super_fund::SuperFund>, XeroError> {
        let path = format!("/SuperFunds/{super_fund_id}");
        let resp: super_fund::SuperFundsResponse = self
            .api
            .client_v1
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        Ok(resp.super_funds)
    }

    pub async fn upsert(
        &self,
        funds: Vec<super_fund::SuperFund>,
    ) -> Result<Vec<super_fund::SuperFund>, XeroError> {
        let body = super_fund::SuperFundsRequest { super_funds: funds };
        let resp: super_fund::SuperFundsResponse = self
            .api
            .client_v1
            .send_request(Method::POST, "/SuperFunds", None, Some(body))
            .await?;
        Ok(resp.super_funds)
    }
}

#[derive(Debug, Clone)]
pub struct SuperFundsListRequest<'a> {
    api: &'a PayrollAuApi,
    page: Option<u32>,
    where_filter: Option<String>,
    order: Option<String>,
}

impl<'a> SuperFundsListRequest<'a> {
    fn new(api: &'a PayrollAuApi) -> Self {
        Self {
            api,
            page: None,
            where_filter: None,
            order: None,
        }
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    pub fn where_filter(mut self, filter: impl Into<String>) -> Self {
        self.where_filter = Some(filter.into());
        self
    }

    pub fn order(mut self, order: impl Into<String>) -> Self {
        self.order = Some(order.into());
        self
    }

    pub async fn send(self) -> Result<Vec<super_fund::SuperFund>, XeroError> {
        let mut query = Vec::new();
        if let Some(page) = self.page {
            query.push(("page".to_string(), page.to_string()));
        }
        if let Some(filter) = self.where_filter {
            query.push(("where".to_string(), filter));
        }
        if let Some(order) = self.order {
            query.push(("order".to_string(), order));
        }

        let resp: super_fund::SuperFundsResponse = self
            .api
            .client_v1
            .send_request(Method::GET, "/SuperFunds", Some(&query), None::<()>)
            .await?;
        Ok(resp.super_funds)
    }
}
