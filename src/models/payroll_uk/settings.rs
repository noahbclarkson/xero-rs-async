//! Models for Payroll UK Settings.

use super::common::GenericRecord;
use serde::{Deserialize, Serialize};

pub type Settings = GenericRecord;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsResponse {
    pub settings: Option<Settings>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsRequest {
    pub settings: Settings,
}
