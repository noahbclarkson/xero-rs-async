//! Contains convenient API handles that are bound to a specific tenant ID.

use crate::auth::TokenSet;
use crate::client::XeroClient;
use crate::error::XeroError;
use crate::models::accounting::{
    common::Allocation,
    contact::{CISSettings as ContactCISSettings, Contact},
    invoice::{Invoice, OnlineInvoice},
    organisation::{CISSettings as OrgCISSettings, Organisation, OrganisationAction},
    purchase_order::PurchaseOrder,
    quote::Quote,
    repeating_invoice::RepeatingInvoice,
    report::Report,
    tax_rate,
    tracking_category::{TrackingCategory, TrackingOption},
    *,
};
use crate::models::assets::{asset, asset_type, settings};
use crate::models::files::{association, file, folder};
use chrono::{DateTime, Utc};
use log::{debug, error, trace};
use reqwest::Method;
use serde::{de::DeserializeOwned, Serialize};
use uuid::Uuid;

const ACCOUNTING_BASE_URL: &str = "https://api.xero.com/api.xro/2.0";
const ASSETS_BASE_URL: &str = "https://api.xero.com/assets.xro/1.0";
const FILES_BASE_URL: &str = "https://api.xero.com/files.xro/1.0";

// A generic helper to reduce boilerplate in the tenanted API implementations.
async fn send_tenanted_request<R, B>(
    client: &XeroClient,
    tenant_id: Uuid,
    token_override: &Option<TokenSet>,
    method: Method,
    base_url: &str,
    path: &str,
    query: Option<&[(String, String)]>,
    body: Option<B>,
) -> Result<R, XeroError>
where
    R: DeserializeOwned,
    B: Serialize,
{
    let access_token = if let Some(token) = token_override {
        token.access_token.clone()
    } else {
        client.token_manager.get_access_token().await?
    };

    let url = format!("{}{}", base_url, path);
    debug!("Sending tenanted API request: {} {}", method, url);

    let mut builder = client
        .http_client
        .request(method, &url)
        .bearer_auth(access_token)
        .header("xero-tenant-id", tenant_id.to_string())
        .header("Accept", "application/json");

    if let Some(q) = query {
        builder = builder.query(q);
    }
    if let Some(b) = body {
        builder = builder.json(&b);
    }

    let _permit = client.rate_limiter.acquire_permit(tenant_id).await?;
    trace!("Rate limiter permit acquired for tenant {}", tenant_id);
    let response = builder.send().await?;

    if response.status().is_success() {
        let response_text = response.text().await?;
        serde_json::from_str::<R>(&response_text).map_err(|e| {
            error!("Failed to deserialize JSON response from {}: {}", url, e);
            debug!(
                "Raw JSON response that failed to parse:\n---\n{}\n---",
                response_text.chars().take(10_000).collect::<String>()
            );
            XeroError::from(e)
        })
    } else {
        let status = response.status();
        let message = response.text().await?;
        error!(
            "API request failed with status: {}. Message: {}",
            status, message
        );
        Err(XeroError::Api { status, message })
    }
}

/// A handle to the Accounting API endpoints, bound to a specific tenant.
#[derive(Debug, Clone)]
pub struct TenantedAccountingApi {
    client: XeroClient,
    tenant_id: Uuid,
    token_override: Option<TokenSet>,
}

impl TenantedAccountingApi {
    pub(crate) fn new(
        client: XeroClient,
        tenant_id: Uuid,
        token_override: Option<TokenSet>,
    ) -> Self {
        Self {
            client,
            tenant_id,
            token_override,
        }
    }

    async fn send_request<R, B>(
        &self,
        method: Method,
        path: &str,
        query: Option<&[(String, String)]>,
        body: Option<B>,
    ) -> Result<R, XeroError>
    where
        R: DeserializeOwned,
        B: Serialize,
    {
        send_tenanted_request(
            &self.client,
            self.tenant_id,
            &self.token_override,
            method,
            ACCOUNTING_BASE_URL,
            path,
            query,
            body,
        )
        .await
    }

    // --- Accounts ---
    pub async fn get_accounts(
        &self,
        account_id: Option<Uuid>,
        _modified_after: Option<DateTime<Utc>>,
        where_filter: Option<String>,
        order_by: Option<String>,
    ) -> Result<Vec<account::Account>, XeroError> {
        let path = if let Some(id) = account_id {
            format!("/Accounts/{}", id)
        } else {
            "/Accounts".to_string()
        };
        let mut query = Vec::new();
        if let Some(filter) = where_filter {
            query.push(("where".to_string(), filter));
        }
        if let Some(order) = order_by {
            query.push(("order".to_string(), order));
        }
        let resp: account::AccountsResponse = self
            .send_request(Method::GET, &path, Some(&query), None::<()>)
            .await?;
        Ok(resp.accounts)
    }

    // --- Invoices ---
    #[allow(clippy::too_many_arguments)]
    pub async fn get_invoices(
        &self,
        invoice_id: Option<Uuid>,
        invoice_numbers: Option<Vec<String>>,
        contact_ids: Option<Vec<Uuid>>,
        statuses: Option<Vec<String>>,
        _modified_after: Option<DateTime<Utc>>,
        where_filter: Option<String>,
        order_by: Option<String>,
        page: Option<u32>,
        page_size: Option<u32>,
        summary_only: Option<bool>,
        search_term: Option<String>,
    ) -> Result<Vec<invoice::Invoice>, XeroError> {
        let path = if let Some(id) = invoice_id {
            format!("/Invoices/{}", id)
        } else {
            "/Invoices".to_string()
        };
        let mut query = Vec::new();
        if let Some(nums) = invoice_numbers {
            query.push(("InvoiceNumbers".to_string(), nums.join(",")));
        }
        if let Some(cids) = contact_ids {
            query.push((
                "ContactIDs".to_string(),
                cids.iter()
                    .map(Uuid::to_string)
                    .collect::<Vec<_>>()
                    .join(","),
            ));
        }
        if let Some(stats) = statuses {
            query.push(("Statuses".to_string(), stats.join(",")));
        }
        if let Some(filter) = where_filter {
            query.push(("where".to_string(), filter));
        }
        if let Some(order) = order_by {
            query.push(("order".to_string(), order));
        }
        if let Some(p) = page {
            query.push(("page".to_string(), p.to_string()));
        }
        if let Some(ps) = page_size {
            query.push(("pageSize".to_string(), ps.to_string()));
        }
        if let Some(so) = summary_only {
            query.push(("summaryOnly".to_string(), so.to_string()));
        }
        if let Some(st) = search_term {
            query.push(("SearchTerm".to_string(), st));
        }
        let resp: invoice::InvoicesResponse = self
            .send_request(Method::GET, &path, Some(&query), None::<()>)
            .await?;
        Ok(resp.invoices)
    }

    // --- History & Notes ---
    pub async fn get_history(
        &self,
        endpoint: &str,
        guid: Uuid,
    ) -> Result<Vec<history::HistoryRecord>, XeroError> {
        let path = format!("/{}/{}/history", endpoint, guid);
        let resp: history::HistoryRecordsResponse = self
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        Ok(resp.history_records)
    }

    // --- Organisation ---
    pub async fn get_organisation(&self) -> Result<Vec<Organisation>, XeroError> {
        let resp: organisation::OrganisationsResponse = self
            .send_request(Method::GET, "/Organisation", None, None::<()>)
            .await?;
        Ok(resp.organisations)
    }

    // --- Reports ---
    pub async fn get_report(
        &self,
        report_name: &str,
        params: Vec<(&str, &str)>,
    ) -> Result<Report, XeroError> {
        let path = format!("/Reports/{}", report_name);
        let query: Vec<(String, String)> = params
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        let resp: report::ReportsResponse = self
            .send_request(Method::GET, &path, Some(&query), None::<()>)
            .await?;
        resp.reports
            .into_iter()
            .next()
            .ok_or_else(|| XeroError::Api {
                status: reqwest::StatusCode::NOT_FOUND,
                message: "Report not found in response".to_string(),
            })
    }

    // --- Online Invoice URL ---
    pub async fn get_online_invoice_url(
        &self,
        invoice_id: Uuid,
    ) -> Result<OnlineInvoice, XeroError> {
        let path = format!("/Invoices/{}/OnlineInvoice", invoice_id);
        let mut resp: invoice::OnlineInvoicesResponse = self
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        resp.online_invoices.pop().ok_or(XeroError::Api {
            status: reqwest::StatusCode::NOT_FOUND,
            message: "OnlineInvoice not found in response".to_string(),
        })
    }

}

/// A handle to the Assets API endpoints, bound to a specific tenant.
#[derive(Debug, Clone)]
pub struct TenantedAssetsApi {
    client: XeroClient,
    tenant_id: Uuid,
}

impl TenantedAssetsApi {
    pub(crate) fn new(client: XeroClient, tenant_id: Uuid) -> Self {
        Self { client, tenant_id }
    }

    pub async fn get_asset_settings(&self) -> Result<settings::Settings, XeroError> {
        self.client.assets().get_asset_settings(self.tenant_id).await
    }

    pub async fn get_asset_types(&self) -> Result<Vec<asset_type::AssetType>, XeroError> {
        self.client.assets().get_asset_types(self.tenant_id).await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn get_assets(
        &self,
        status: asset::AssetStatus,
        page: Option<u32>,
        page_size: Option<u32>,
        order_by: Option<String>,
        sort_direction: Option<String>,
        filter_by: Option<String>,
    ) -> Result<Vec<asset::Asset>, XeroError> {
        self.client
            .assets()
            .get_assets(
                self.tenant_id,
                status,
                page,
                page_size,
                order_by,
                sort_direction,
                filter_by,
            )
            .await
    }

    pub async fn get_asset_by_id(&self, asset_id: Uuid) -> Result<asset::Asset, XeroError> {
        self.client
            .assets()
            .get_asset_by_id(self.tenant_id, asset_id)
            .await
    }
}

/// A handle to the Files API endpoints, bound to a specific tenant.
#[derive(Debug, Clone)]
pub struct TenantedFilesApi {
    client: XeroClient,
    tenant_id: Uuid,
}

impl TenantedFilesApi {
    pub(crate) fn new(client: XeroClient, tenant_id: Uuid) -> Self {
        Self { client, tenant_id }
    }

    pub async fn get_folders(&self, sort: Option<String>) -> Result<Vec<folder::Folder>, XeroError> {
        self.client.files().get_folders(self.tenant_id, sort).await
    }

    pub async fn get_folder_by_id(&self, folder_id: Uuid) -> Result<folder::Folder, XeroError> {
        self.client
            .files()
            .get_folder_by_id(self.tenant_id, folder_id)
            .await
    }

    pub async fn get_files(
        &self,
        page_size: Option<u32>,
        page: Option<u32>,
        sort: Option<String>,
        direction: Option<String>,
    ) -> Result<Vec<file::File>, XeroError> {
        self.client
            .files()
            .get_files(self.tenant_id, page_size, page, sort, direction)
            .await
    }

    pub async fn get_file_by_id(&self, file_id: Uuid) -> Result<file::File, XeroError> {
        self.client
            .files()
            .get_file_by_id(self.tenant_id, file_id)
            .await
    }

    pub async fn get_file_content(&self, file_id: Uuid) -> Result<Vec<u8>, XeroError> {
        self.client
            .files()
            .get_file_content(self.tenant_id, file_id)
            .await
    }

    pub async fn get_object_associations(
        &self,
        object_id: Uuid,
    ) -> Result<Vec<association::Association>, XeroError> {
        self.client
            .files()
            .get_object_associations(self.tenant_id, object_id)
            .await
    }

    pub async fn get_associations_count(
        &self,
        object_ids: Vec<Uuid>,
    ) -> Result<association::AssociationCount, XeroError> {
        self.client
            .files()
            .get_associations_count(self.tenant_id, object_ids)
            .await
    }

    pub async fn get_file_associations(
        &self,
        file_id: Uuid,
    ) -> Result<Vec<association::Association>, XeroError> {
        self.client
            .files()
            .get_file_associations(self.tenant_id, file_id)
            .await
    }
}