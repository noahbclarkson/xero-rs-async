//! Models for Payroll AU Payroll Calendars.

use crate::models::payroll_au::common::ExtraFields;
use crate::util::xero_date_format_opt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "PascalCase", default)]
pub struct PayrollCalendar {
    #[serde(rename = "PayrollCalendarID", skip_serializing_if = "Option::is_none")]
    pub payroll_calendar_id: Option<Uuid>,
    pub name: Option<String>,
    pub calendar_type: Option<String>,
    #[serde(with = "xero_date_format_opt", default)]
    pub start_date: Option<DateTime<Utc>>,
    #[serde(with = "xero_date_format_opt", default)]
    pub payment_date: Option<DateTime<Utc>>,
    #[serde(with = "xero_date_format_opt", default)]
    pub reference_date: Option<DateTime<Utc>>,
    #[serde(rename = "UpdatedDateUTC", with = "xero_date_format_opt", default)]
    pub updated_date_utc: Option<DateTime<Utc>>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PayrollCalendarsResponse {
    pub payroll_calendars: Vec<PayrollCalendar>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PayrollCalendarsRequest {
    pub payroll_calendars: Vec<PayrollCalendar>,
}
