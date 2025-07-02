//! Model for the AssetType resource.

use super::common::{AveragingMethod, DepreciationCalculationMethod, DepreciationMethod};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AssetType {
    pub asset_type_id: Uuid,
    pub asset_type_name: String,
    pub fixed_asset_account_id: Uuid,
    pub depreciation_expense_account_id: Uuid,
    pub accumulated_depreciation_account_id: Uuid,
    pub book_depreciation_setting: AssetTypeBookDepreciationSetting,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locks: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AssetTypeBookDepreciationSetting {
    pub depreciation_method: DepreciationMethod,
    pub averaging_method: AveragingMethod,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depreciation_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_life_years: Option<u32>,
    pub depreciation_calculation_method: DepreciationCalculationMethod,
}
