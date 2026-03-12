//! Models for Payroll NZ EarningsRates.

use super::common::GenericRecord;
use serde::{Deserialize, Serialize};

pub type EarningsRate = GenericRecord;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarningsRateResponse {
    pub earnings_rates: Option<Vec<EarningsRate>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EarningsRateRequest {
    pub earnings_rates: Vec<EarningsRate>,
}
