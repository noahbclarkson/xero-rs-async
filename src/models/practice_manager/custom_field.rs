//! Custom field models for the XPM Practice Manager API v3.1.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Response wrappers — definitions
// ---------------------------------------------------------------------------

/// `GET customfield.api/definition` — list of all custom field definitions.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "Response")]
pub struct CustomFieldDefinitionsResponse {
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "CustomFieldDefinitions")]
    pub definitions: Option<CustomFieldDefinitionList>,
}

/// Inner wrapper for `<CustomFieldDefinitions>`.
#[derive(Debug, Clone, Deserialize)]
pub struct CustomFieldDefinitionList {
    #[serde(rename = "CustomFieldDefinition", default)]
    pub items: Vec<CustomFieldDefinition>,
}

/// `GET customfield.api/get/[uuid]` — single custom field definition.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "Response")]
pub struct CustomFieldDefinitionResponse {
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "CustomFieldDefinition")]
    pub definition: Option<CustomFieldDefinition>,
}

// ---------------------------------------------------------------------------
// Response wrappers — values
// ---------------------------------------------------------------------------

/// Response for `GET .../customfield` — custom field values for an entity.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "Response")]
pub struct CustomFieldsResponse {
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "CustomFields")]
    pub custom_fields: Option<CustomFieldValueList>,
}

/// Inner wrapper for `<CustomFields>`.
#[derive(Debug, Clone, Deserialize)]
pub struct CustomFieldValueList {
    #[serde(rename = "CustomField", default)]
    pub items: Vec<CustomFieldValue>,
}

// ---------------------------------------------------------------------------
// Main structs
// ---------------------------------------------------------------------------

/// A custom field definition in XPM.
#[derive(Debug, Clone, Deserialize)]
pub struct CustomFieldDefinition {
    #[serde(rename = "UUID")]
    pub uuid: Uuid,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    /// e.g. Text, Decimal, Date, Dropdown List, Value Link, etc.
    #[serde(rename = "Type")]
    pub field_type: Option<String>,
    /// URL for Value Link field types.
    #[serde(rename = "LinkUrl")]
    pub link_url: Option<String>,
    /// Options for Dropdown lists.
    #[serde(rename = "Options")]
    pub options: Option<String>,
    #[serde(rename = "UseClient")]
    pub use_client: Option<String>,
    #[serde(rename = "UseContact")]
    pub use_contact: Option<String>,
    #[serde(rename = "UseJob")]
    pub use_job: Option<String>,
    #[serde(rename = "UseJobTask")]
    pub use_job_task: Option<String>,
    #[serde(rename = "UseJobCost")]
    pub use_job_cost: Option<String>,
    #[serde(rename = "UseJobTime")]
    pub use_job_time: Option<String>,
    /// Identifies the XML element for the field value: Text, Decimal, Number,
    /// Boolean, or Date.
    #[serde(rename = "ValueElement")]
    pub value_element: Option<String>,
}

/// A custom field value for a specific entity (client, contact, job, etc.).
///
/// Only one of `text`, `number`, `decimal`, `boolean`, `date` will be populated,
/// depending on the field type.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CustomFieldValue {
    #[serde(rename = "UUID")]
    pub uuid: Uuid,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Text")]
    pub text: Option<String>,
    #[serde(rename = "Number")]
    pub number: Option<String>,
    #[serde(rename = "Decimal")]
    pub decimal: Option<String>,
    #[serde(rename = "Boolean")]
    pub boolean: Option<String>,
    #[serde(rename = "Date")]
    pub date: Option<String>,
}

// ---------------------------------------------------------------------------
// Request types — definitions
// ---------------------------------------------------------------------------

/// Request body for `POST customfield.api/add`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "CustomFieldDefinition")]
pub struct AddCustomFieldDefinitionRequest {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub field_type: Option<String>,
    #[serde(rename = "LinkUrl", skip_serializing_if = "Option::is_none")]
    pub link_url: Option<String>,
    #[serde(rename = "Options", skip_serializing_if = "Option::is_none")]
    pub options: Option<String>,
    #[serde(rename = "UseClient", skip_serializing_if = "Option::is_none")]
    pub use_client: Option<String>,
    #[serde(rename = "UseContact", skip_serializing_if = "Option::is_none")]
    pub use_contact: Option<String>,
    #[serde(rename = "UseJob", skip_serializing_if = "Option::is_none")]
    pub use_job: Option<String>,
    #[serde(rename = "UseJobTask", skip_serializing_if = "Option::is_none")]
    pub use_job_task: Option<String>,
    #[serde(rename = "UseJobCost", skip_serializing_if = "Option::is_none")]
    pub use_job_cost: Option<String>,
    #[serde(rename = "UseJobTime", skip_serializing_if = "Option::is_none")]
    pub use_job_time: Option<String>,
}

/// Request body for `PUT customfield.api/update`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "CustomFieldDefinition")]
pub struct UpdateCustomFieldDefinitionRequest {
    #[serde(rename = "UUID")]
    pub uuid: Uuid,
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub field_type: Option<String>,
    #[serde(rename = "LinkUrl", skip_serializing_if = "Option::is_none")]
    pub link_url: Option<String>,
    #[serde(rename = "Options", skip_serializing_if = "Option::is_none")]
    pub options: Option<String>,
    #[serde(rename = "UseClient", skip_serializing_if = "Option::is_none")]
    pub use_client: Option<String>,
    #[serde(rename = "UseContact", skip_serializing_if = "Option::is_none")]
    pub use_contact: Option<String>,
    #[serde(rename = "UseJob", skip_serializing_if = "Option::is_none")]
    pub use_job: Option<String>,
    #[serde(rename = "UseJobTask", skip_serializing_if = "Option::is_none")]
    pub use_job_task: Option<String>,
    #[serde(rename = "UseJobCost", skip_serializing_if = "Option::is_none")]
    pub use_job_cost: Option<String>,
    #[serde(rename = "UseJobTime", skip_serializing_if = "Option::is_none")]
    pub use_job_time: Option<String>,
}

/// Request body for `POST customfield.api/delete`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "CustomFieldDefinition")]
pub struct DeleteCustomFieldDefinitionRequest {
    #[serde(rename = "UUID")]
    pub uuid: Uuid,
}

// ---------------------------------------------------------------------------
// Request types — values
// ---------------------------------------------------------------------------

/// Wrapper for custom field value entries in update requests.
#[derive(Debug, Clone, Serialize)]
pub struct UpdateCustomFieldValueEntry {
    #[serde(rename = "UUID")]
    pub uuid: Uuid,
    #[serde(rename = "Text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "Number", skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(rename = "Decimal", skip_serializing_if = "Option::is_none")]
    pub decimal: Option<String>,
    #[serde(rename = "Boolean", skip_serializing_if = "Option::is_none")]
    pub boolean: Option<String>,
    #[serde(rename = "Date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
}

/// Inner list of custom field values for update.
#[derive(Debug, Clone, Serialize)]
pub struct UpdateCustomFieldValueListInner {
    #[serde(rename = "CustomField")]
    pub items: Vec<UpdateCustomFieldValueEntry>,
}

/// Request body for `PUT .../customfield` — update custom field values.
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "CustomFields")]
pub struct UpdateCustomFieldsRequest {
    #[serde(rename = "CustomField")]
    pub items: Vec<UpdateCustomFieldValueEntry>,
}
