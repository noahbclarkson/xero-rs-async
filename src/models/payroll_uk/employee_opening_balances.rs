//! Models for Payroll UK Employee Opening Balances.

use super::common::GenericRecord;
use serde::{Deserialize, Serialize};

pub type EmployeeOpeningBalances = GenericRecord;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmployeeOpeningBalancesResponse {
    pub opening_balances: Option<EmployeeOpeningBalances>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EmployeeOpeningBalancesRequest {
    pub opening_balances: EmployeeOpeningBalances,
}
