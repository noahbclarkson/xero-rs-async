//! Model for the User resource.

use crate::util::xero_date_format_opt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum UserRole {
    Readonly,
    Invoiceonly,
    Standard,
    Financialadviser,
    Managedclient,
    Cashbookclient,
    Admin,
    Removed,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct User {
    // FIX: Made optional as API sometimes returns GlobalUserID instead
    #[serde(rename = "UserID", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Uuid>,
    // FIX: Added GlobalUserID to handle cases where UserID is missing
    #[serde(rename = "GlobalUserID", skip_serializing_if = "Option::is_none")]
    pub global_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub email_address: Option<String>,
    pub first_name: String,
    pub last_name: String,
    #[serde(with = "xero_date_format_opt", default, rename = "UpdatedDateUTC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_date_utc: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub is_subscriber: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub organisation_role: Option<UserRole>,
}

// Wrapper for the response
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct UsersResponse {
    pub users: Vec<User>,
}