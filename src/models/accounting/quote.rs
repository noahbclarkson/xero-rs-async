//! Model for the Quote resource.

use super::common::{LineAmountType, LineItem};
use super::contact::Contact;
use crate::util::{xero_date_format, xero_date_format_opt};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum QuoteStatus {
    Draft,
    Deleted,
    Sent,
    Declined,
    Accepted,
    Invoiced,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Quote {
    pub contact: Contact,
    pub line_items: Vec<LineItem>,
    #[serde(with = "xero_date_format")]
    pub date: DateTime<Utc>,
    #[serde(with = "xero_date_format_opt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<QuoteStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_amount_types: Option<LineAmountType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_total: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tax: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_discount: Option<f64>,
    #[serde(with = "xero_date_format_opt", default, rename = "UpdatedDateUTC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_date_utc: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_rate: Option<f64>,
    #[serde(rename = "QuoteID", skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(rename = "BrandingThemeID", skip_serializing_if = "Option::is_none")]
    pub branding_theme_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms: Option<String>,
}

// Wrapper for the response
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct QuotesResponse {
    pub quotes: Vec<Quote>,
}

// Wrapper for the request
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct QuotesRequest {
    pub quotes: Vec<Quote>,
}
