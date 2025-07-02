//! Model for the ManualJournal resource.

use super::common::{LineAmountType, TrackingCategory};
use crate::util::xero_date_format_opt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum ManualJournalStatus {
    Draft,
    Posted,
    Deleted,
    Voided,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ManualJournal {
    pub narration: String,
    pub journal_lines: Vec<ManualJournalLine>,
    #[serde(with = "xero_date_format_opt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_amount_types: Option<LineAmountType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ManualJournalStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_on_cash_basis_reports: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_attachments: Option<bool>,
    #[serde(with = "xero_date_format_opt", default, rename = "UpdatedDateUTC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_date_utc: Option<DateTime<Utc>>,
    #[serde(rename = "ManualJournalID", skip_serializing_if = "Option::is_none")]
    pub manual_journal_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ManualJournalLine {
    pub line_amount: f64,
    pub account_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_type: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tracking: Vec<TrackingCategory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_amount: Option<f64>,
}

// Wrapper for the response
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct ManualJournalsResponse {
    pub manual_journals: Vec<ManualJournal>,
}

// Wrapper for the request
#[allow(dead_code)]
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct ManualJournalsRequest {
    pub manual_journals: Vec<ManualJournal>,
}
