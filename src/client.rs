//! The main asynchronous Xero API client.

#[cfg(feature = "accounting")]
use crate::api::accounting::AccountingApi;
#[cfg(feature = "bank-feeds")]
use crate::api::bank_feeds::BankFeedsApi;
#[cfg(feature = "payroll-au")]
use crate::api::payroll_au::PayrollAuApi;
#[cfg(feature = "payroll-nz")]
use crate::api::payroll_nz::PayrollNzApi;
#[cfg(feature = "payroll-uk")]
use crate::api::payroll_uk::PayrollUkApi;
#[cfg(feature = "practice-manager")]
use crate::api::practice_manager::PracticeManagerApi;
#[cfg(feature = "projects")]
use crate::api::projects::ProjectsApi;
use crate::auth::{TokenManager, TokenSet};
#[cfg(feature = "assets")]
use crate::endpoints::assets::AssetsApi;
#[cfg(feature = "files")]
use crate::endpoints::files::FilesApi;
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

/// A tenant-bound client that vends API handles without requiring tenant IDs per call.
#[derive(Debug, Clone)]
pub struct TenantClient {
    client: XeroClient,
    tenant_id: Uuid,
    token_override: Option<Arc<TokenSet>>,
}

impl TenantClient {
    pub(crate) fn new(client: XeroClient, tenant_id: Uuid) -> Self {
        Self {
            client,
            tenant_id,
            token_override: None,
        }
    }

    pub(crate) fn with_token(client: XeroClient, tenant_id: Uuid, token: TokenSet) -> Self {
        Self {
            client,
            tenant_id,
            token_override: Some(Arc::new(token)),
        }
    }

    /// Returns an Accounting API handle bound to this tenant.
    #[must_use]
    #[cfg(feature = "accounting")]
    pub fn accounting(&self) -> AccountingApi {
        let api = AccountingApi::new(self.client.clone(), self.tenant_id);
        if let Some(token) = &self.token_override {
            api.with_token_override(token.clone())
        } else {
            api
        }
    }

    /// Returns an Assets API handle bound to this tenant.
    #[must_use]
    #[cfg(feature = "assets")]
    pub fn assets(&self) -> AssetsApi {
        let api = AssetsApi::new(self.client.clone(), self.tenant_id);
        if let Some(token) = &self.token_override {
            api.with_token_override(token.clone())
        } else {
            api
        }
    }

    /// Returns a Files API handle bound to this tenant.
    #[must_use]
    #[cfg(feature = "files")]
    pub fn files(&self) -> FilesApi {
        let api = FilesApi::new(self.client.clone(), self.tenant_id);
        if let Some(token) = &self.token_override {
            api.with_token_override(token.clone())
        } else {
            api
        }
    }

    /// Returns a Payroll AU API handle bound to this tenant.
    #[must_use]
    #[cfg(feature = "payroll-au")]
    pub fn payroll_au(&self) -> PayrollAuApi {
        let api = PayrollAuApi::new(self.client.clone(), self.tenant_id);
        if let Some(token) = &self.token_override {
            api.with_token_override(token.clone())
        } else {
            api
        }
    }

    /// Returns a Payroll UK API handle bound to this tenant.
    #[must_use]
    #[cfg(feature = "payroll-uk")]
    pub fn payroll_uk(&self) -> PayrollUkApi {
        let api = PayrollUkApi::new(self.client.clone(), self.tenant_id);
        if let Some(token) = &self.token_override {
            api.with_token_override(token.clone())
        } else {
            api
        }
    }

    /// Returns a Payroll NZ API handle bound to this tenant.
    #[must_use]
    #[cfg(feature = "payroll-nz")]
    pub fn payroll_nz(&self) -> PayrollNzApi {
        let api = PayrollNzApi::new(self.client.clone(), self.tenant_id);
        if let Some(token) = &self.token_override {
            api.with_token_override(token.clone())
        } else {
            api
        }
    }

    /// Returns a Projects API handle bound to this tenant.
    #[must_use]
    #[cfg(feature = "projects")]
    pub fn projects(&self) -> ProjectsApi {
        let api = ProjectsApi::new(self.client.clone(), self.tenant_id);
        if let Some(token) = &self.token_override {
            api.with_token_override(token.clone())
        } else {
            api
        }
    }

    /// Returns a Bank Feeds API handle bound to this tenant.
    #[must_use]
    #[cfg(feature = "bank-feeds")]
    pub fn bank_feeds(&self) -> BankFeedsApi {
        let api = BankFeedsApi::new(self.client.clone(), self.tenant_id);
        if let Some(token) = &self.token_override {
            api.with_token_override(token.clone())
        } else {
            api
        }
    }

    /// Returns a Practice Manager API handle bound to this tenant.
    #[must_use]
    #[cfg(feature = "practice-manager")]
    pub fn practice_manager(&self) -> PracticeManagerApi {
        let api = PracticeManagerApi::new(self.client.clone(), self.tenant_id);
        if let Some(token) = &self.token_override {
            api.with_token_override(token.clone())
        } else {
            api
        }
    }
}

impl XeroClient {
    /// Creates a new `XeroClient`.
    ///
    /// # Arguments
    ///
    /// * `client_id` - Your Xero App's client ID.
    /// * `client_secret` - Your Xero App's client secret.
    /// * `redirect_uri` - The redirect URI configured in your Xero App.
    /// * `rate_limiter` - An Arc-wrapped, shared `RateLimiter` instance.
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

    /// Creates a new `XeroClient` with an isolated `TokenManager` pre-seeded with the given token.
    ///
    /// Unlike `new`, this constructor does not share a `TokenManager` with any other client.
    /// Use this when you need per-job token isolation (e.g., concurrent jobs for different tenants).
    ///
    /// # Arguments
    ///
    /// * `client_id` - Your Xero App's client ID.
    /// * `client_secret` - Your Xero App's client secret.
    /// * `redirect_uri` - The redirect URI configured in your Xero App.
    /// * `rate_limiter` - A shared `RateLimiter` instance (can be the same one used by other clients).
    /// * `initial_token` - A `TokenSet` to pre-seed the new `TokenManager` with.
    pub async fn new_with_token(
        client_id: String,
        client_secret: String,
        redirect_uri: String,
        rate_limiter: Arc<RateLimiter>,
        initial_token: TokenSet,
    ) -> Result<Self, XeroError> {
        debug!("Creating new XeroClient instance with pre-seeded token.");
        let http_client = Client::new();
        let token_manager = Arc::new(TokenManager::new(
            http_client.clone(),
            client_id,
            client_secret,
            redirect_uri,
        ));
        token_manager.set_token(&initial_token).await;

        info!("XeroClient created successfully with pre-seeded token.");
        Ok(Self {
            http_client,
            token_manager,
            rate_limiter,
        })
    }

    /// Retrieves the list of tenants (organisations) connected to the current token.
    pub async fn get_connections(&self) -> Result<Vec<Connection>, XeroError> {
        let access_token = self.token_manager.get_access_token().await?;
        self.get_connections_with_access_token(&access_token).await
    }

    /// Retrieves the list of tenants (organisations) for an explicit access token.
    pub async fn get_connections_with_access_token(
        &self,
        access_token: &str,
    ) -> Result<Vec<Connection>, XeroError> {
        let url = "https://api.xero.com/connections";
        let response = self
            .http_client
            .get(url)
            .bearer_auth(access_token)
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

    /// Returns a clone of the shared `RateLimiter` `Arc` used by this client.
    ///
    /// This is useful when constructing a new isolated client (via `new_with_token`)
    /// that should share the same rate-limiting budget as the global client.
    #[must_use]
    pub fn rate_limiter(&self) -> Arc<RateLimiter> {
        self.rate_limiter.clone()
    }

    /// Returns a tenant-bound client that vends API handles.
    #[must_use]
    pub fn tenant(&self, tenant_id: Uuid) -> TenantClient {
        TenantClient::new(self.clone(), tenant_id)
    }

    /// Returns a tenant-bound client that uses a specific token.
    #[must_use]
    pub fn tenant_with_token(&self, tenant_id: Uuid, token: TokenSet) -> TenantClient {
        TenantClient::with_token(self.clone(), tenant_id, token)
    }

    /// Returns a convenient API handle for the Accounting API that is bound to a specific tenant.
    #[must_use]
    #[cfg(feature = "accounting")]
    pub fn accounting_for_tenant(&self, tenant_id: Uuid) -> AccountingApi {
        self.tenant(tenant_id).accounting()
    }

    /// Returns a convenient API handle for the Assets API that is bound to a specific tenant.
    #[must_use]
    #[cfg(feature = "assets")]
    pub fn assets_for_tenant(&self, tenant_id: Uuid) -> AssetsApi {
        self.tenant(tenant_id).assets()
    }

    /// Returns a convenient API handle for the Files API that is bound to a specific tenant.
    #[must_use]
    #[cfg(feature = "files")]
    pub fn files_for_tenant(&self, tenant_id: Uuid) -> FilesApi {
        self.tenant(tenant_id).files()
    }

    /// Returns a convenient API handle for the Payroll AU API that is bound to a specific tenant.
    #[must_use]
    #[cfg(feature = "payroll-au")]
    pub fn payroll_au_for_tenant(&self, tenant_id: Uuid) -> PayrollAuApi {
        self.tenant(tenant_id).payroll_au()
    }

    /// Returns a convenient API handle for the Payroll UK API that is bound to a specific tenant.
    #[must_use]
    #[cfg(feature = "payroll-uk")]
    pub fn payroll_uk_for_tenant(&self, tenant_id: Uuid) -> PayrollUkApi {
        self.tenant(tenant_id).payroll_uk()
    }

    /// Returns a convenient API handle for the Payroll NZ API that is bound to a specific tenant.
    #[must_use]
    #[cfg(feature = "payroll-nz")]
    pub fn payroll_nz_for_tenant(&self, tenant_id: Uuid) -> PayrollNzApi {
        self.tenant(tenant_id).payroll_nz()
    }

    /// Returns a convenient API handle for the Projects API that is bound to a specific tenant.
    #[must_use]
    #[cfg(feature = "projects")]
    pub fn projects_for_tenant(&self, tenant_id: Uuid) -> ProjectsApi {
        self.tenant(tenant_id).projects()
    }

    /// Returns a convenient API handle for the Bank Feeds API that is bound to a specific tenant.
    #[must_use]
    #[cfg(feature = "bank-feeds")]
    pub fn bank_feeds_for_tenant(&self, tenant_id: Uuid) -> BankFeedsApi {
        self.tenant(tenant_id).bank_feeds()
    }

    /// Returns a convenient API handle for the Accounting API that is bound to a specific tenant and uses a specific token.
    #[must_use]
    #[cfg(feature = "accounting")]
    pub fn accounting_for_tenant_with_token(
        &self,
        tenant_id: Uuid,
        token: TokenSet,
    ) -> AccountingApi {
        self.tenant_with_token(tenant_id, token).accounting()
    }

    /// Returns a convenient API handle for the Assets API that is bound to a specific tenant and uses a specific token.
    #[must_use]
    #[cfg(feature = "assets")]
    pub fn assets_for_tenant_with_token(&self, tenant_id: Uuid, token: TokenSet) -> AssetsApi {
        self.tenant_with_token(tenant_id, token).assets()
    }

    /// Returns a convenient API handle for the Files API that is bound to a specific tenant and uses a specific token.
    #[must_use]
    #[cfg(feature = "files")]
    pub fn files_for_tenant_with_token(&self, tenant_id: Uuid, token: TokenSet) -> FilesApi {
        self.tenant_with_token(tenant_id, token).files()
    }

    /// Returns a convenient API handle for the Payroll AU API that is bound to a specific tenant and uses a specific token.
    #[must_use]
    #[cfg(feature = "payroll-au")]
    pub fn payroll_au_for_tenant_with_token(
        &self,
        tenant_id: Uuid,
        token: TokenSet,
    ) -> PayrollAuApi {
        self.tenant_with_token(tenant_id, token).payroll_au()
    }

    /// Returns a convenient API handle for the Payroll UK API that is bound to a specific tenant and uses a specific token.
    #[must_use]
    #[cfg(feature = "payroll-uk")]
    pub fn payroll_uk_for_tenant_with_token(
        &self,
        tenant_id: Uuid,
        token: TokenSet,
    ) -> PayrollUkApi {
        self.tenant_with_token(tenant_id, token).payroll_uk()
    }

    /// Returns a convenient API handle for the Payroll NZ API that is bound to a specific tenant and uses a specific token.
    #[must_use]
    #[cfg(feature = "payroll-nz")]
    pub fn payroll_nz_for_tenant_with_token(
        &self,
        tenant_id: Uuid,
        token: TokenSet,
    ) -> PayrollNzApi {
        self.tenant_with_token(tenant_id, token).payroll_nz()
    }

    /// Returns a convenient API handle for the Projects API that is bound to a specific tenant and uses a specific token.
    #[must_use]
    #[cfg(feature = "projects")]
    pub fn projects_for_tenant_with_token(&self, tenant_id: Uuid, token: TokenSet) -> ProjectsApi {
        self.tenant_with_token(tenant_id, token).projects()
    }

    /// Returns a convenient API handle for the Bank Feeds API that is bound to a specific tenant and uses a specific token.
    #[must_use]
    #[cfg(feature = "bank-feeds")]
    pub fn bank_feeds_for_tenant_with_token(
        &self,
        tenant_id: Uuid,
        token: TokenSet,
    ) -> BankFeedsApi {
        self.tenant_with_token(tenant_id, token).bank_feeds()
    }

    /// Returns a convenient API handle for the Practice Manager API that is bound to a specific tenant.
    #[must_use]
    #[cfg(feature = "practice-manager")]
    pub fn practice_manager_for_tenant(&self, tenant_id: Uuid) -> PracticeManagerApi {
        self.tenant(tenant_id).practice_manager()
    }

    /// Returns a convenient API handle for the Practice Manager API that is bound to a specific tenant and uses a specific token.
    #[must_use]
    #[cfg(feature = "practice-manager")]
    pub fn practice_manager_for_tenant_with_token(
        &self,
        tenant_id: Uuid,
        token: TokenSet,
    ) -> PracticeManagerApi {
        self.tenant_with_token(tenant_id, token).practice_manager()
    }
}
