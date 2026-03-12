//! API client for the Projects API.

use crate::auth::TokenSet;
use crate::client::XeroClient;
use crate::http::ApiClient;
use std::sync::Arc;
use uuid::Uuid;

pub mod projects;
pub mod tasks;
pub mod time_entries;
pub mod users;

const BASE_URL: &str = "https://api.xero.com/projects.xro/2.0";

#[derive(Debug, Clone)]
pub struct ProjectsApi {
    client: ApiClient,
}

impl ProjectsApi {
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
    pub fn projects(&self) -> projects::ProjectsResource<'_> {
        projects::ProjectsResource::new(self)
    }

    #[must_use]
    pub fn tasks(&self) -> tasks::TasksResource<'_> {
        tasks::TasksResource::new(self)
    }

    #[must_use]
    pub fn time_entries(&self) -> time_entries::TimeEntriesResource<'_> {
        time_entries::TimeEntriesResource::new(self)
    }

    #[must_use]
    pub fn users(&self) -> users::UsersResource<'_> {
        users::UsersResource::new(self)
    }
}
