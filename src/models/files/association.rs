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

// Wrapper for the response (can be an object or a raw list).
#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum AssociationsResponse {
    Wrapper {
        #[serde(rename = "Items")]
        associations: Vec<Association>,
    },
    List(Vec<Association>),
}

impl AssociationsResponse {
    pub fn into_vec(self) -> Vec<Association> {
        match self {
            Self::Wrapper { associations } => associations,
            Self::List(items) => items,
        }
    }
}
