//! Models for Payroll UK Reimbursements.

use super::common::GenericRecord;
use serde::{Deserialize, Serialize};

pub type Reimbursement = GenericRecord;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReimbursementResponse {
    pub reimbursements: Option<Vec<Reimbursement>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReimbursementRequest {
    pub reimbursements: Vec<Reimbursement>,
}
