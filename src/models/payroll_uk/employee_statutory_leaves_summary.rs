//! Models for Payroll UK Statutory Leaves Summary.

use super::common::GenericRecord;
use serde::Deserialize;

pub type StatutoryLeaveSummary = GenericRecord;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatutoryLeavesSummaryResponse {
    pub statutory_leaves: Option<Vec<StatutoryLeaveSummary>>,
}
