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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(rename = "Id")]
    pub id: Uuid,
}

// Wrapper for the response (can be an object or a raw list).
#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum FoldersResponse {
    Wrapper {
        #[serde(rename = "Items")]
        folders: Vec<Folder>,
    },
    List(Vec<Folder>),
}

impl FoldersResponse {
    pub fn into_vec(self) -> Vec<Folder> {
        match self {
            Self::Wrapper { folders } => folders,
            Self::List(items) => items,
        }
    }
}
