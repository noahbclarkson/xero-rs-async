//! Models for Payroll UK Tracking Categories.

use super::common::GenericRecord;
use serde::{Deserialize, Serialize};

pub type TrackingCategory = GenericRecord;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingCategoryResponse {
    pub tracking_categories: Option<Vec<TrackingCategory>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingCategoryRequest {
    pub tracking_categories: Vec<TrackingCategory>,
}
