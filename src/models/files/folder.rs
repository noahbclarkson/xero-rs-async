//! Model for the Folder resource.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Folder {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub is_inbox: bool,
    #[serde(rename = "Id")]
    pub id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct User {
    pub name: String,
    pub first_name: String,
    pub last_name: String,
    pub full_name: String,
    #[serde(rename = "Id")]
    pub id: Uuid,
}

// Wrapper for the response
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct FoldersResponse {
    #[serde(rename = "Items")]
    pub folders: Vec<Folder>,
}
