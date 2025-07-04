//! Model for the Overpayment resource.

use super::common::{Allocation, LineAmountType, LineItem};
use super::contact::Contact;
use super::payment::Payment;
use crate::util::{xero_date_format, xero_date_format_opt};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub enum OverpaymentType {
    #[serde(rename = "RECEIVE-OVERPAYMENT")]
    #[default]
    ReceiveOverpayment,
    #[serde(rename = "SPEND-OVERPAYMENT")]
    SpendOverpayment,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum OverpaymentStatus {
    Authorised,
    Paid,
    Voided,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Overpayment {
    #[serde(rename = "Type", default, skip_serializing_if = "Option::is_none")]
    pub overpayment_type: Option<OverpaymentType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    #[serde(with = "xero_date_format")]
    pub date: DateTime<Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<OverpaymentStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_amount_types: Option<LineAmountType>,
    pub line_items: Vec<LineItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_total: Option<f64>,
    pub total_tax: f64,
    pub total: f64,
    #[serde(with = "xero_date_format_opt", default, rename = "UpdatedDateUTC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_date_utc: Option<DateTime<Utc>>,
    pub currency_code: String,
    #[serde(rename = "OverpaymentID")]
    pub overpayment_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_rate: Option<f64>,
    pub remaining_credit: f64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allocations: Vec<Allocation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payments: Option<Vec<Payment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_attachments: Option<bool>,
}

// Wrapper for the response
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct OverpaymentsResponse {
    pub overpayments: Vec<Overpayment>,
}
