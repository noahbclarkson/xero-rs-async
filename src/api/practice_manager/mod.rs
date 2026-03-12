//! API client for the Xero Practice Manager API v3.1.

use crate::auth::TokenSet;
use crate::client::XeroClient;
use crate::http::ApiClient;
use std::sync::Arc;
use uuid::Uuid;

pub mod categories;
pub mod client_groups;
pub mod clients;
pub mod costs;
pub mod custom_fields;
pub mod invoices;
pub mod jobs;
pub mod quotes;
pub mod staff;
pub mod tasks;
pub mod templates;
pub mod time;

const BASE_URL: &str = "https://api.xero.com/practicemanager/3.1";

#[derive(Debug, Clone)]
pub struct PracticeManagerApi {
    pub(crate) client: ApiClient,
}

impl PracticeManagerApi {
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

    /// Returns a Clients resource handle.
    #[must_use]
    pub fn clients(&self) -> clients::ClientsResource<'_> {
        clients::ClientsResource::new(self)
    }

    /// Returns a Client Groups resource handle.
    #[must_use]
    pub fn client_groups(&self) -> client_groups::ClientGroupsResource<'_> {
        client_groups::ClientGroupsResource::new(self)
    }

    /// Returns a Staff resource handle.
    #[must_use]
    pub fn staff(&self) -> staff::StaffResource<'_> {
        staff::StaffResource::new(self)
    }

    /// Returns a Jobs resource handle.
    #[must_use]
    pub fn jobs(&self) -> jobs::JobsResource<'_> {
        jobs::JobsResource::new(self)
    }

    /// Returns a Tasks resource handle.
    #[must_use]
    pub fn tasks(&self) -> tasks::TasksResource<'_> {
        tasks::TasksResource::new(self)
    }

    /// Returns a Time resource handle.
    #[must_use]
    pub fn time(&self) -> time::TimeResource<'_> {
        time::TimeResource::new(self)
    }

    /// Returns an Invoices resource handle.
    #[must_use]
    pub fn invoices(&self) -> invoices::InvoicesResource<'_> {
        invoices::InvoicesResource::new(self)
    }

    /// Returns a Quotes resource handle.
    #[must_use]
    pub fn quotes(&self) -> quotes::QuotesResource<'_> {
        quotes::QuotesResource::new(self)
    }

    /// Returns a Costs resource handle.
    #[must_use]
    pub fn costs(&self) -> costs::CostsResource<'_> {
        costs::CostsResource::new(self)
    }

    /// Returns a Categories resource handle.
    #[must_use]
    pub fn categories(&self) -> categories::CategoriesResource<'_> {
        categories::CategoriesResource::new(self)
    }

    /// Returns a Custom Fields resource handle.
    #[must_use]
    pub fn custom_fields(&self) -> custom_fields::CustomFieldsResource<'_> {
        custom_fields::CustomFieldsResource::new(self)
    }

    /// Returns a Templates resource handle.
    #[must_use]
    pub fn templates(&self) -> templates::TemplatesResource<'_> {
        templates::TemplatesResource::new(self)
    }
}
