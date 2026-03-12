//! Entry point for interacting with the Xero Assets API.

use crate::auth::TokenSet;
use crate::client::XeroClient;
use crate::error::XeroError;
use crate::http::ApiClient;
use crate::models::assets::{
    asset::{Asset, AssetStatus, AssetsResponse},
    asset_type::{AssetType, AssetTypesResponse},
    settings::Settings,
};
use reqwest::Method;
use std::sync::Arc;
use uuid::Uuid;

const BASE_URL: &str = "https://api.xero.com/assets.xro/1.0";

/// A handle to the Assets API endpoints.
#[derive(Debug, Clone)]
pub struct AssetsApi {
    client: ApiClient,
}

impl AssetsApi {
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

    /// Retrieves a list of asset types.
    pub async fn get_asset_types(&self) -> Result<Vec<AssetType>, XeroError> {
        let resp: AssetTypesResponse = self
            .client
            .send_request(Method::GET, "/AssetTypes", None, None::<()>)
            .await?;
        Ok(resp.into_vec())
    }

    /// Creates a new asset type.
    pub async fn create_asset_type(&self, asset_type: AssetType) -> Result<AssetType, XeroError> {
        self.client
            .send_request(Method::POST, "/AssetTypes", None, Some(asset_type))
            .await
    }

    /// Retrieves a list of assets.
    #[allow(clippy::too_many_arguments)]
    pub async fn get_assets(
        &self,
        status: AssetStatus,
        page: Option<u32>,
        page_size: Option<u32>,
        order_by: Option<String>,
        sort_direction: Option<String>,
        filter_by: Option<String>,
    ) -> Result<Vec<Asset>, XeroError> {
        let mut query = vec![("status".to_string(), format!("{status:?}").to_uppercase())];
        if let Some(p) = page {
            query.push(("page".to_string(), p.to_string()));
        }
        if let Some(ps) = page_size {
            query.push(("pageSize".to_string(), ps.to_string()));
        }
        if let Some(o) = order_by {
            query.push(("orderBy".to_string(), o));
        }
        if let Some(sd) = sort_direction {
            query.push(("sortDirection".to_string(), sd));
        }
        if let Some(f) = filter_by {
            query.push(("filterBy".to_string(), f));
        }

        let resp: AssetsResponse = self
            .client
            .send_request(Method::GET, "/Assets", Some(&query), None::<()>)
            .await?;
        Ok(resp.items)
    }

    /// Retrieves a single asset by its ID.
    pub async fn get_asset_by_id(&self, asset_id: Uuid) -> Result<Asset, XeroError> {
        let path = format!("/Assets/{asset_id}");
        self.client
            .send_request(Method::GET, &path, None, None::<()>)
            .await
    }

    /// Creates a new draft fixed asset.
    pub async fn create_asset(&self, asset: Asset) -> Result<Asset, XeroError> {
        self.client
            .send_request(Method::POST, "/Assets", None, Some(asset))
            .await
    }

    /// Retrieves the organisation's fixed asset settings.
    pub async fn get_asset_settings(&self) -> Result<Settings, XeroError> {
        self.client
            .send_request(Method::GET, "/Settings", None, None::<()>)
            .await
    }

    /// Debug method: Returns the raw JSON response from the Assets API.
    /// Use this to diagnose deserialization issues.
    pub async fn debug_get_assets_raw(&self, status: AssetStatus) -> Result<String, XeroError> {
        let query = vec![("status".to_string(), format!("{status:?}").to_uppercase())];
        self.client
            .send_request_text(Method::GET, "/Assets", Some(&query))
            .await
    }
}
