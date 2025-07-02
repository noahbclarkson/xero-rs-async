//! Model for the ContactGroup resource.

use super::contact::Contact;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum ContactGroupStatus {
    Active,
    Deleted,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ContactGroup {
    #[serde(rename = "ContactGroupID", skip_serializing_if = "Option::is_none")]
    pub contact_group_id: Option<Uuid>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ContactGroupStatus>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contacts: Vec<Contact>,
}

// Wrapper for the response
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct ContactGroupsResponse {
    pub contact_groups: Vec<ContactGroup>,
}

// Wrapper for the request
#[allow(dead_code)]
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct ContactGroupsRequest {
    pub contact_groups: Vec<ContactGroup>,
}
