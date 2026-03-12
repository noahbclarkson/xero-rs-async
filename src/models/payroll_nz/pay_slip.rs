//! Models for Payroll NZ PaySlips.

use crate::util::iso_datetime_format_opt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct PaySlip {
    #[serde(rename = "paySlipID", skip_serializing_if = "Option::is_none")]
    pub pay_slip_id: Option<Uuid>,
    #[serde(rename = "employeeID", skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<Uuid>,
    #[serde(rename = "payRunID", skip_serializing_if = "Option::is_none")]
    pub pay_run_id: Option<Uuid>,
    #[serde(with = "iso_datetime_format_opt", default)]
    pub period_start_date: Option<DateTime<Utc>>,
    #[serde(with = "iso_datetime_format_opt", default)]
    pub period_end_date: Option<DateTime<Utc>>,
    #[serde(with = "iso_datetime_format_opt", default)]
    pub payment_date: Option<DateTime<Utc>>,
    pub total_pay: Option<f64>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaySlipsResponse {
    pub pay_slips: Option<Vec<PaySlip>>,
}
