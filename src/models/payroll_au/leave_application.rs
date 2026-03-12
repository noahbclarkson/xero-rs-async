//! Models for Payroll AU Leave Applications.

use crate::util::xero_date_format_opt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct LeavePeriod {
    #[serde(with = "xero_date_format_opt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_period_start_date: Option<DateTime<Utc>>,
    #[serde(with = "xero_date_format_opt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_period_end_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leave_period_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_units: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct LeaveApplication {
    #[serde(rename = "LeaveApplicationID", skip_serializing_if = "Option::is_none")]
    pub leave_application_id: Option<Uuid>,
    #[serde(rename = "EmployeeID", skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<Uuid>,
    #[serde(rename = "LeaveTypeID", skip_serializing_if = "Option::is_none")]
    pub leave_type_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(with = "xero_date_format_opt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<DateTime<Utc>>,
    #[serde(with = "xero_date_format_opt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_out_type: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub leave_periods: Vec<LeavePeriod>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LeaveApplicationsResponse {
    pub leave_applications: Vec<LeaveApplication>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct LeaveApplicationsRequest {
    pub leave_applications: Vec<LeaveApplication>,
}
