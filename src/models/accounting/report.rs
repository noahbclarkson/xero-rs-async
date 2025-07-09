//! Model for the Report resource.

use crate::util::xero_date_format_opt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Report {
    #[serde(rename = "ReportID")]
    pub report_id: String,
    pub report_name: String,
    pub report_type: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub report_titles: Vec<String>,
    pub report_date: String,
    #[serde(with = "xero_date_format_opt", default, rename = "UpdatedDateUTC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_date_utc: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rows: Vec<Box<ReportRow>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<ReportField>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attributes: Vec<ReportAttribute>,
    // For 1099 Report
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contacts: Vec<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ReportRow {
    pub row_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cells: Vec<Box<ReportCell>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rows: Vec<Box<ReportRow>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ReportCell {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attributes: Vec<ReportAttribute>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ReportAttribute {
    pub value: String,
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ReportField {
    #[serde(rename = "FieldID")]
    pub field_id: String,
    pub description: String,
    pub value: String,
}

// Wrapper for the response
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct ReportsResponse {
    pub reports: Vec<Report>,
}
