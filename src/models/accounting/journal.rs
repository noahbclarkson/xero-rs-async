//! Model for the Journal resource.

use super::account::AccountType;
use super::common::TrackingCategory;
use crate::util::{xero_date_format, xero_date_format_opt};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum JournalSourceType {
    Accrec,
    Accpay,
    Accreccredit,
    Accpaycredit,
    Accrecpayment,
    Accpaypayment,
    Arcreditpayment,
    Apcreditpayment,
    Cashrec,
    Cashpaid,
    Transfer,
    Arprepayment,
    Apprepayment,
    Aroverpayment,
    Apoverpayment,
    Expclaim,
    Exppayment,
    Manjournal,
    Payslip,
    Wagepayable,
    Integratedpayrollpe,
    Integratedpayrollpt,
    Externalspendmoney,
    Integratedpayrollptpayment,
    Integratedpayrollcn,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Journal {
    #[serde(rename = "JournalID")]
    pub journal_id: Uuid,
    #[serde(with = "xero_date_format")]
    pub journal_date: DateTime<Utc>,
    pub journal_number: i32,
    #[serde(with = "xero_date_format_opt", default, rename = "CreatedDateUTC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_utc: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(rename = "SourceID", skip_serializing_if = "Option::is_none")]
    pub source_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<JournalSourceType>,
    pub journal_lines: Vec<JournalLine>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct JournalLine {
    #[serde(rename = "JournalLineID")]
    pub journal_line_id: Uuid,
    #[serde(rename = "AccountID")]
    pub account_id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<AccountType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub net_amount: f64,
    pub gross_amount: f64,
    pub tax_amount: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tracking_categories: Vec<TrackingCategory>,
}

// Wrapper for the response
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct JournalsResponse {
    pub journals: Vec<Journal>,
}
