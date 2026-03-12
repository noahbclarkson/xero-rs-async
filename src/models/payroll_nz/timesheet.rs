//! Models for Payroll NZ Timesheets.

use crate::util::iso_datetime_format_opt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct TimesheetLine {
    #[serde(rename = "timesheetLineID", skip_serializing_if = "Option::is_none")]
    pub timesheet_line_id: Option<Uuid>,
    #[serde(with = "iso_datetime_format_opt", default)]
    pub date: Option<DateTime<Utc>>,
    #[serde(rename = "earningsRateID", skip_serializing_if = "Option::is_none")]
    pub earnings_rate_id: Option<Uuid>,
    #[serde(rename = "trackingItemID", skip_serializing_if = "Option::is_none")]
    pub tracking_item_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_units: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Timesheet {
    #[serde(rename = "timesheetID", skip_serializing_if = "Option::is_none")]
    pub timesheet_id: Option<Uuid>,
    #[serde(rename = "payrollCalendarID", skip_serializing_if = "Option::is_none")]
    pub payroll_calendar_id: Option<Uuid>,
    #[serde(rename = "employeeID", skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<Uuid>,
    #[serde(with = "iso_datetime_format_opt", default)]
    pub start_date: Option<DateTime<Utc>>,
    #[serde(with = "iso_datetime_format_opt", default)]
    pub end_date: Option<DateTime<Utc>>,
    pub status: Option<String>,
    #[serde(rename = "totalHours", skip_serializing_if = "Option::is_none")]
    pub total_hours: Option<f64>,
    #[serde(with = "iso_datetime_format_opt", default)]
    pub updated_date_utc: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub timesheet_lines: Vec<TimesheetLine>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimesheetsResponse {
    pub timesheets: Option<Vec<Timesheet>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimesheetResponse {
    pub timesheet: Option<Timesheet>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimesheetLineResponse {
    pub timesheet_line: Option<TimesheetLine>,
}
