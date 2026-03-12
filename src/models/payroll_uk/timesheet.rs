//! Models for Payroll UK Timesheets.

use super::common::GenericRecord;
use serde::{Deserialize, Serialize};

pub type Timesheet = GenericRecord;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimesheetsResponse {
    pub timesheets: Option<Vec<Timesheet>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimesheetResponse {
    pub timesheet: Option<Timesheet>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TimesheetRequest {
    pub timesheet: Timesheet,
}
