//! Models for Payroll UK PayRunCalendars.

use super::common::GenericRecord;
use serde::{Deserialize, Serialize};

pub type PayRunCalendar = GenericRecord;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayRunCalendarResponse {
    pub pay_run_calendars: Option<Vec<PayRunCalendar>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PayRunCalendarRequest {
    pub pay_run_calendars: Vec<PayRunCalendar>,
}
