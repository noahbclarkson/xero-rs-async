//! Model for the Invoice resource.
use super::common::{Address, LineAmountType, LineItem};
use super::contact::Contact;
use super::credit_note::CreditNote;
use super::overpayment::Overpayment;
use super::payment::Payment;
use super::prepayment::Prepayment;
use crate::util::{xero_date_format, xero_date_format_opt};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum InvoiceType {
    Accpay,
    #[default]
    Accrec,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum InvoiceStatus {
    Draft,
    Submitted,
    Deleted,
    Authorised,
    Paid,
    Voided,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct Invoice {
    #[serde(rename = "Type")]
    pub invoice_type: InvoiceType,
    pub contact: Contact,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub line_items: Vec<LineItem>,
    #[serde(with = "xero_date_format")]
    pub date: DateTime<Utc>,
    #[serde(with = "xero_date_format_opt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_amount_types: Option<LineAmountType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invoice_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(rename = "BrandingThemeID", skip_serializing_if = "Option::is_none")]
    pub branding_theme_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub status: Option<InvoiceStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent_to_contact: Option<bool>,
    #[serde(with = "xero_date_format_opt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_payment_date: Option<DateTime<Utc>>,
    #[serde(with = "xero_date_format_opt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub planned_payment_date: Option<DateTime<Utc>>,
    #[serde(rename = "InvoiceID", skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<Uuid>,
    #[serde(rename = "RepeatingInvoiceID", skip_serializing_if = "Option::is_none")]
    pub repeating_invoice_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_attachments: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payments: Option<Vec<Payment>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub credit_notes: Vec<CreditNote>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub prepayments: Vec<Prepayment>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub overpayments: Vec<Overpayment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_due: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_paid: Option<f64>,
    #[serde(with = "xero_date_format_opt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fully_paid_on_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_credited: Option<f64>,
    #[serde(with = "xero_date_format_opt", default, rename = "UpdatedDateUTC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_date_utc: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_total: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tax: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_discount: Option<f64>,
    #[serde(rename = "CISDeduction", skip_serializing_if = "Option::is_none")]
    pub cis_deduction: Option<f64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub invoice_addresses: Vec<Address>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct OnlineInvoice {
    pub online_invoice_url: String,
}

// Wrapper for the response
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct InvoicesResponse {
    pub invoices: Vec<Invoice>,
}

// Wrapper for the request
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct InvoicesRequest {
    pub invoices: Vec<Invoice>,
}

// Wrapper for the online invoice response
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct OnlineInvoicesResponse {
    pub online_invoices: Vec<OnlineInvoice>,
}