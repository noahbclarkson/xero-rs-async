//! Models for Payroll UK Employment.

use crate::util::iso_datetime_format_opt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct NiCategory {
    pub ni_category: Option<String>,
    #[serde(with = "iso_datetime_format_opt", default)]
    pub start_date: Option<DateTime<Utc>>,
    #[serde(with = "iso_datetime_format_opt", default)]
    pub date_first_employed_as_civilian: Option<DateTime<Utc>>,
    pub workplace_postcode: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct DevelopmentalRoleDetail {
    pub developmental_role_public_key: Option<String>,
    pub developmental_role: Option<String>,
    #[serde(with = "iso_datetime_format_opt", default)]
    pub start_date: Option<DateTime<Utc>>,
    #[serde(with = "iso_datetime_format_opt", default)]
    pub end_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Contract {
    pub public_key: Option<String>,
    pub is_fixed_term: Option<bool>,
    pub employment_status: Option<String>,
    pub contract_type: Option<String>,
    #[serde(with = "iso_datetime_format_opt", default)]
    pub start_date: Option<DateTime<Utc>>,
    #[serde(with = "iso_datetime_format_opt", default)]
    pub fixed_term_end_date: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub developmental_role_details: Vec<DevelopmentalRoleDetail>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Employment {
    #[serde(rename = "payrollCalendarID", skip_serializing_if = "Option::is_none")]
    pub payroll_calendar_id: Option<String>,
    #[serde(with = "iso_datetime_format_opt", default)]
    pub start_date: Option<DateTime<Utc>>,
    pub employee_number: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ni_categories: Vec<NiCategory>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contracts: Vec<Contract>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmploymentResponse {
    pub employment: Option<Employment>,
}
