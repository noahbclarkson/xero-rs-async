//! Model for the CreditNote resource.

use super::common::{Allocation, LineAmountType, LineItem};
use super::contact::Contact;
use crate::util::{xero_date_format_opt};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub enum CreditNoteType {
    #[serde(rename = "ACCPAYCREDIT")]
    AccountsPayable,
    #[serde(rename = "ACCRECCREDIT")]
    #[default]
    AccountsReceivable,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum CreditNoteStatus {
    Draft,
    Submitted,
    Deleted,
    Authorised,
    Paid,
    Voided,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct CreditNote {
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub credit_note_type: Option<CreditNoteType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    #[serde(with = "xero_date_format_opt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<DateTime<Utc>>,
    // FIX: Made optional as it's not always present in nested responses (e.g., inside an Invoice)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CreditNoteStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_amount_types: Option<LineAmountType>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub line_items: Vec<LineItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_total: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tax: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
    #[serde(rename = "CISDeduction", skip_serializing_if = "Option::is_none")]
    pub cis_deduction: Option<f64>,
    #[serde(with = "xero_date_format_opt", default, rename = "UpdatedDateUTC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_date_utc: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(with = "xero_date_format_opt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fully_paid_on_date: Option<DateTime<Utc>>,
    #[serde(rename = "CreditNoteID", skip_serializing_if = "Option::is_none")]
    pub credit_note_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_note_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent_to_contact: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_credit: Option<f64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allocations: Vec<Allocation>,
    #[serde(rename = "BrandingThemeID", skip_serializing_if = "Option::is_none")]
    pub branding_theme_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_attachments: Option<bool>,
}

// Wrapper for the response
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct CreditNotesResponse {
    pub credit_notes: Vec<CreditNote>,
}

// Wrapper for the request
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct CreditNotesRequest {
    pub credit_notes: Vec<CreditNote>,
}

// Wrapper for the allocation response
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct AllocationsResponse {
    pub allocations: Vec<Allocation>,
}