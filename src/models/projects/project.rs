//! Models for Projects.

use crate::util::iso_datetime_format_opt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct MoneyAmount {
    pub currency: Option<String>,
    pub value: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Project {
    pub project_id: Option<Uuid>,
    pub contact_id: Option<Uuid>,
    pub name: Option<String>,
    pub currency_code: Option<String>,
    pub minutes_logged: Option<i64>,
    pub minutes_to_be_invoiced: Option<i64>,
    pub total_task_amount: Option<MoneyAmount>,
    pub total_expense_amount: Option<MoneyAmount>,
    pub task_amount_to_be_invoiced: Option<MoneyAmount>,
    pub task_amount_invoiced: Option<MoneyAmount>,
    pub expense_amount_to_be_invoiced: Option<MoneyAmount>,
    pub expense_amount_invoiced: Option<MoneyAmount>,
    pub project_amount_invoiced: Option<MoneyAmount>,
    pub deposit: Option<MoneyAmount>,
    pub deposit_applied: Option<MoneyAmount>,
    pub credit_note_amount: Option<MoneyAmount>,
    pub total_invoiced: Option<MoneyAmount>,
    pub total_to_be_invoiced: Option<MoneyAmount>,
    pub estimate: Option<MoneyAmount>,
    pub status: Option<String>,
    #[serde(with = "iso_datetime_format_opt", default)]
    pub deadline_utc: Option<DateTime<Utc>>,
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
pub struct ProjectsResponse {
    pub pagination: Option<Pagination>,
    pub items: Option<Vec<Project>>,
}
