//! Models for Payroll NZ Employee Pay Templates.

use super::common::GenericRecord;
use serde::{Deserialize, Serialize};

pub type PayTemplateEarning = GenericRecord;
pub type PayTemplateDeduction = GenericRecord;
pub type PayTemplateBenefit = GenericRecord;
pub type PayTemplateReimbursement = GenericRecord;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayTemplatesResponse {
    pub earning_templates: Option<Vec<PayTemplateEarning>>,
    pub deduction_templates: Option<Vec<PayTemplateDeduction>>,
    pub benefit_templates: Option<Vec<PayTemplateBenefit>>,
    pub reimbursement_templates: Option<Vec<PayTemplateReimbursement>>,
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
