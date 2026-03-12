//! Models for Payroll UK Employee Pay Templates.

use super::common::GenericRecord;
use serde::{Deserialize, Serialize};

pub type PayTemplateEarning = GenericRecord;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayTemplatesResponse {
    pub earning_templates: Option<Vec<PayTemplateEarning>>,
    pub deduction_templates: Option<Vec<GenericRecord>>,
    pub benefit_templates: Option<Vec<GenericRecord>>,
    pub reimbursement_templates: Option<Vec<GenericRecord>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayTemplateEarningResponse {
    pub earning_template: Option<PayTemplateEarning>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PayTemplateEarningRequest {
    pub earning_template: PayTemplateEarning,
}
