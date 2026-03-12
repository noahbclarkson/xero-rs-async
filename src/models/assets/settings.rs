//! Model for the Asset Settings resource.

use super::date_format;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub asset_number_prefix: String,
    pub asset_number_sequence: String,
    #[serde(with = "date_format")]
    pub asset_start_date: NaiveDate,
    #[serde(with = "date_format")]
    pub last_depreciation_date: NaiveDate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_gain_on_disposal_account_id: Option<Uuid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_loss_on_disposal_account_id: Option<Uuid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_capital_gain_on_disposal_account_id: Option<Uuid>,
    pub opt_in_for_tax: bool,
}
