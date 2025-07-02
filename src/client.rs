//! The main asynchronous Xero API client.

use crate::auth::TokenManager;
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
use std::path::PathBuf;
use std::sync::Arc;
use uuid::Uuid;
// Duplicate imports removed above. Only one import per item remains.

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
    /// * `token_cache_path` - Path to a file for caching OAuth tokens.
    /// * `rate_limiter` - An Arc-wrapped, shared RateLimiter instance.
    pub async fn new(
        client_id: String,
        client_secret: String,
        redirect_uri: String,
        token_cache_path: PathBuf,
        rate_limiter: Arc<RateLimiter>,
    ) -> Result<Self, XeroError> {
        debug!("Creating new XeroClient instance.");
        let http_client = Client::new();
        let token_manager = Arc::new(TokenManager::new(
            http_client.clone(),
            client_id,
            client_secret,
            redirect_uri,
            token_cache_path,
        ));

        info!("XeroClient created successfully.");
        Ok(Self {
            http_client,
            token_manager,
            rate_limiter,
        })
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
}
