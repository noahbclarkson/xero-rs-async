use super::BankFeedsApi;
use crate::error::XeroError;
use crate::models::bank_feeds::feed_connection;
use reqwest::Method;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct FeedConnectionsResource<'a> {
    api: &'a BankFeedsApi,
}

impl<'a> FeedConnectionsResource<'a> {
    pub(crate) fn new(api: &'a BankFeedsApi) -> Self {
        Self { api }
    }

    pub fn list(&self) -> FeedConnectionsListRequest<'a> {
        FeedConnectionsListRequest::new(self.api)
    }

    pub async fn get(
        &self,
        feed_connection_id: Uuid,
    ) -> Result<feed_connection::FeedConnection, XeroError> {
        let path = format!("/FeedConnections/{feed_connection_id}");
        self.api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await
    }

    pub async fn create(
        &self,
        items: Vec<feed_connection::FeedConnectionCreate>,
    ) -> Result<feed_connection::FeedConnectionsCreateResponse, XeroError> {
        let body = feed_connection::FeedConnectionsCreateRequest { items };
        self.api
            .client
            .send_request(Method::POST, "/FeedConnections", None, Some(body))
            .await
    }

    pub async fn delete(
        &self,
        items: Vec<feed_connection::FeedConnectionDeleteRequestItem>,
    ) -> Result<feed_connection::FeedConnectionsDeleteResponse, XeroError> {
        let body = feed_connection::FeedConnectionsDeleteRequest { items };
        self.api
            .client
            .send_request(
                Method::POST,
                "/FeedConnections/DeleteRequests",
                None,
                Some(body),
            )
            .await
    }
}

#[derive(Debug, Clone)]
pub struct FeedConnectionsListRequest<'a> {
    api: &'a BankFeedsApi,
    page: Option<u32>,
    page_size: Option<u32>,
}

impl<'a> FeedConnectionsListRequest<'a> {
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

    pub async fn send(self) -> Result<Vec<feed_connection::FeedConnection>, XeroError> {
        let mut query = Vec::new();
        if let Some(page) = self.page {
            query.push(("page".to_string(), page.to_string()));
        }
        if let Some(page_size) = self.page_size {
            query.push(("pageSize".to_string(), page_size.to_string()));
        }
        let resp: feed_connection::FeedConnectionsResponse = self
            .api
            .client
            .send_request(Method::GET, "/FeedConnections", Some(&query), None::<()>)
            .await?;
        Ok(resp.items.unwrap_or_default())
    }
}
