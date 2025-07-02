//! Model for the BankTransaction resource.

use super::account::Account;
use super::batch_payment::BatchPayment;
use super::common::{LineAmountType, LineItem};
use super::contact::Contact;
use crate::util::{xero_date_format, xero_date_format_opt};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub enum BankTransactionType {
    #[serde(rename = "RECEIVE")]
    Receive,
    #[serde(rename = "RECEIVE-OVERPAYMENT")]
    ReceiveOverpayment,
    #[serde(rename = "RECEIVE-PREPAYMENT")]
    ReceivePrepayment,
    #[serde(rename = "SPEND")]
    #[default]
    Spend,
    #[serde(rename = "SPEND-OVERPAYMENT")]
    SpendOverpayment,
    #[serde(rename = "SPEND-PREPAYMENT")]
    SpendPrepayment,
    #[serde(rename = "RECEIVE-TRANSFER")]
    ReceiveTransfer,
    #[serde(rename = "SPEND-TRANSFER")]
    SpendTransfer,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum BankTransactionStatus {
    Authorised,
    Deleted,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct BankTransaction {
    #[serde(
        rename = "Type",
        alias = "type",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub transaction_type: Option<BankTransactionType>,
    pub contact: Contact,
    pub line_items: Vec<LineItem>,
    pub bank_account: Account,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_reconciled: Option<bool>,
    #[serde(with = "xero_date_format")]
    pub date: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<BankTransactionStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_amount_types: Option<LineAmountType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_total: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tax: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
    #[serde(rename = "BankTransactionID", skip_serializing_if = "Option::is_none")]
    pub bank_transaction_id: Option<Uuid>,
    #[serde(rename = "PrepaymentID", skip_serializing_if = "Option::is_none")]
    pub prepayment_id: Option<Uuid>,
    #[serde(rename = "OverpaymentID", skip_serializing_if = "Option::is_none")]
    pub overpayment_id: Option<Uuid>,
    #[serde(with = "xero_date_format_opt", default, rename = "UpdatedDateUTC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_date_utc: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_attachments: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_payment: Option<BatchPayment>,
}

// Wrapper for the response
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct BankTransactionsResponse {
    pub bank_transactions: Vec<BankTransaction>,
}

// Wrapper for the request
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct BankTransactionsRequest {
    pub bank_transactions: Vec<BankTransaction>,
}
