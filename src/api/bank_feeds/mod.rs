//! API client for the Bank Feeds API.

use crate::auth::TokenSet;
use crate::client::XeroClient;
use crate::http::ApiClient;
use std::sync::Arc;
use uuid::Uuid;

pub mod feed_connections;
pub mod statements;

const BASE_URL: &str = "https://api.xero.com/bankfeeds.xro/1.0";

#[derive(Debug, Clone)]
pub struct BankFeedsApi {
    client: ApiClient,
}

impl BankFeedsApi {
    pub(crate) fn new(client: XeroClient, tenant_id: Uuid) -> Self {
        Self {
            client: ApiClient::new(
                BASE_URL,
                tenant_id,
                client.http_client.clone(),
                client.token_manager.clone(),
                client.rate_limiter.clone(),
            ),
        }
    }

    pub(crate) fn with_token_override(mut self, token: Arc<TokenSet>) -> Self {
        self.client = self.client.with_token_override(token);
        self
    }

    #[must_use]
    pub fn feed_connections(&self) -> feed_connections::FeedConnectionsResource<'_> {
        feed_connections::FeedConnectionsResource::new(self)
    }

    #[must_use]
    pub fn statements(&self) -> statements::StatementsResource<'_> {
        statements::StatementsResource::new(self)
    }
}
