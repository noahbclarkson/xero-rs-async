//! Models for Payroll AU Earnings Rates (v2 endpoint).

use crate::models::payroll_au::common::ExtraFields;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct EarningsRate {
    #[serde(rename = "earningsRateID", skip_serializing_if = "Option::is_none")]
    pub earnings_rate_id: Option<Uuid>,
    pub name: Option<String>,
    pub earnings_type: Option<String>,
    pub rate_type: Option<String>,
    pub type_of_units: Option<String>,
    pub current_record: Option<bool>,
    #[serde(rename = "expenseAccountID", skip_serializing_if = "Option::is_none")]
    pub expense_account_id: Option<Uuid>,
    pub rate_per_unit: Option<f64>,
    pub multiple_of_ordinary_earnings_rate: Option<f64>,
    pub fixed_amount: Option<f64>,
    pub is_subject_to_tax: Option<bool>,
    pub is_subject_to_super: Option<bool>,
    pub is_reportable_as_w1: Option<bool>,
    pub accrue_leave: Option<bool>,
    pub employment_termination_payment_type: Option<String>,
    pub allowance_type: Option<String>,
    pub allowance_category: Option<String>,
    pub allowance_contributes_to_annual_leave_rate: Option<bool>,
    pub allowance_contributes_to_overtime_rate: Option<bool>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarningsRatesResponse {
    pub earnings_rates: Option<Vec<EarningsRate>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EarningsRatesRequest {
    pub earnings_rates: Vec<EarningsRate>,
}
