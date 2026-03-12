//! Models for Payroll UK Employer Pensions (Benefits).

use super::common::GenericRecord;
use serde::{Deserialize, Serialize};

pub type EmployerPension = GenericRecord;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmployerPensionsResponse {
    pub benefits: Option<Vec<EmployerPension>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmployerPensionResponse {
    pub benefit: Option<EmployerPension>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EmployerPensionRequest {
    pub benefit: EmployerPension,
}
