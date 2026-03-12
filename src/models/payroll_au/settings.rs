//! Models for Payroll AU Settings.

use crate::models::payroll_au::common::ExtraFields;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct SettingsAccount {
    #[serde(rename = "AccountID", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<Uuid>,
    pub name: Option<String>,
    pub code: Option<String>,
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct SettingsTrackingCategory {
    #[serde(rename = "TrackingCategoryID", skip_serializing_if = "Option::is_none")]
    pub tracking_category_id: Option<Uuid>,
    pub tracking_category_name: Option<String>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct Settings {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub accounts: Vec<SettingsAccount>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tracking_categories: Vec<SettingsTrackingCategory>,
    pub days_in_payroll_year: Option<String>,
    pub employees_are_stp2: Option<bool>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SettingsResponse {
    pub settings: Option<Settings>,
}
