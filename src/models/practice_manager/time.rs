//! Time entry models for the XPM Practice Manager API v3.1.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::common::StaffRef;

// ---------------------------------------------------------------------------
// Response wrappers
// ---------------------------------------------------------------------------

/// Response for list/job/staff endpoints returning multiple time entries.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "Response")]
pub struct TimesResponse {
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Times")]
    pub times: Option<TimeList>,
}

/// Inner wrapper for `<Times>`.
#[derive(Debug, Clone, Deserialize)]
pub struct TimeList {
    #[serde(rename = "Time", default)]
    pub items: Vec<TimeEntry>,
}

/// `GET time.api/get/[uuid]` — single time entry.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "Response")]
pub struct TimeResponse {
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Time")]
    pub time: Option<TimeEntry>,
}

// ---------------------------------------------------------------------------
// Main struct
// ---------------------------------------------------------------------------

/// A time sheet entry in XPM.
#[derive(Debug, Clone, Deserialize)]
pub struct TimeEntry {
    #[serde(rename = "UUID")]
    pub uuid: Uuid,
    #[serde(rename = "Job")]
    pub job: Option<TimeJobRef>,
    #[serde(rename = "Task")]
    pub task: Option<TimeTaskRef>,
    #[serde(rename = "Staff")]
    pub staff: Option<StaffRef>,
    #[serde(rename = "Date")]
    pub date: Option<String>,
    #[serde(rename = "Minutes")]
    pub minutes: Option<String>,
    #[serde(rename = "Note")]
    pub note: Option<String>,
    #[serde(rename = "Billable")]
    pub billable: Option<String>,
    #[serde(rename = "Start")]
    pub start: Option<String>,
    #[serde(rename = "End")]
    pub end: Option<String>,
    #[serde(rename = "InvoiceTaskUUID")]
    pub invoice_task_uuid: Option<String>,
}

/// Job reference inside a time entry (has ID + Name, not UUID).
#[derive(Debug, Clone, Deserialize)]
pub struct TimeJobRef {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Name")]
    pub name: String,
}

/// Task reference inside a time entry (has UUID + Name).
#[derive(Debug, Clone, Deserialize)]
pub struct TimeTaskRef {
    #[serde(rename = "UUID")]
    pub uuid: Uuid,
    #[serde(rename = "Name")]
    pub name: String,
}

// ---------------------------------------------------------------------------
// Request types
// ---------------------------------------------------------------------------

/// Request body for `POST time.api/add`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "Timesheet")]
pub struct AddTimesheetRequest {
    /// The job number, e.g. "J000309".
    #[serde(rename = "Job")]
    pub job: String,
    #[serde(rename = "TaskUUID")]
    pub task_uuid: Uuid,
    #[serde(rename = "StaffUUID")]
    pub staff_uuid: Uuid,
    #[serde(rename = "Date")]
    pub date: String,
    /// Duration in minutes. Use this OR start/end.
    #[serde(rename = "Minutes", skip_serializing_if = "Option::is_none")]
    pub minutes: Option<i64>,
    #[serde(rename = "Note", skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// Start time, e.g. "13:00". Use with `end` instead of `minutes`.
    #[serde(rename = "Start", skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    /// End time, e.g. "17:00". Use with `start` instead of `minutes`.
    #[serde(rename = "End", skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
}

/// Request body for `PUT time.api/update`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "Timesheet")]
pub struct UpdateTimesheetRequest {
    #[serde(rename = "UUID")]
    pub uuid: Uuid,
    #[serde(rename = "Job")]
    pub job: String,
    #[serde(rename = "TaskUUID")]
    pub task_uuid: Uuid,
    #[serde(rename = "StaffUUID")]
    pub staff_uuid: Uuid,
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "Minutes", skip_serializing_if = "Option::is_none")]
    pub minutes: Option<i64>,
    #[serde(rename = "Note", skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(rename = "Start", skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    #[serde(rename = "End", skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
}
