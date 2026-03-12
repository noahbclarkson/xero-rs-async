//! Models for Payroll AU PayRuns.

use crate::util::xero_date_format_opt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum PayRunStatus {
    Draft,
    Posted,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct PayRun {
    #[serde(rename = "PayRunID", skip_serializing_if = "Option::is_none")]
    pub pay_run_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payroll_calendar_id: Option<Uuid>,
    #[serde(with = "xero_date_format_opt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_run_period_start_date: Option<DateTime<Utc>>,
    #[serde(with = "xero_date_format_opt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_run_period_end_date: Option<DateTime<Utc>>,
    #[serde(with = "xero_date_format_opt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_run_status: Option<PayRunStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_pay: Option<f64>,
    #[serde(rename = "UpdatedDateUTC", with = "xero_date_format_opt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_date_utc: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PayRunsResponse {
    pub pay_runs: Vec<PayRun>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PayRunsRequest {
    pub pay_runs: Vec<PayRun>,
}
