//! Model for the File resource.

use super::folder::User;
use crate::util::xero_date_format_opt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct File {
    pub name: String,
    pub mime_type: String,
    pub size: u64,
    #[serde(with = "xero_date_format_opt", default, rename = "CreatedDateUTC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_utc: Option<DateTime<Utc>>,
    #[serde(with = "xero_date_format_opt", default, rename = "UpdatedDateUTC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_date_utc: Option<DateTime<Utc>>,
    pub user: User,
    #[serde(rename = "FolderId", skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<Uuid>,
    #[serde(rename = "Id")]
    pub id: Uuid,
}

// Wrapper for the response
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct FilesResponse {
    pub items: Vec<File>,
}
