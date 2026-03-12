//! Models for Payroll UK Employee Statutory Leave Balance.

use super::common::GenericRecord;
use serde::Deserialize;

pub type StatutoryLeaveBalance = GenericRecord;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatutoryLeaveBalanceResponse {
    pub leave_balance: Option<StatutoryLeaveBalance>,
}
