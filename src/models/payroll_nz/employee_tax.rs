//! Models for Payroll NZ Employee Tax.

use super::common::GenericRecord;
use serde::{Deserialize, Serialize};

pub type EmployeeTax = GenericRecord;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmployeeTaxResponse {
    pub tax: Option<EmployeeTax>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EmployeeTaxRequest {
    pub tax: EmployeeTax,
}
