//! Model for the Budget resource.

use super::tracking_category::TrackingCategory;
use crate::util::xero_date_format_opt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum BudgetType {
    Overall,
    Tracking,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Budget {
    #[serde(rename = "BudgetID")]
    pub budget_id: Uuid,
    #[serde(rename = "Type")]
    pub budget_type: BudgetType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(with = "xero_date_format_opt", default, rename = "UpdatedDateUTC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_date_utc: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tracking: Vec<TrackingCategory>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub budget_lines: Vec<BudgetLine>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BudgetLine {
    #[serde(rename = "AccountID")]
    pub account_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_code: Option<String>,
    #[serde(rename = "BudgetBalances")]
    pub balances: Vec<BudgetBalance>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BudgetBalance {
    pub period: String, // e.g., "2019-08"
    pub amount: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<f64>,
}

// Wrapper for the response
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct BudgetsResponse {
    pub budgets: Vec<Budget>,
}
