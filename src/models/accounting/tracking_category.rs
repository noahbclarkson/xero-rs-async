//! Model for the TrackingCategory resource.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum TrackingCategoryStatus {
    Active,
    Archived,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct TrackingCategory {
    #[serde(rename = "TrackingCategoryID", skip_serializing_if = "Option::is_none")]
    pub tracking_category_id: Option<Uuid>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TrackingCategoryStatus>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<TrackingOption>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct TrackingOption {
    #[serde(rename = "TrackingOptionID", skip_serializing_if = "Option::is_none")]
    pub tracking_option_id: Option<Uuid>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TrackingCategoryStatus>,
}

// Wrapper for the response
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct TrackingCategoriesResponse {
    pub tracking_categories: Vec<TrackingCategory>,
}

// Wrapper for the request
#[allow(dead_code)]
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct TrackingCategoriesRequest {
    pub tracking_categories: Vec<TrackingCategory>,
}

// Wrapper for the options response
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct TrackingOptionsResponse {
    pub options: Vec<TrackingOption>,
}

// Wrapper for the options request
#[allow(dead_code)]
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct TrackingOptionsRequest {
    pub options: Vec<TrackingOption>,
}
