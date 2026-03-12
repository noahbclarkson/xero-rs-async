//! Models for Payroll NZ Leave.

use crate::util::xero_naive_date_format_opt;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct LeavePeriod {
    #[serde(with = "xero_naive_date_format_opt", default)]
    pub pay_period_start_date: Option<NaiveDate>,
    #[serde(with = "xero_naive_date_format_opt", default)]
    pub pay_period_end_date: Option<NaiveDate>,
    pub number_of_units: Option<f64>,
    pub leave_period_status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Leave {
    #[serde(rename = "leaveID", skip_serializing_if = "Option::is_none")]
    pub leave_id: Option<Uuid>,
    #[serde(rename = "leaveTypeID", skip_serializing_if = "Option::is_none")]
    pub leave_type_id: Option<Uuid>,
    pub description: Option<String>,
    #[serde(with = "xero_naive_date_format_opt", default)]
    pub start_date: Option<NaiveDate>,
    #[serde(with = "xero_naive_date_format_opt", default)]
    pub end_date: Option<NaiveDate>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub periods: Vec<LeavePeriod>,
    #[serde(with = "xero_naive_date_format_opt", default)]
    pub updated_date_utc: Option<NaiveDate>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaveResponse {
    pub leave: Option<Leave>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeavesResponse {
    pub leave: Option<Vec<Leave>>,
}
