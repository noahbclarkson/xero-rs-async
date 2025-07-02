//! Model for the Organisation resource.

use super::common::{Address, Link, PaymentTerm, Phone};
use crate::util::xero_date_format_opt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Organisation {
    #[serde(rename = "APIKey", skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    pub name: String,
    pub legal_name: String,
    pub pays_tax: bool,
    pub version: String,
    pub organisation_type: String,
    pub base_currency: String,
    pub country_code: String,
    pub is_demo_company: bool,
    pub organisation_status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<String>,
    #[serde(
        rename = "EmployerIdentificationNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub employer_identification_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_number: Option<String>,
    pub financial_year_end_day: u32,
    pub financial_year_end_month: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales_tax_basis: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales_tax_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_sales_tax: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_purchases_tax: Option<String>,
    #[serde(with = "xero_date_format_opt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_lock_date: Option<DateTime<Utc>>,
    #[serde(with = "xero_date_format_opt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_of_year_lock_date: Option<DateTime<Utc>>,
    #[serde(with = "xero_date_format_opt", default, rename = "CreatedDateUTC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_utc: Option<DateTime<Utc>>,
    pub timezone: String,
    pub organisation_entity_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_code: Option<String>,
    #[serde(rename = "OrganisationID")]
    pub organisation_id: Uuid,
    pub edition: String,
    pub class: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_of_business: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub addresses: Vec<Address>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub phones: Vec<Phone>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub external_links: Vec<Link>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_terms: Option<OrganisationPaymentTerms>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct OrganisationPaymentTerms {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bills: Option<PaymentTerm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales: Option<PaymentTerm>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct OrganisationAction {
    pub name: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CISSettings {
    #[serde(rename = "CISContractorEnabled")]
    pub cis_contractor_enabled: bool,
    #[serde(rename = "CISSubContractorEnabled")]
    pub cis_sub_contractor_enabled: bool,
}

// Wrapper for the response
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct OrganisationsResponse {
    pub organisations: Vec<Organisation>,
}

// Wrapper for the actions response
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct ActionsResponse {
    pub actions: Vec<OrganisationAction>,
}
