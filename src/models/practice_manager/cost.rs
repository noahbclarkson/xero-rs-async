//! Cost (global definition) models for the XPM Practice Manager API v3.1.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Response wrappers
// ---------------------------------------------------------------------------

/// `GET cost.api/list` — paginated list of cost definitions.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "Response")]
pub struct CostsResponse {
    #[serde(rename = "Status")]
    pub status: String,
    /// Number of records returned in this page (max 1000).
    #[serde(rename = "Records")]
    pub records: Option<String>,
    #[serde(rename = "Costs")]
    pub costs: Option<CostList>,
}

/// Inner wrapper for `<Costs>`.
#[derive(Debug, Clone, Deserialize)]
pub struct CostList {
    #[serde(rename = "Cost", default)]
    pub items: Vec<Cost>,
}

/// `GET cost.api/get/[uuid]` — single cost definition.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "Response")]
pub struct CostResponse {
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Cost")]
    pub cost: Option<Cost>,
}

// ---------------------------------------------------------------------------
// Main struct
// ---------------------------------------------------------------------------

/// A global cost definition in XPM.
#[derive(Debug, Clone, Deserialize)]
pub struct Cost {
    #[serde(rename = "UUID")]
    pub uuid: Uuid,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "Code")]
    pub code: Option<String>,
    #[serde(rename = "Note")]
    pub note: Option<String>,
    #[serde(rename = "UnitCost")]
    pub unit_cost: Option<String>,
    #[serde(rename = "UnitPrice")]
    pub unit_price: Option<String>,
    #[serde(rename = "IncomeAccount")]
    pub income_account: Option<String>,
    #[serde(rename = "CostOfSaleAccount")]
    pub cost_of_sale_account: Option<String>,
}

// ---------------------------------------------------------------------------
// Request types
// ---------------------------------------------------------------------------

/// Request body for `POST cost.api/add`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "Cost")]
pub struct AddCostRequest {
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "Note", skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(rename = "UnitCost")]
    pub unit_cost: String,
    #[serde(rename = "UnitPrice")]
    pub unit_price: String,
}

/// Request body for `PUT cost.api/update`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "Cost")]
pub struct UpdateCostRequest {
    #[serde(rename = "UUID")]
    pub uuid: Uuid,
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "Note", skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(rename = "UnitCost", skip_serializing_if = "Option::is_none")]
    pub unit_cost: Option<String>,
    #[serde(rename = "UnitPrice", skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<String>,
}

/// Request body for `POST cost.api/delete`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "Cost")]
pub struct DeleteCostRequest {
    #[serde(rename = "UUID")]
    pub uuid: Uuid,
}
