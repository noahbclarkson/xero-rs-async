//! Models for Projects Tasks.

use crate::models::projects::project::{MoneyAmount, Pagination};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Task {
    pub name: Option<String>,
    pub rate: Option<MoneyAmount>,
    pub charge_type: Option<String>,
    pub estimate_minutes: Option<i64>,
    pub task_id: Option<Uuid>,
    pub project_id: Option<Uuid>,
    pub total_minutes: Option<i64>,
    pub total_amount: Option<MoneyAmount>,
    pub minutes_invoiced: Option<i64>,
    pub minutes_to_be_invoiced: Option<i64>,
    pub fixed_minutes: Option<i64>,
    pub non_chargeable_minutes: Option<i64>,
    pub amount_to_be_invoiced: Option<MoneyAmount>,
    pub amount_invoiced: Option<MoneyAmount>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TasksResponse {
    pub pagination: Option<Pagination>,
    pub items: Option<Vec<Task>>,
}
