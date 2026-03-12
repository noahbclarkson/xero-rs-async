//! Models for Payroll NZ Salary and Wages.

use crate::util::iso_datetime_format_opt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct SalaryAndWage {
    #[serde(rename = "salaryAndWagesID", skip_serializing_if = "Option::is_none")]
    pub salary_and_wages_id: Option<Uuid>,
    #[serde(rename = "earningsRateID", skip_serializing_if = "Option::is_none")]
    pub earnings_rate_id: Option<Uuid>,
    pub number_of_units_per_week: Option<f64>,
    pub rate_per_unit: Option<f64>,
    pub number_of_units_per_day: Option<f64>,
    #[serde(with = "iso_datetime_format_opt", default)]
    pub effective_from: Option<DateTime<Utc>>,
    pub annual_salary: Option<f64>,
    pub status: Option<String>,
    pub payment_type: Option<String>,
    pub days_per_week: Option<f64>,
    pub work_pattern_type: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SalaryAndWagesResponse {
    pub salary_and_wages: Option<Vec<SalaryAndWage>>,
}
