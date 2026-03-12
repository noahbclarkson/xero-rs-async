//! Models for Payroll UK LeaveTypes.

use super::common::GenericRecord;
use serde::{Deserialize, Serialize};

pub type LeaveType = GenericRecord;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaveTypeResponse {
    pub leave_types: Option<Vec<LeaveType>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaveTypeRequest {
    pub leave_types: Vec<LeaveType>,
}
