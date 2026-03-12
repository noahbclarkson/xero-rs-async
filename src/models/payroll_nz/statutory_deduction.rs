//! Models for Payroll NZ Statutory Deductions.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct StatutoryDeduction {
    #[serde(
        rename = "statutoryDeductionID",
        skip_serializing_if = "Option::is_none"
    )]
    pub statutory_deduction_id: Option<Uuid>,
    pub name: Option<String>,
    pub calculation_type: Option<String>,
    pub amount: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatutoryDeductionsResponse {
    pub statutory_deductions: Option<Vec<StatutoryDeduction>>,
}
