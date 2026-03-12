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
    #[serde(
        default,
        deserialize_with = "crate::util::deserialize_opt_u64_from_string_or_number"
    )]
    pub content_length: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_online: Option<bool>,
}

// Wrapper for the response
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct AttachmentsResponse {
    pub attachments: Vec<Attachment>,
}

#[cfg(test)]
mod tests {
    use super::AttachmentsResponse;

    #[test]
    fn deserializes_numeric_content_length() {
        let payload = r#"{
            "Attachments": [{
                "AttachmentID": "44dc5d9e-7488-4db1-8185-177be8ab7c6f",
                "FileName": "invoice.pdf",
                "Url": "https://example.com/invoice.pdf",
                "MimeType": "application/pdf",
                "ContentLength": 256570
            }]
        }"#;

        let parsed: AttachmentsResponse =
            serde_json::from_str(payload).expect("attachments response should deserialize");
        assert_eq!(parsed.attachments.len(), 1);
        assert_eq!(parsed.attachments[0].content_length, Some(256_570));
    }

    #[test]
    fn deserializes_string_content_length() {
        let payload = r#"{
            "Attachments": [{
                "AttachmentID": "44dc5d9e-7488-4db1-8185-177be8ab7c6f",
                "FileName": "invoice.pdf",
                "Url": "https://example.com/invoice.pdf",
                "MimeType": "application/pdf",
                "ContentLength": "256570"
            }]
        }"#;

        let parsed: AttachmentsResponse =
            serde_json::from_str(payload).expect("attachments response should deserialize");
        assert_eq!(parsed.attachments[0].content_length, Some(256_570));
    }
}
