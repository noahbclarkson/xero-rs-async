use super::PayrollAuApi;
use crate::error::XeroError;
use crate::models::payroll_au::pay_item;
use reqwest::Method;

#[derive(Debug, Clone, Copy)]
pub struct PayItemsResource<'a> {
    api: &'a PayrollAuApi,
}

impl<'a> PayItemsResource<'a> {
    pub(crate) fn new(api: &'a PayrollAuApi) -> Self {
        Self { api }
    }

    /// Retrieves payroll pay items.
    pub async fn get(&self) -> Result<pay_item::PayItems, XeroError> {
        let resp: pay_item::PayItemsResponse = self
            .api
            .client_v1
            .send_request(Method::GET, "/PayItems", None, None::<()>)
            .await?;
        resp.pay_items.ok_or(XeroError::Api {
            status: reqwest::StatusCode::NOT_FOUND,
            message: "PayItems not found in response".to_string(),
        })
    }

    /// Creates or updates payroll pay items.
    pub async fn upsert(
        &self,
        pay_items: pay_item::PayItems,
    ) -> Result<pay_item::PayItems, XeroError> {
        let body = pay_item::PayItemsRequest { pay_items };
        let resp: pay_item::PayItemsResponse = self
            .api
            .client_v1
            .send_request(Method::POST, "/PayItems", None, Some(body))
            .await?;
        resp.pay_items.ok_or(XeroError::Api {
            status: reqwest::StatusCode::NOT_FOUND,
            message: "PayItems not found in response".to_string(),
        })
    }
}
