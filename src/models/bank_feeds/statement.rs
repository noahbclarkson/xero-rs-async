//! Models for Bank Feeds Statements.

use crate::util::xero_naive_date_format_opt;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Balance {
    pub amount: Option<String>,
    pub credit_debit_indicator: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct StatementLine {
    pub amount: Option<String>,
    pub credit_debit_indicator: Option<String>,
    pub description: Option<String>,
    pub transaction_id: Option<String>,
    #[serde(with = "xero_naive_date_format_opt", default)]
    pub posted_date: Option<NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Statement {
    pub id: Option<Uuid>,
    pub feed_connection_id: Option<Uuid>,
    pub status: Option<String>,
    #[serde(with = "xero_naive_date_format_opt", default)]
    pub start_date: Option<NaiveDate>,
    #[serde(with = "xero_naive_date_format_opt", default)]
    pub end_date: Option<NaiveDate>,
    pub start_balance: Option<Balance>,
    pub end_balance: Option<Balance>,
    pub statement_line_count: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub statement_lines: Vec<StatementLine>,
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
pub struct StatementsResponse {
    pub pagination: Option<Pagination>,
    pub items: Option<Vec<Statement>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatementsCreateRequest {
    pub items: Vec<Statement>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatementsCreateResponse {
    pub items: Option<Vec<StatementCreateResult>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct StatementCreateResult {
    pub id: Option<Uuid>,
    pub status: Option<String>,
    pub errors: Option<Vec<serde_json::Value>>,
}
