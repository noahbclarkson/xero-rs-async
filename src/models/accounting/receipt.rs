//! Model for the Receipt resource.

use super::common::{LineAmountType, LineItem};
use super::contact::Contact;
use super::user::User;
use crate::util::{xero_date_format, xero_date_format_opt};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum ReceiptStatus {
    Draft,
    Submitted,
    Authorised,
    Declined,
    Deleted,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Receipt {
    #[serde(rename = "ReceiptID", skip_serializing_if = "Option::is_none")]
    pub receipt_id: Option<Uuid>,
    pub user: User,
    pub contact: Contact,
    pub line_items: Vec<LineItem>,
    #[serde(with = "xero_date_format")]
    pub date: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_amount_types: Option<LineAmountType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_total: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tax: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ReceiptStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receipt_number: Option<u64>,
    #[serde(with = "xero_date_format_opt", default, rename = "UpdatedDateUTC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_date_utc: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_attachments: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

// Wrapper for the response
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct ReceiptsResponse {
    pub receipts: Vec<Receipt>,
}

// Wrapper for the request
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct ReceiptsRequest {
    pub receipts: Vec<Receipt>,
}
