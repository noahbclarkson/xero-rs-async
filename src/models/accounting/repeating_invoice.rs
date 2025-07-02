//! Model for the RepeatingInvoice resource.

use super::common::{LineAmountType, LineItem, PaymentTermType};
use super::contact::Contact;
use super::invoice::InvoiceType;
use crate::util::{xero_date_format, xero_date_format_opt};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum RepeatingInvoiceStatus {
    Draft,
    Authorised,
    Deleted,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum ScheduleUnit {
    Weekly,
    Monthly,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct RepeatingInvoice {
    #[serde(rename = "RepeatingInvoiceID", skip_serializing_if = "Option::is_none")]
    pub repeating_invoice_id: Option<Uuid>,
    #[serde(rename = "Type")]
    pub invoice_type: InvoiceType,
    pub contact: Contact,
    pub schedule: Schedule,
    pub line_items: Vec<LineItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_amount_types: Option<LineAmountType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(rename = "BrandingThemeID", skip_serializing_if = "Option::is_none")]
    pub branding_theme_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<RepeatingInvoiceStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_total: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tax: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_attachments: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_for_sending: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_copy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark_as_sent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_pdf: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")] // Fix: Add this attribute
pub struct Schedule {
    pub period: i32,
    pub unit: ScheduleUnit,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date_type: Option<PaymentTermType>,
    #[serde(with = "xero_date_format")]
    pub start_date: DateTime<Utc>,
    #[serde(with = "xero_date_format_opt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_scheduled_date: Option<DateTime<Utc>>,
    #[serde(with = "xero_date_format_opt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<DateTime<Utc>>,
}

// Wrapper for the response
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct RepeatingInvoicesResponse {
    pub repeating_invoices: Vec<RepeatingInvoice>,
}

// Wrapper for the request
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct RepeatingInvoicesRequest {
    pub repeating_invoices: Vec<RepeatingInvoice>,
}
