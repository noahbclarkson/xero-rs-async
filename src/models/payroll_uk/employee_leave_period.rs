//! Models for Payroll UK EmployeeLeavePeriods.

use super::common::GenericRecord;
use serde::{Deserialize, Serialize};

pub type EmployeeLeavePeriod = GenericRecord;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeavePeriodResponse {
    pub leave_periods: Option<Vec<EmployeeLeavePeriod>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LeavePeriodRequest {
    pub leave_periods: Vec<EmployeeLeavePeriod>,
}
