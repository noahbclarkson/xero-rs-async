//! Models for Project Time Entries.

use crate::util::iso_datetime_format_opt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct TimeEntry {
    pub time_entry_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub project_id: Option<Uuid>,
    pub task_id: Option<Uuid>,
    #[serde(with = "iso_datetime_format_opt", default)]
    pub date_utc: Option<DateTime<Utc>>,
    #[serde(with = "iso_datetime_format_opt", default)]
    pub date_entered_utc: Option<DateTime<Utc>>,
    pub duration: Option<i64>,
    pub description: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub page_count: Option<u32>,
    pub item_count: Option<u32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeEntriesResponse {
    pub pagination: Option<Pagination>,
    pub items: Option<Vec<TimeEntry>>,
}
