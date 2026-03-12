//! Model for the Asset resource.

use std::collections::HashMap;

use super::common::{AveragingMethod, DepreciationCalculationMethod, DepreciationMethod};
use super::date_format;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum AssetStatus {
    Draft,
    Registered,
    Disposed,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub asset_id: Uuid,
    pub asset_name: String,
    pub asset_number: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "date_format::option")]
    pub purchase_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disposal_price: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "date_format::option")]
    pub disposal_date: Option<NaiveDate>,
    pub asset_status: AssetStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "date_format::option")]
    pub warranty_expiry_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_type_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub book_depreciation_setting: Option<BookDepreciationSetting>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub book_depreciation_detail: Option<BookDepreciationDetail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_roll_back: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_book_value: Option<f64>,
    /// Captures any additional fields Xero may return that aren't explicitly modeled
    #[serde(flatten)]
    #[serde(skip_serializing)]
    #[allow(dead_code)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BookDepreciationSetting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "date_format::option")]
    pub effective_from_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depreciation_method: Option<DepreciationMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub averaging_method: Option<AveragingMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depreciation_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_life_years: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depreciation_calculation_method: Option<DepreciationCalculationMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub residual_value: Option<f64>,
    /// Captures any additional fields Xero may return
    #[serde(flatten)]
    #[serde(skip_serializing)]
    #[allow(dead_code)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BookDepreciationDetail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_capital_gain: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_gain_loss: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "date_format::option")]
    pub depreciation_start_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prior_accum_depreciation_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_accum_depreciation_amount: Option<f64>,
    /// Captures any additional fields Xero may return
    #[serde(flatten)]
    #[serde(skip_serializing)]
    #[allow(dead_code)]
    pub extra: HashMap<String, Value>,
}

// Pagination info for Assets API responses
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetsPagination {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub page_count: Option<u32>,
    pub item_count: Option<u32>,
}

// Link structure for HATEOAS-style navigation
#[derive(Debug, Deserialize, Clone)]
pub struct AssetsLink {
    pub href: Option<String>,
}

// Links container for Assets API pagination
#[derive(Debug, Deserialize, Clone, Default)]
pub struct AssetsLinks {
    pub first: Option<AssetsLink>,
    pub next: Option<AssetsLink>,
    pub previous: Option<AssetsLink>,
    pub last: Option<AssetsLink>,
}

// Wrapper for the response - matches Xero Assets API envelope
// Note: pagination and links fields exist for serde deserialization but are unused
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AssetsResponse {
    /// The pagination metadata for this response (unused, but needed for deserialization)
    #[serde(default)]
    #[allow(dead_code)]
    pub pagination: Option<AssetsPagination>,
    /// HATEOAS links for navigation (unused, but needed for deserialization)
    #[serde(default)]
    #[allow(dead_code)]
    pub links: Option<AssetsLinks>,
    /// The actual asset items
    pub items: Vec<Asset>,
}
