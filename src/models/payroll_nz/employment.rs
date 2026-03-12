//! Models for Payroll NZ Employment.

use super::common::GenericRecord;
use serde::{Deserialize, Serialize};

pub type Employment = GenericRecord;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmploymentResponse {
    pub employment: Option<Employment>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EmploymentRequest {
    pub employment: Employment,
}
