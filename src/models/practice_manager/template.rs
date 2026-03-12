//! Template models for the XPM Practice Manager API v3.1.

use serde::Deserialize;
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Response wrappers
// ---------------------------------------------------------------------------

/// `GET template.api/list` — list of all job templates.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "Response")]
pub struct TemplatesResponse {
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Templates")]
    pub templates: Option<TemplateList>,
}

/// Inner wrapper for `<Templates>`.
#[derive(Debug, Clone, Deserialize)]
pub struct TemplateList {
    #[serde(rename = "Template", default)]
    pub items: Vec<Template>,
}

// ---------------------------------------------------------------------------
// Main struct
// ---------------------------------------------------------------------------

/// A job template in XPM.
#[derive(Debug, Clone, Deserialize)]
pub struct Template {
    #[serde(rename = "UUID")]
    pub uuid: Uuid,
    #[serde(rename = "Name")]
    pub name: String,
}
