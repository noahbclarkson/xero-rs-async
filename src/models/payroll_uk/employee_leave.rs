//! Models for Payroll UK Employee Leave.

use super::common::GenericRecord;
use serde::{Deserialize, Serialize};

pub type EmployeeLeave = GenericRecord;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmployeeLeavesResponse {
    pub leave: Option<Vec<EmployeeLeave>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmployeeLeaveResponse {
    pub leave: Option<EmployeeLeave>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EmployeeLeaveRequest {
    pub leave: EmployeeLeave,
}
