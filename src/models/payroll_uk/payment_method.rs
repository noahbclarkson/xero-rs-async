//! Models for Payroll UK Payment Methods.

use super::common::GenericRecord;
use serde::{Deserialize, Serialize};

pub type PaymentMethod = GenericRecord;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentMethodResponse {
    pub payment_method: Option<PaymentMethod>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentMethodRequest {
    pub payment_method: PaymentMethod,
}
