//! Models for Payroll UK Deductions.

use super::common::GenericRecord;
use serde::{Deserialize, Serialize};

pub type Deduction = GenericRecord;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeductionResponse {
    pub deductions: Option<Vec<Deduction>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeductionRequest {
    pub deductions: Vec<Deduction>,
}
