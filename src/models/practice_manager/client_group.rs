//! Client Group models for the XPM Practice Manager API v3.1.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::common::ClientRef;

// ---------------------------------------------------------------------------
// Response wrappers
// ---------------------------------------------------------------------------

/// `GET clientgroup.api/list` — list of all client groups.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "Response")]
pub struct ClientGroupsResponse {
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Groups")]
    pub groups: Option<ClientGroupList>,
}

/// Inner wrapper for `<Groups>` containing many `<Group>` elements.
#[derive(Debug, Clone, Deserialize)]
pub struct ClientGroupList {
    #[serde(rename = "Group", default)]
    pub items: Vec<ClientGroup>,
}

/// `GET clientgroup.api/get/[uuid]` — single client group detail.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "Response")]
pub struct ClientGroupResponse {
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Group")]
    pub group: Option<ClientGroup>,
}

// ---------------------------------------------------------------------------
// Main struct
// ---------------------------------------------------------------------------

/// A client group, with optional nested client members.
#[derive(Debug, Clone, Deserialize)]
pub struct ClientGroup {
    #[serde(rename = "UUID")]
    pub uuid: Uuid,
    #[serde(rename = "Name")]
    pub name: String,
    /// Returned if the Practice Management module is enabled (Yes/No).
    #[serde(rename = "Taxable")]
    pub taxable: Option<String>,
    #[serde(rename = "Clients")]
    pub clients: Option<ClientGroupClientList>,
}

/// Inner wrapper for the list of clients inside a group detail response.
#[derive(Debug, Clone, Deserialize)]
pub struct ClientGroupClientList {
    #[serde(rename = "Client", default)]
    pub items: Vec<ClientRef>,
}

/// Alias for `ClientGroupsResponse` used by the API layer.
pub type GroupsResponse = ClientGroupsResponse;

/// Alias for `ClientGroupResponse` used by the API layer.
pub type GroupResponse = ClientGroupResponse;

// ---------------------------------------------------------------------------
// Request types
// ---------------------------------------------------------------------------

/// Request body for `POST clientgroup.api/add`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "Group")]
pub struct AddGroupRequest {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Taxable", skip_serializing_if = "Option::is_none")]
    pub taxable: Option<String>,
    #[serde(rename = "ClientUUID", skip_serializing_if = "Option::is_none")]
    pub client_uuid: Option<Uuid>,
}

/// Request body for `POST clientgroup.api/delete`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "Group")]
pub struct DeleteGroupRequest {
    #[serde(rename = "UUID")]
    pub uuid: Uuid,
}

/// Request body for `PUT clientgroup.api/members`.
///
/// The XPM API uses `<add uuid="..."/>` and `<remove uuid="..."/>` attribute
/// syntax which cannot be directly expressed via serde. Callers should build
/// the XML manually for this endpoint. This struct captures the group UUID
/// and the lists of clients to add/remove so the caller has the data model.
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "Group")]
pub struct UpdateMembersRequest {
    #[serde(rename = "UUID")]
    pub uuid: Uuid,
    /// UUIDs of clients to add to the group.
    #[serde(skip)]
    pub add: Vec<Uuid>,
    /// UUIDs of clients to remove from the group.
    #[serde(skip)]
    pub remove: Vec<Uuid>,
}
