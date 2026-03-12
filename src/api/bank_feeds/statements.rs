use super::BankFeedsApi;
use crate::error::XeroError;
use crate::models::bank_feeds::statement;
use reqwest::Method;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct StatementsResource<'a> {
    api: &'a BankFeedsApi,
}

impl<'a> StatementsResource<'a> {
    pub(crate) fn new(api: &'a BankFeedsApi) -> Self {
        Self { api }
    }

    pub fn list(&self) -> StatementsListRequest<'a> {
        StatementsListRequest::new(self.api)
    }

    pub async fn get(&self, statement_id: Uuid) -> Result<statement::Statement, XeroError> {
        let path = format!("/Statements/{statement_id}");
        self.api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await
    }

    pub async fn create(
        &self,
        statements: Vec<statement::Statement>,
    ) -> Result<statement::StatementsCreateResponse, XeroError> {
        let body = statement::StatementsCreateRequest { items: statements };
        self.api
            .client
            .send_request(Method::POST, "/Statements", None, Some(body))
            .await
    }
}

#[derive(Debug, Clone)]
pub struct StatementsListRequest<'a> {
    api: &'a BankFeedsApi,
    page: Option<u32>,
    page_size: Option<u32>,
}

impl<'a> StatementsListRequest<'a> {
    fn new(api: &'a BankFeedsApi) -> Self {
        Self {
            api,
            page: None,
            page_size: None,
        }
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub async fn send(self) -> Result<Vec<statement::Statement>, XeroError> {
        let mut query = Vec::new();
        if let Some(page) = self.page {
            query.push(("page".to_string(), page.to_string()));
        }
        if let Some(page_size) = self.page_size {
            query.push(("pageSize".to_string(), page_size.to_string()));
        }
        let resp: statement::StatementsResponse = self
            .api
            .client
            .send_request(Method::GET, "/Statements", Some(&query), None::<()>)
            .await?;
        Ok(resp.items.unwrap_or_default())
    }
}
