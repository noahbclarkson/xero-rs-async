//! Model for the `AssetType` resource.

use std::collections::HashMap;

use super::common::{AveragingMethod, DepreciationCalculationMethod, DepreciationMethod};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AssetType {
    pub asset_type_id: Uuid,
    pub asset_type_name: String,
    #[serde(default)]
    pub fixed_asset_account_id: Option<Uuid>,
    #[serde(default)]
    pub depreciation_expense_account_id: Option<Uuid>,
    #[serde(default)]
    pub accumulated_depreciation_account_id: Option<Uuid>,
    #[serde(default)]
    pub book_depreciation_setting: Option<AssetTypeBookDepreciationSetting>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locks: Option<u32>,
    /// Captures any additional fields Xero may return
    #[serde(flatten)]
    #[serde(skip_serializing)]
    #[allow(dead_code)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AssetTypeBookDepreciationSetting {
    #[serde(default)]
    pub depreciation_method: Option<DepreciationMethod>,
    #[serde(default)]
    pub averaging_method: Option<AveragingMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depreciation_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_life_years: Option<u32>,
    #[serde(default)]
    pub depreciation_calculation_method: Option<DepreciationCalculationMethod>,
}

/// Wrapper for GET /AssetTypes response (can be an object or a raw list).
#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum AssetTypesResponse {
    Wrapper {
        #[serde(default)]
        asset_types: Vec<AssetType>,
    },
    List(Vec<AssetType>),
}

impl AssetTypesResponse {
    pub fn into_vec(self) -> Vec<AssetType> {
        match self {
            Self::Wrapper { asset_types } => asset_types,
            Self::List(items) => items,
        }
    }
}
