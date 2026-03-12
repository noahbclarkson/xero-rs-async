//! Models for Payroll AU Super Funds.

use crate::models::payroll_au::common::ExtraFields;
use crate::util::xero_date_format_opt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct SuperFund {
    #[serde(rename = "SuperFundID", skip_serializing_if = "Option::is_none")]
    pub super_fund_id: Option<Uuid>,
    pub name: Option<String>,
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub fund_type: Option<String>,
    #[serde(rename = "ABN", skip_serializing_if = "Option::is_none")]
    pub abn: Option<String>,
    #[serde(rename = "USI", skip_serializing_if = "Option::is_none")]
    pub usi: Option<String>,
    #[serde(rename = "SPIN", skip_serializing_if = "Option::is_none")]
    pub spin: Option<String>,
    #[serde(rename = "BSB", skip_serializing_if = "Option::is_none")]
    pub bsb: Option<String>,
    pub account_number: Option<String>,
    pub account_name: Option<String>,
    pub electronic_service_address: Option<String>,
    pub employer_number: Option<String>,
    #[serde(rename = "UpdatedDateUTC", with = "xero_date_format_opt", default)]
    pub updated_date_utc: Option<DateTime<Utc>>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SuperFundsResponse {
    pub super_funds: Vec<SuperFund>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SuperFundsRequest {
    pub super_funds: Vec<SuperFund>,
}
