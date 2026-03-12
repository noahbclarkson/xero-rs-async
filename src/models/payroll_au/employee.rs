//! Models for Payroll AU Employees.

use crate::util::xero_date_format_opt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum EmployeeStatus {
    Active,
    Terminated,
    Archived,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct Employee {
    #[serde(rename = "EmployeeID", skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_names: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(with = "xero_date_format_opt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<DateTime<Utc>>,
    #[serde(with = "xero_date_format_opt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<DateTime<Utc>>,
    #[serde(rename = "UpdatedDateUTC", with = "xero_date_format_opt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_date_utc: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<EmployeeStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payroll_calendar_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordinary_earnings_rate_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_stp2_qualified: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EmployeesResponse {
    pub employees: Vec<Employee>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct EmployeesRequest {
    pub employees: Vec<Employee>,
}
