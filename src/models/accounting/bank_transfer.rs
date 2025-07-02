//! Model for the BankTransfer resource.

use super::account::Account;
use crate::util::xero_date_format_opt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BankTransfer {
    pub from_bank_account: Account,
    pub to_bank_account: Account,
    pub amount: f64,
    #[serde(with = "xero_date_format_opt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<DateTime<Utc>>,
    #[serde(rename = "BankTransferID", skip_serializing_if = "Option::is_none")]
    pub bank_transfer_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_rate: Option<f64>,
    #[serde(
        rename = "FromBankTransactionID",
        skip_serializing_if = "Option::is_none"
    )]
    pub from_bank_transaction_id: Option<Uuid>,
    #[serde(
        rename = "ToBankTransactionID",
        skip_serializing_if = "Option::is_none"
    )]
    pub to_bank_transaction_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_is_reconciled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_is_reconciled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_attachments: Option<bool>,
    #[serde(with = "xero_date_format_opt", default, rename = "CreatedDateUTC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_utc: Option<DateTime<Utc>>,
}

// Wrapper for the response
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct BankTransfersResponse {
    pub bank_transfers: Vec<BankTransfer>,
}

// Wrapper for the request
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct BankTransfersRequest {
    pub bank_transfers: Vec<BankTransfer>,
}
