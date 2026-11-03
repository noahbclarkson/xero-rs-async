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
    /// Absent when the organisation has never configured its Fixed Asset Register (Xero omits the
    /// key rather than sending null).
    #[serde(
        with = "date_format::option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub asset_start_date: Option<NaiveDate>,
    /// Absent when the organisation has never posted a depreciation run, even with a configured
    /// register and registered assets. Treat `None` as "depreciation has never been run", not as an
    /// error.
    #[serde(
        with = "date_format::option",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_depreciation_date: Option<NaiveDate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_gain_on_disposal_account_id: Option<Uuid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_loss_on_disposal_account_id: Option<Uuid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_capital_gain_on_disposal_account_id: Option<Uuid>,
    pub opt_in_for_tax: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A configured register that has never run depreciation: Xero omits `lastDepreciationDate`
    /// entirely, which made the whole response undeserialisable while the field was required.
    #[test]
    fn settings_without_last_depreciation_date_parse() {
        let json = r#"{"assetNumberPrefix":"FA-","assetNumberSequence":"0001","assetStartDate":"2025-04-01T00:00:00","optInForTax":false}"#;
        let s: Settings = serde_json::from_str(json).expect("parse");
        assert_eq!(s.asset_start_date, NaiveDate::from_ymd_opt(2025, 4, 1));
        assert_eq!(s.last_depreciation_date, None);
    }

    /// An organisation that has never configured Fixed Assets omits both dates.
    #[test]
    fn settings_without_any_dates_parse() {
        let json =
            r#"{"assetNumberPrefix":"FA-","assetNumberSequence":"0001","optInForTax":false}"#;
        let s: Settings = serde_json::from_str(json).expect("parse");
        assert_eq!(s.asset_start_date, None);
        assert_eq!(s.last_depreciation_date, None);
    }

    #[test]
    fn settings_with_dates_parse() {
        let json = r#"{"assetNumberPrefix":"FA-","assetNumberSequence":"0002","assetStartDate":"2024-04-01T00:00:00","lastDepreciationDate":"2026-03-31T00:00:00","optInForTax":true}"#;
        let s: Settings = serde_json::from_str(json).expect("parse");
        assert_eq!(
            s.last_depreciation_date,
            NaiveDate::from_ymd_opt(2026, 3, 31)
        );
    }
}
