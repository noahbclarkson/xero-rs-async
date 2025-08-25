//! The main asynchronous Xero API client.

use crate::auth::{TokenManager, TokenSet};
use crate::endpoints::{
    accounting::AccountingApi,
    assets::AssetsApi,
    files::FilesApi,
    tenanted::{TenantedAccountingApi, TenantedAssetsApi, TenantedFilesApi},
};
use crate::error::XeroError;
use crate::rate_limiter::RateLimiter;

use log::{debug, info};
use reqwest::Client;
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

/// Represents a Xero tenant connection.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Connection {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub tenant_type: String,
    pub tenant_name: Option<String>,
}

/// The main client for interacting with all Xero APIs.
#[derive(Debug, Clone)]
pub struct XeroClient {
    pub(crate) http_client: Client,
    pub token_manager: Arc<TokenManager>,
    pub(crate) rate_limiter: Arc<RateLimiter>,
}

impl XeroClient {
    /// Creates a new `XeroClient`.
    ///
    /// # Arguments
    ///
    /// * `client_id` - Your Xero App's client ID.
    /// * `client_secret` - Your Xero App's client secret.
    /// * `redirect_uri` - The redirect URI configured in your Xero App.
    /// * `rate_limiter` - An Arc-wrapped, shared RateLimiter instance.
    pub async fn new(
        client_id: String,
        client_secret: String,
        redirect_uri: String,
        rate_limiter: Arc<RateLimiter>,
    ) -> Result<Self, XeroError> {
        debug!("Creating new XeroClient instance.");
        let http_client = Client::new();
        let token_manager = Arc::new(TokenManager::new(
            http_client.clone(),
            client_id,
            client_secret,
            redirect_uri,
        ));

        info!("XeroClient created successfully.");
        Ok(Self {
            http_client,
            token_manager,
            rate_limiter,
        })
    }

    /// Retrieves the list of tenants (organisations) connected to the current token.
    pub async fn get_connections(&self) -> Result<Vec<Connection>, XeroError> {
        let url = "https://api.xero.com/connections";
        let response = self
            .http_client
            .get(url)
            .bearer_auth(self.token_manager.get_access_token().await?)
            .header("Accept", "application/json")
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json::<Vec<Connection>>().await?)
        } else {
            let status = response.status();
            let message = response.text().await?;
            Err(XeroError::Api { status, message })
        }
    }

    /// Returns an API handle for the Accounting API endpoints.
    /// This handle requires the `tenant_id` to be passed for each call.
    pub fn accounting(&self) -> AccountingApi {
        AccountingApi::new(self.clone())
    }

    /// Returns an API handle for the Assets API endpoints.
    /// This handle requires the `tenant_id` to be passed for each call.
    pub fn assets(&self) -> AssetsApi {
        AssetsApi::new(self.clone())
    }

    /// Returns an API handle for the Files API endpoints.
    /// This handle requires the `tenant_id` to be passed for each call.
    pub fn files(&self) -> FilesApi {
        FilesApi::new(self.clone())
    }

    /// Returns a convenient API handle for the Accounting API that is bound to a specific tenant.
    pub fn accounting_for_tenant(&self, tenant_id: Uuid) -> TenantedAccountingApi {
        TenantedAccountingApi::new(self.clone(), tenant_id)
    }

    /// Returns a convenient API handle for the Assets API that is bound to a specific tenant.
    pub fn assets_for_tenant(&self, tenant_id: Uuid) -> TenantedAssetsApi {
        TenantedAssetsApi::new(self.clone(), tenant_id)
    }

    /// Returns a convenient API handle for the Files API that is bound to a specific tenant.
    pub fn files_for_tenant(&self, tenant_id: Uuid) -> TenantedFilesApi {
        TenantedFilesApi::new(self.clone(), tenant_id)
    }

    /// Returns a convenient API handle for the Accounting API that is bound to a specific tenant and uses a specific token.
    pub fn accounting_for_tenant_with_token(
        &self,
        tenant_id: Uuid,
        token: TokenSet,
    ) -> TenantedAccountingApi {
        TenantedAccountingApi::with_token(self.clone(), tenant_id, token)
    }

    /// Returns a convenient API handle for the Assets API that is bound to a specific tenant and uses a specific token.
    pub fn assets_for_tenant_with_token(
        &self,
        tenant_id: Uuid,
        token: TokenSet,
    ) -> TenantedAssetsApi {
        TenantedAssetsApi::with_token(self.clone(), tenant_id, token)
    }

    /// Returns a convenient API handle for the Files API that is bound to a specific tenant and uses a specific token.
    pub fn files_for_tenant_with_token(
        &self,
        tenant_id: Uuid,
        token: TokenSet,
    ) -> TenantedFilesApi {
        TenantedFilesApi::with_token(self.clone(), tenant_id, token)
    }
}