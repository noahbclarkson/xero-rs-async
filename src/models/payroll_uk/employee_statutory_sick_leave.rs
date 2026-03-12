//! Models for Payroll UK Statutory Sick Leave.

use super::common::GenericRecord;
use serde::Deserialize;

pub type StatutorySickLeave = GenericRecord;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatutorySickLeaveResponse {
    pub statutory_sick_leave: Option<StatutorySickLeave>,
}
