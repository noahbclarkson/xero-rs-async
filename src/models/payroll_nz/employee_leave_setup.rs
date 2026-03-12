//! Models for Payroll NZ Employee Leave Setup.

use super::common::GenericRecord;
use serde::Deserialize;

pub type EmployeeLeaveSetup = GenericRecord;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmployeeLeaveSetupResponse {
    pub leave_setup: Option<EmployeeLeaveSetup>,
}
