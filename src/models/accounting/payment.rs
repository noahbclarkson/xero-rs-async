//! Model for the Payment resource.

use super::account::Account;
use super::credit_note::CreditNote;
use super::invoice::Invoice;
use super::overpayment::Overpayment;
use super::prepayment::Prepayment;
use crate::util::{xero_date_format, xero_date_format_opt};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum PaymentType {
    Accrecpayment,
    Accpaypayment,
    Arcreditpayment,
    Apcreditpayment,
    Aroverpaymentpayment,
    Arprepaymentpayment,
    Apprepaymentpayment,
    Apoverpaymentpayment,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum PaymentStatus {
    Authorised,
    Deleted,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct Payment {
    #[serde(rename = "PaymentID", skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<Invoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_note: Option<CreditNote>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepayment: Option<Prepayment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overpayment: Option<Overpayment>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account: Option<Account>,
    #[serde(with = "xero_date_format")]
    pub date: DateTime<Utc>,
    pub amount: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_reconciled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub status: Option<PaymentStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<PaymentType>,
    #[serde(with = "xero_date_format_opt", default, rename = "UpdatedDateUTC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_date_utc: Option<DateTime<Utc>>,
}

// Wrapper for the response
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct PaymentsResponse {
    pub payments: Vec<Payment>,
}

// Wrapper for the request
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct PaymentsRequest {
    pub payments: Vec<Payment>,
}