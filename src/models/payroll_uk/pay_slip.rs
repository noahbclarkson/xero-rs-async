//! Models for Payroll UK Payslips.

use super::common::GenericRecord;
use serde::Deserialize;

pub type PaySlip = GenericRecord;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaySlipResponse {
    pub pay_slip: Option<PaySlip>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaySlipsResponse {
    pub pay_slips: Option<Vec<PaySlip>>,
}
