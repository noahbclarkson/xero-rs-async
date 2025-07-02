//! Model for the Item resource.

use crate::util::xero_date_format_opt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Item {
    #[serde(rename = "ItemID", skip_serializing_if = "Option::is_none")]
    pub item_id: Option<Uuid>,
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_sold: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_purchased: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_details: Option<ItemDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales_details: Option<ItemDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_tracked_as_inventory: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_asset_account_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_cost_pool: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity_on_hand: Option<f64>,
    #[serde(with = "xero_date_format_opt", default, rename = "UpdatedDateUTC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_date_utc: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ItemDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_code: Option<String>,
    #[serde(rename = "COGSAccountCode", skip_serializing_if = "Option::is_none")]
    pub cogs_account_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_type: Option<String>,
}

// Wrapper for the response
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct ItemsResponse {
    pub items: Vec<Item>,
}

// Wrapper for the request
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct ItemsRequest {
    pub items: Vec<Item>,
}
