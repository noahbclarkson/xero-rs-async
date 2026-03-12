//! Models for Payroll NZ Employees.

use crate::util::iso_datetime_format_opt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Employee {
    #[serde(rename = "employeeID", skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<Uuid>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    #[serde(with = "iso_datetime_format_opt", default)]
    pub date_of_birth: Option<DateTime<Utc>>,
    pub gender: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    #[serde(with = "iso_datetime_format_opt", default)]
    pub start_date: Option<DateTime<Utc>>,
    pub address: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmployeesResponse {
    pub employees: Option<Vec<Employee>>,
}
