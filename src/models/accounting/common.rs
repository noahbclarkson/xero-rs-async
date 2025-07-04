// src/models/accounting/common.rs

//! Contains common data structures shared across multiple Accounting API endpoints.

use crate::util::xero_date_format;
use chrono::{DateTime, Utc};
use serde::{self, Deserialize, Deserializer, Serialize};
use uuid::Uuid;

// --- Enums from "Types and Codes" ---

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum AddressType {
    Pobox,
    #[default]
    Street,
    Delivery,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "UPPERCASE")]
pub enum PhoneType {
    #[default]
    Default,
    Ddi,
    Mobile,
    Fax,
    Office,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum PaymentTermType {
    Daysafterbilldate,
    Daysafterbillmonth,
    Ofcurrentmonth,
    Offollowingmonth,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum LineAmountType {
    Exclusive,
    Inclusive,
    NoTax,
    None, // FIX: Added this variant
}

// Custom deserializer to handle both "Exclusive" and "EXCLUSIVE" etc.
impl<'de> Deserialize<'de> for LineAmountType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_uppercase().as_str() {
            "EXCLUSIVE" => Ok(LineAmountType::Exclusive),
            "INCLUSIVE" => Ok(LineAmountType::Inclusive),
            "NOTAX" => Ok(LineAmountType::NoTax),
            "NONE" => Ok(LineAmountType::None), // FIX: Handle the "NONE" variant
            _ => Err(serde::de::Error::unknown_variant(
                &s,
                &["Exclusive", "Inclusive", "NoTax", "None"], // FIX: Update expected variants
            )),
        }
    }
}

// --- Shared Structs ---

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Address {
    pub address_type: AddressType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line3: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attention_to: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Phone {
    pub phone_type: PhoneType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_area_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_country_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct PaymentTerm {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day: Option<u32>,
    #[serde(rename = "Type")]
    pub term_type: PaymentTermType,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Link {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Allocation {
    #[serde(rename = "AllocationID", skip_serializing_if = "Option::is_none")]
    pub allocation_id: Option<Uuid>,
    pub amount: f64,
    #[serde(with = "xero_date_format")]
    pub date: DateTime<Utc>,
    pub invoice: InvoiceSummary,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct InvoiceSummary {
    #[serde(rename = "InvoiceID")]
    pub invoice_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_number: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "PascalCase")]
pub struct LineItem {
    #[serde(rename = "LineItemID", skip_serializing_if = "Option::is_none")]
    pub line_item_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_code: Option<String>,
    #[serde(rename = "AccountID", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_amount: Option<f64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tracking: Vec<TrackingCategory>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct TrackingCategory {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub option: String,
    #[serde(rename = "TrackingCategoryID", skip_serializing_if = "Option::is_none")]
    pub tracking_category_id: Option<Uuid>,
    #[serde(rename = "TrackingOptionID", skip_serializing_if = "Option::is_none")]
    pub tracking_option_id: Option<Uuid>,
}
