//! Models for Payroll NZ PayRuns.

use crate::util::iso_datetime_format_opt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct PayRun {
    #[serde(rename = "payRunID", skip_serializing_if = "Option::is_none")]
    pub pay_run_id: Option<Uuid>,
    #[serde(rename = "payrollCalendarID", skip_serializing_if = "Option::is_none")]
    pub payroll_calendar_id: Option<Uuid>,
    #[serde(with = "iso_datetime_format_opt", default)]
    pub pay_run_period_start_date: Option<DateTime<Utc>>,
    #[serde(with = "iso_datetime_format_opt", default)]
    pub pay_run_period_end_date: Option<DateTime<Utc>>,
    #[serde(with = "iso_datetime_format_opt", default)]
    pub payment_date: Option<DateTime<Utc>>,
    pub pay_run_status: Option<String>,
    pub total_pay: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayRunsResponse {
    pub pay_runs: Option<Vec<PayRun>>,
}
