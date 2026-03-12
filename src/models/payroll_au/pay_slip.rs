//! Models for Payroll AU Payslips.

use crate::models::payroll_au::common::ExtraFields;
use crate::util::xero_date_format_opt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct Payslip {
    #[serde(rename = "PayslipID", skip_serializing_if = "Option::is_none")]
    pub payslip_id: Option<Uuid>,
    #[serde(rename = "EmployeeID", skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<Uuid>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    #[serde(with = "xero_date_format_opt", default)]
    pub last_edited: Option<DateTime<Utc>>,
    pub wages: Option<f64>,
    pub deductions: Option<f64>,
    pub net_pay: Option<f64>,
    pub tax: Option<f64>,
    #[serde(rename = "Super", skip_serializing_if = "Option::is_none")]
    pub super_amount: Option<f64>,
    pub reimbursements: Option<f64>,
    #[serde(rename = "UpdatedDateUTC", with = "xero_date_format_opt", default)]
    pub updated_date_utc: Option<DateTime<Utc>>,
    #[serde(default)]
    pub earnings_lines: Vec<serde_json::Value>,
    #[serde(default)]
    pub leave_earnings_lines: Vec<serde_json::Value>,
    #[serde(default)]
    pub timesheet_earnings_lines: Vec<serde_json::Value>,
    #[serde(default)]
    pub deduction_lines: Vec<serde_json::Value>,
    #[serde(default)]
    pub leave_accrual_lines: Vec<serde_json::Value>,
    #[serde(default)]
    pub reimbursement_lines: Vec<serde_json::Value>,
    #[serde(default)]
    pub superannuation_lines: Vec<serde_json::Value>,
    #[serde(default)]
    pub tax_lines: Vec<serde_json::Value>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PayslipResponse {
    pub payslip: Option<Payslip>,
}
