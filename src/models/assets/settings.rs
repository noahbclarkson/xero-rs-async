//! Model for the Asset Settings resource.

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub asset_number_prefix: String,
    pub asset_number_sequence: String,
    pub asset_start_date: NaiveDate,
    pub last_depreciation_date: NaiveDate,
    pub default_gain_on_disposal_account_id: Uuid,
    pub default_loss_on_disposal_account_id: Uuid,
    pub default_capital_gain_on_disposal_account_id: Uuid,
    pub opt_in_for_tax: bool,
}
