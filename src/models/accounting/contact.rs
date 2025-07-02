//! Model for the Contact resource.

use super::common::{Address, LineAmountType, PaymentTerm, Phone, TrackingCategory};
use super::contact_group::ContactGroup;
use crate::util::xero_date_format_opt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum ContactStatus {
    Active,
    Archived,
    Gdprrequest,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Contact {
    #[serde(rename = "ContactID", skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_status: Option<ContactStatus>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub email_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub bank_account_details: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts_receivable_tax_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts_payable_tax_type: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub addresses: Vec<Address>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub phones: Vec<Phone>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_supplier: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_customer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_currency: Option<String>,
    #[serde(with = "xero_date_format_opt", default, rename = "UpdatedDateUTC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_date_utc: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact_persons: Vec<ContactPerson>,
    #[serde(rename = "XeroNetworkKey", skip_serializing_if = "Option::is_none")]
    pub xero_network_key: Option<String>,
    #[serde(rename = "MergedToContactID", skip_serializing_if = "Option::is_none")]
    pub merged_to_contact_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales_default_account_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchases_default_account_code: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sales_tracking_categories: Vec<TrackingCategory>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub purchases_tracking_categories: Vec<TrackingCategory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales_default_line_amount_type: Option<LineAmountType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchases_default_line_amount_type: Option<LineAmountType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_terms: Option<PaymentTerms>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contact_groups: Vec<ContactGroup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(rename = "BrandingTheme", skip_serializing_if = "Option::is_none")]
    pub branding_theme: Option<super::branding_theme::BrandingTheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_payments: Option<BatchPaymentDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balances: Option<ContactBalances>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_attachments: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ContactPerson {
    pub first_name: String,
    pub last_name: String,
    pub email_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_in_emails: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "PascalCase")]
pub struct PaymentTerms {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bills: Option<PaymentTerm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales: Option<PaymentTerm>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "PascalCase")]
pub struct BatchPaymentDetails {
    #[serde(rename = "BankAccountNumber")]
    pub bank_account_number: Option<String>,
    #[serde(rename = "Particulars")]
    pub particulars: Option<String>,
    #[serde(rename = "Code")]
    pub code: Option<String>,
    #[serde(rename = "Reference")]
    pub reference: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ContactBalances {
    pub accounts_receivable: BalanceDetails,
    pub accounts_payable: BalanceDetails,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "PascalCase")]
pub struct BalanceDetails {
    pub outstanding: f64,
    pub overdue: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "PascalCase")]
pub struct CISSettings {
    #[serde(rename = "CISEnabled")]
    pub cis_enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<f64>,
}

// Wrapper for the response
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct ContactsResponse {
    pub contacts: Vec<Contact>,
}

// Wrapper for the request
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct ContactsRequest {
    pub contacts: Vec<Contact>,
}

// Wrapper for the CIS settings response
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct CISSettingsResponse {
    #[serde(rename = "CISSettings")]
    pub cis_settings: Vec<CISSettings>,
}

// Wrapper for the CIS settings request
#[allow(dead_code)]
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct CISSettingsRequest {
    pub cis_settings: CISSettings,
}
