//! Models for Payroll NZ EmployeeLeaveBalances.

use super::common::GenericRecord;
use serde::{Deserialize, Serialize};

pub type EmployeeLeaveBalance = GenericRecord;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaveBalanceResponse {
    pub leave_balances: Option<Vec<EmployeeLeaveBalance>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LeaveBalanceRequest {
    pub leave_balances: Vec<EmployeeLeaveBalance>,
}
