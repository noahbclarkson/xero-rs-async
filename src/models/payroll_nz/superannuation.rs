//! Models for Payroll NZ Superannuation.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Superannuation {
    #[serde(rename = "superannuationID", skip_serializing_if = "Option::is_none")]
    pub superannuation_id: Option<Uuid>,
    pub name: Option<String>,
    pub employee_contribution_rate: Option<f64>,
    pub employer_contribution_rate: Option<f64>,
    pub deduction_type: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SuperannuationResponse {
    pub superannuations: Option<Vec<Superannuation>>,
}
