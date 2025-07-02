//! Model for the Attachment resource.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Attachment {
    #[serde(rename = "AttachmentID")]
    pub attachment_id: Uuid,
    pub file_name: String,
    pub url: String,
    pub mime_type: String,
    pub content_length: String, // Note: API returns this as a string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_online: Option<bool>,
}

// Wrapper for the response
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct AttachmentsResponse {
    pub attachments: Vec<Attachment>,
}
