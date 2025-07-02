//! Model for the Asset resource.

use super::common::{AveragingMethod, DepreciationCalculationMethod, DepreciationMethod};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disposal_price: Option<f64>,
    pub asset_status: AssetStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warranty_expiry_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub book_depreciation_setting: Option<BookDepreciationSetting>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub book_depreciation_detail: Option<BookDepreciationDetail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_roll_back: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_book_value: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BookDepreciationSetting {
    #[serde(skip_serializing_if = "Option::is_none")]
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
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BookDepreciationDetail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_capital_gain: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_gain_loss: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depreciation_start_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prior_accum_depreciation_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_accum_depreciation_amount: Option<f64>,
}

// Wrapper for the response
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct AssetsResponse {
    pub items: Vec<Asset>,
}
