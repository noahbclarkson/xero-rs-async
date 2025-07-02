//! Model for the Association resource.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Association {
    #[serde(rename = "FileId")]
    pub file_id: Uuid,
    #[serde(rename = "ObjectId")]
    pub object_id: Uuid,
    pub object_group: String,
    #[serde(rename = "ObjectType", skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_with_object: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
}

pub type AssociationCount = HashMap<Uuid, u32>;

// Wrapper for the response
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct AssociationsResponse {
    #[serde(rename = "Items")]
    pub associations: Vec<Association>,
}
