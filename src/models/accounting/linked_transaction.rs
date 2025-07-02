//! Model for the LinkedTransaction resource.

use crate::util::xero_date_format_opt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum LinkedTransactionStatus {
    Draft,
    Approved,
    Ondraft,
    Billed,
    Voided,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum LinkedTransactionType {
    Billableexpense,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum SourceTransactionTypeCode {
    Accpay,
    Spend,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct LinkedTransaction {
    #[serde(
        rename = "LinkedTransactionID",
        skip_serializing_if = "Option::is_none"
    )]
    pub linked_transaction_id: Option<Uuid>,
    #[serde(rename = "SourceTransactionID")]
    pub source_transaction_id: Uuid,
    #[serde(rename = "SourceLineItemID")]
    pub source_line_item_id: Uuid,
    #[serde(rename = "ContactID", skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<Uuid>,
    #[serde(
        rename = "TargetTransactionID",
        skip_serializing_if = "Option::is_none"
    )]
    pub target_transaction_id: Option<Uuid>,
    #[serde(rename = "TargetLineItemID", skip_serializing_if = "Option::is_none")]
    pub target_line_item_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<LinkedTransactionStatus>,
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub linked_transaction_type: Option<LinkedTransactionType>,
    #[serde(with = "xero_date_format_opt", default, rename = "UpdatedDateUTC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_date_utc: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_transaction_type_code: Option<SourceTransactionTypeCode>,
}

// Wrapper for the response
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct LinkedTransactionsResponse {
    pub linked_transactions: Vec<LinkedTransaction>,
}

// Wrapper for the request
#[allow(dead_code)]
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct LinkedTransactionsRequest {
    pub linked_transactions: Vec<LinkedTransaction>,
}
