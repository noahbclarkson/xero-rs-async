//! Model for the BatchPayment resource.

use super::account::Account;
use super::invoice::Invoice;
use crate::util::{xero_date_format, xero_date_format_opt};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum BatchPaymentType {
    Paybatch,
    Recbatch,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum BatchPaymentStatus {
    Authorised,
    Deleted,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BatchPayment {
    pub account: Account,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payments: Option<Vec<PaymentDetail>>,
    #[serde(with = "xero_date_format")]
    pub date: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub particulars: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub narrative: Option<String>,
    #[serde(rename = "BatchPaymentID", skip_serializing_if = "Option::is_none")]
    pub batch_payment_id: Option<Uuid>,
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub batch_payment_type: Option<BatchPaymentType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<BatchPaymentStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_reconciled: Option<bool>,
    #[serde(with = "xero_date_format_opt", default, rename = "UpdatedDateUTC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_date_utc: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct PaymentDetail {
    pub invoice: Box<Invoice>,
    pub amount: f64,
    #[serde(rename = "PaymentID", skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub particulars: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_amount: Option<f64>,
}

// Wrapper for the response
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct BatchPaymentsResponse {
    pub batch_payments: Vec<BatchPayment>,
}
