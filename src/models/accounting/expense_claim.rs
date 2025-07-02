//! Model for the ExpenseClaim resource.
use super::payment::Payment;
use super::receipt::Receipt;
use super::user::User;
use crate::util::xero_date_format_opt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum ExpenseClaimStatus {
    Submitted,
    Authorised,
    Paid,
    Voided,
    Deleted,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ExpenseClaim {
    #[serde(rename = "ExpenseClaimID", skip_serializing_if = "Option::is_none")]
    pub expense_claim_id: Option<Uuid>,
    pub user: User,
    pub receipts: Vec<Receipt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ExpenseClaimStatus>,
    #[serde(with = "xero_date_format_opt", default, rename = "UpdatedDateUTC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_date_utc: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_due: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_paid: Option<f64>,
    #[serde(with = "xero_date_format_opt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_due_date: Option<DateTime<Utc>>,
    #[serde(with = "xero_date_format_opt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporting_date: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub payments: Vec<Payment>,
}

// Wrapper for the response
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct ExpenseClaimsResponse {
    pub expense_claims: Vec<ExpenseClaim>,
}

// Wrapper for the request
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct ExpenseClaimsRequest {
    pub expense_claims: Vec<ExpenseClaim>,
}