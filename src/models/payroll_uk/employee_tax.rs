//! Models for Payroll UK Employee Tax.

use crate::util::iso_datetime_format_opt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct EmployeeTax {
    pub starter_type: Option<String>,
    pub starter_declaration: Option<String>,
    pub tax_code: Option<String>,
    pub w1_m1: Option<bool>,
    pub previous_taxable_pay: Option<f64>,
    pub previous_tax_paid: Option<f64>,
    pub student_loan_deduction: Option<String>,
    pub has_post_graduate_loans: Option<bool>,
    pub is_director: Option<bool>,
    #[serde(with = "iso_datetime_format_opt", default)]
    pub directorship_start_date: Option<DateTime<Utc>>,
    pub nic_calculation_method: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmployeeTaxResponse {
    pub employee_tax: Option<EmployeeTax>,
}
