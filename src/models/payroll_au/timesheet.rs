//! Models for Payroll AU Timesheets.

use crate::util::xero_date_format_opt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct TimesheetLine {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub earnings_rate_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_item_id: Option<Uuid>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub number_of_units: Vec<f64>,
    #[serde(rename = "UpdatedDateUTC", with = "xero_date_format_opt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_date_utc: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum TimesheetStatus {
    Draft,
    Processed,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct Timesheet {
    #[serde(rename = "TimesheetID", skip_serializing_if = "Option::is_none")]
    pub timesheet_id: Option<Uuid>,
    #[serde(rename = "EmployeeID", skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<Uuid>,
    #[serde(with = "xero_date_format_opt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<DateTime<Utc>>,
    #[serde(with = "xero_date_format_opt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TimesheetStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours: Option<f64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub timesheet_lines: Vec<TimesheetLine>,
    #[serde(rename = "UpdatedDateUTC", with = "xero_date_format_opt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_date_utc: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TimesheetsResponse {
    pub timesheets: Vec<Timesheet>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct TimesheetsRequest {
    pub timesheets: Vec<Timesheet>,
}
