//! Entry point for interacting with the Xero Assets API.

use crate::client::XeroClient;
use crate::error::XeroError;
use crate::models::assets::{
    asset::{Asset, AssetStatus, AssetsResponse},
    asset_type::AssetType,
    settings::Settings,
};
use log::{error, trace};
use reqwest::Method;
use serde::de::DeserializeOwned;
use serde::Serialize;
use uuid::Uuid;

const BASE_URL: &str = "https://api.xero.com/assets.xro/1.0";

/// A handle to the Assets API endpoints.
#[derive(Debug, Clone)]
pub struct AssetsApi {
    client: XeroClient,
}

/// Private helper methods for the Assets API.
impl AssetsApi {
    async fn send_request<R, B>(
        &self,
        method: Method,
        tenant_id: Uuid,
        path: &str,
        query: Option<&[(String, String)]>,
        body: Option<B>,
    ) -> Result<R, XeroError>
    where
        R: DeserializeOwned,
        B: Serialize,
    {
        let url = format!("{}{}", BASE_URL, path);
        let mut builder = self
            .client
            .http_client
            .request(method, &url)
            .bearer_auth(self.client.token_manager.get_access_token().await?)
            .header("xero-tenant-id", tenant_id.to_string())
            .header("Accept", "application/json");

        if let Some(q) = query {
            builder = builder.query(q);
        }
        if let Some(b) = body {
            builder = builder.json(&b);
        }

        let _permit = self.client.rate_limiter.acquire_permit(tenant_id).await?;
        let response = builder.send().await?;

        if response.status().is_success() {
            let response_text = response.text().await?;
            serde_json::from_str::<R>(&response_text).map_err(|e| {
                error!("Failed to deserialize JSON response from {}: {}", url, e);
                trace!(
                    "Raw JSON response that failed to parse:\n---\n{}\n---",
                    response_text
                );
                XeroError::from(e)
            })
        } else {
            let status = response.status();
            let message = response.text().await?;
            Err(XeroError::Api { status, message })
        }
    }
}

impl AssetsApi {
    pub(crate) fn new(client: XeroClient) -> Self {
        Self { client }
    }

    /// Retrieves a list of asset types.
    pub async fn get_asset_types(&self, tenant_id: Uuid) -> Result<Vec<AssetType>, XeroError> {
        self.send_request(Method::GET, tenant_id, "/AssetTypes", None, None::<()>)
            .await
    }

    /// Creates a new asset type.
    pub async fn create_asset_type(
        &self,
        tenant_id: Uuid,
        asset_type: AssetType,
    ) -> Result<AssetType, XeroError> {
        self.send_request(
            Method::POST,
            tenant_id,
            "/AssetTypes",
            None,
            Some(asset_type),
        )
        .await
    }

    /// Retrieves a list of assets.
    #[allow(clippy::too_many_arguments)]
    pub async fn get_assets(
        &self,
        tenant_id: Uuid,
        status: AssetStatus,
        page: Option<u32>,
        page_size: Option<u32>,
        order_by: Option<String>,
        sort_direction: Option<String>,
        filter_by: Option<String>,
    ) -> Result<Vec<Asset>, XeroError> {
        let mut query = vec![("status".to_string(), format!("{:?}", status).to_uppercase())];
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
            .send_request(Method::GET, tenant_id, "/Assets", Some(&query), None::<()>)
            .await?;
        Ok(resp.items)
    }

    /// Retrieves a single asset by its ID.
    pub async fn get_asset_by_id(
        &self,
        tenant_id: Uuid,
        asset_id: Uuid,
    ) -> Result<Asset, XeroError> {
        let path = format!("/Assets/{}", asset_id);
        self.send_request(Method::GET, tenant_id, &path, None, None::<()>)
            .await
    }

    /// Creates a new draft fixed asset.
    pub async fn create_asset(&self, tenant_id: Uuid, asset: Asset) -> Result<Asset, XeroError> {
        self.send_request(Method::POST, tenant_id, "/Assets", None, Some(asset))
            .await
    }

    /// Retrieves the organisation's fixed asset settings.
    pub async fn get_asset_settings(&self, tenant_id: Uuid) -> Result<Settings, XeroError> {
        self.send_request(Method::GET, tenant_id, "/Settings", None, None::<()>)
            .await
    }
}
