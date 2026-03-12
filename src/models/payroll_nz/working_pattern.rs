//! Models for Payroll NZ WorkingPatterns.

use super::common::GenericRecord;
use serde::{Deserialize, Serialize};

pub type WorkingPattern = GenericRecord;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkingPatternResponse {
    pub working_patterns: Option<Vec<WorkingPattern>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkingPatternRequest {
    pub working_patterns: Vec<WorkingPattern>,
}
