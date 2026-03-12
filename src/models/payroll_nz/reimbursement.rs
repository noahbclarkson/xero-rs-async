//! Models for Payroll NZ Reimbursements.

use crate::util::iso_datetime_format_opt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Reimbursement {
    #[serde(rename = "reimbursementID", skip_serializing_if = "Option::is_none")]
    pub reimbursement_id: Option<Uuid>,
    #[serde(rename = "earningsRateID", skip_serializing_if = "Option::is_none")]
    pub earnings_rate_id: Option<Uuid>,
    pub description: Option<String>,
    pub amount: Option<f64>,
    #[serde(with = "iso_datetime_format_opt", default)]
    pub reimbursement_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReimbursementsResponse {
    pub reimbursements: Option<Vec<Reimbursement>>,
}
