//! Model for the PurchaseOrder resource.

use super::common::{LineAmountType, LineItem};
use super::contact::Contact;
use crate::util::xero_date_format_opt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum PurchaseOrderStatus {
    Draft,
    Submitted,
    Authorised,
    Billed,
    Deleted,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct PurchaseOrder {
    pub contact: Contact,
    pub line_items: Vec<LineItem>,
    #[serde(with = "xero_date_format_opt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<DateTime<Utc>>,
    #[serde(with = "xero_date_format_opt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_amount_types: Option<LineAmountType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_order_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(rename = "BrandingThemeID", skip_serializing_if = "Option::is_none")]
    pub branding_theme_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PurchaseOrderStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent_to_contact: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attention_to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telephone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_instructions: Option<String>,
    #[serde(with = "xero_date_format_opt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_arrival_date: Option<DateTime<Utc>>,
    #[serde(rename = "PurchaseOrderID", skip_serializing_if = "Option::is_none")]
    pub purchase_order_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_total: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tax: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_discount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_attachments: Option<bool>,
    #[serde(with = "xero_date_format_opt", default, rename = "UpdatedDateUTC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_date_utc: Option<DateTime<Utc>>,
}

// Wrapper for the response
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct PurchaseOrdersResponse {
    pub purchase_orders: Vec<PurchaseOrder>,
}

// Wrapper for the request
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct PurchaseOrdersRequest {
    pub purchase_orders: Vec<PurchaseOrder>,
}
