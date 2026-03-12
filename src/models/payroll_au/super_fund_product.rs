//! Models for Payroll AU Super Fund Products.

use crate::models::payroll_au::common::ExtraFields;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct SuperFundProduct {
    #[serde(rename = "ABN", skip_serializing_if = "Option::is_none")]
    pub abn: Option<String>,
    #[serde(rename = "USI", skip_serializing_if = "Option::is_none")]
    pub usi: Option<String>,
    #[serde(rename = "SPIN", skip_serializing_if = "Option::is_none")]
    pub spin: Option<String>,
    pub product_name: Option<String>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SuperFundProductsResponse {
    pub super_fund_products: Vec<SuperFundProduct>,
}
