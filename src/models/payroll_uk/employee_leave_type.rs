//! Models for Payroll UK EmployeeLeaveTypes.

use super::common::GenericRecord;
use serde::{Deserialize, Serialize};

pub type EmployeeLeaveType = GenericRecord;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaveTypeResponse {
    pub leave_types: Option<Vec<EmployeeLeaveType>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaveTypeRequest {
    pub leave_types: Vec<EmployeeLeaveType>,
}
