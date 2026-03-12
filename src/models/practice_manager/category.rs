//! Category models for the XPM Practice Manager API v3.1.

use serde::Deserialize;
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Response wrappers
// ---------------------------------------------------------------------------

/// `GET category.api/list` — list of all job categories.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "Response")]
pub struct CategoriesResponse {
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Categories")]
    pub categories: Option<CategoryList>,
}

/// Inner wrapper for `<Categories>`.
#[derive(Debug, Clone, Deserialize)]
pub struct CategoryList {
    #[serde(rename = "Category", default)]
    pub items: Vec<Category>,
}

// ---------------------------------------------------------------------------
// Main struct
// ---------------------------------------------------------------------------

/// A job category in XPM.
#[derive(Debug, Clone, Deserialize)]
pub struct Category {
    #[serde(rename = "UUID")]
    pub uuid: Uuid,
    #[serde(rename = "Name")]
    pub name: String,
}
