//! Model for the BrandingTheme resource.

use crate::util::xero_date_format_opt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum BrandingThemeType {
    Invoice,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BrandingTheme {
    #[serde(rename = "BrandingThemeID")]
    pub branding_theme_id: Uuid,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    #[serde(rename = "Type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_type: Option<BrandingThemeType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<i32>,
    #[serde(with = "xero_date_format_opt", default, rename = "CreatedDateUTC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date_utc: Option<DateTime<Utc>>,
}

// Wrapper for the response
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct BrandingThemesResponse {
    pub branding_themes: Vec<BrandingTheme>,
}
