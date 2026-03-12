//! Models for Bank Feeds Feed Connections.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct FeedConnection {
    pub id: Option<Uuid>,
    pub account_token: Option<String>,
    pub account_type: Option<String>,
    pub account_number: Option<String>,
    pub account_name: Option<String>,
    pub account_id: Option<Uuid>,
    pub currency: Option<String>,
    pub country: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct FeedConnectionCreate {
    pub account_token: Option<String>,
    pub account_id: Option<Uuid>,
    pub account_number: Option<String>,
    pub account_type: Option<String>,
    pub account_name: Option<String>,
    pub currency: Option<String>,
    pub country: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct FeedConnectionDeleteRequestItem {
    pub id: Option<Uuid>,
    pub account_token: Option<String>,
    pub account_type: Option<String>,
    pub country: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub page_count: Option<u32>,
    pub item_count: Option<u32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeedConnectionsResponse {
    pub pagination: Option<Pagination>,
    pub items: Option<Vec<FeedConnection>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FeedConnectionsCreateRequest {
    pub items: Vec<FeedConnectionCreate>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeedConnectionsCreateResponse {
    pub items: Option<Vec<FeedConnectionResult>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct FeedConnectionResult {
    pub account_token: Option<String>,
    pub status: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FeedConnectionsDeleteRequest {
    pub items: Vec<FeedConnectionDeleteRequestItem>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeedConnectionsDeleteResponse {
    pub items: Option<Vec<FeedConnectionResult>>,
}
