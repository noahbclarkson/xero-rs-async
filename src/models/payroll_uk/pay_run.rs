//! Models for Payroll UK PayRuns.

use super::common::GenericRecord;
use serde::{Deserialize, Serialize};

pub type PayRun = GenericRecord;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayRunResponse {
    pub pay_runs: Option<Vec<PayRun>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PayRunRequest {
    pub pay_runs: Vec<PayRun>,
}
