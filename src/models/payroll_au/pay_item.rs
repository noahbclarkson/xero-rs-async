//! Models for Payroll AU Pay Items (v1 endpoint).

use crate::models::payroll_au::common::ExtraFields;
use crate::util::xero_date_format_opt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct PayItemEarningsRate {
    #[serde(rename = "EarningsRateID", skip_serializing_if = "Option::is_none")]
    pub earnings_rate_id: Option<Uuid>,
    pub name: Option<String>,
    pub earnings_type: Option<String>,
    pub rate_type: Option<String>,
    pub account_code: Option<String>,
    pub type_of_units: Option<String>,
    pub rate_per_unit: Option<f64>,
    pub multiplier: Option<f64>,
    pub accrue_leave: Option<bool>,
    pub amount: Option<f64>,
    pub is_exempt_from_tax: Option<bool>,
    pub is_exempt_from_super: Option<bool>,
    pub is_reportable_as_w1: Option<bool>,
    pub current_record: Option<bool>,
    #[serde(rename = "UpdatedDateUTC", with = "xero_date_format_opt", default)]
    pub updated_date_utc: Option<DateTime<Utc>>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct PayItemDeductionType {
    #[serde(rename = "DeductionTypeID", skip_serializing_if = "Option::is_none")]
    pub deduction_type_id: Option<Uuid>,
    pub name: Option<String>,
    pub deduction_category: Option<String>,
    pub account_code: Option<String>,
    pub reduces_tax: Option<bool>,
    pub reduces_super: Option<bool>,
    pub is_exempt_from_w1: Option<bool>,
    pub current_record: Option<bool>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct PayItemLeaveType {
    #[serde(rename = "LeaveTypeID", skip_serializing_if = "Option::is_none")]
    pub leave_type_id: Option<Uuid>,
    pub name: Option<String>,
    pub type_of_units: Option<String>,
    pub is_paid_leave: Option<bool>,
    pub show_on_payslip: Option<bool>,
    pub normal_entitlement: Option<f64>,
    pub leave_category_code: Option<String>,
    pub current_record: Option<bool>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct PayItemReimbursementType {
    #[serde(
        rename = "ReimbursementTypeID",
        skip_serializing_if = "Option::is_none"
    )]
    pub reimbursement_type_id: Option<Uuid>,
    pub name: Option<String>,
    pub account_code: Option<String>,
    pub current_record: Option<bool>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct PayItems {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub earnings_rates: Vec<PayItemEarningsRate>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub deduction_types: Vec<PayItemDeductionType>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub leave_types: Vec<PayItemLeaveType>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reimbursement_types: Vec<PayItemReimbursementType>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PayItemsResponse {
    pub pay_items: Option<PayItems>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PayItemsRequest {
    pub pay_items: PayItems,
}
