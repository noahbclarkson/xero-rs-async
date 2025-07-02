//! Model for the TaxRate resource.

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum TaxRateStatus {
    Active,
    Deleted,
    Archived,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct TaxRate {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_type: Option<String>,
    pub tax_components: Vec<TaxComponent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TaxRateStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_tax_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_apply_to_assets: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_apply_to_equity: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_apply_to_expenses: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_apply_to_liabilities: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_apply_to_revenue: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_tax_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_rate: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct TaxComponent {
    pub name: String,
    pub rate: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_compound: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_non_recoverable: Option<bool>,
}

// Wrapper for the response
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct TaxRatesResponse {
    pub tax_rates: Vec<TaxRate>,
}

// Wrapper for the request
#[allow(dead_code)]
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct TaxRatesRequest {
    pub tax_rates: Vec<TaxRate>,
}
