//! Models for Payroll UK Employees.

use super::common::GenericRecord;
use serde::{Deserialize, Serialize};

pub type Employee = GenericRecord;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmployeeResponse {
    pub employees: Option<Vec<Employee>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EmployeeRequest {
    pub employees: Vec<Employee>,
}
