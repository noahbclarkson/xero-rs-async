//! Model for the HistoryRecord resource.
use crate::util::xero_date_format;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct HistoryRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changes: Option<String>,
    #[serde(with = "xero_date_format", default)]
    #[serde(rename = "DateUTC")] // Fix: API uses all-caps UTC
    pub date_utc: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    pub details: String,
}

// Wrapper for the response
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct HistoryRecordsResponse {
    pub history_records: Vec<HistoryRecord>,
}

// Wrapper for the request
#[allow(dead_code)]
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct HistoryRecordsRequest {
    pub history_records: Vec<HistoryRecord>,
}
