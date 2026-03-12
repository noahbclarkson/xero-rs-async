//! Models for Payroll UK EarningsOrders.

use super::common::GenericRecord;
use serde::{Deserialize, Serialize};

pub type EarningsOrder = GenericRecord;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarningsOrderResponse {
    pub earnings_orders: Option<Vec<EarningsOrder>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EarningsOrderRequest {
    pub earnings_orders: Vec<EarningsOrder>,
}
