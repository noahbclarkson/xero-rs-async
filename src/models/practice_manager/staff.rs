//! Staff models for the XPM Practice Manager API v3.1.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Response wrappers
// ---------------------------------------------------------------------------

/// `GET staff.api/list` — list of all staff.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "Response")]
pub struct StaffListResponse {
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "StaffList")]
    pub staff_list: Option<StaffListInner>,
}

/// Inner wrapper for `<StaffList>` containing many `<Staff>` elements.
#[derive(Debug, Clone, Deserialize)]
pub struct StaffListInner {
    #[serde(rename = "Staff", default)]
    pub items: Vec<Staff>,
}

/// `GET staff.api/get/[uuid]` — single staff detail.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "Response")]
pub struct StaffResponse {
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "WebUrl")]
    pub web_url: Option<String>,
    #[serde(rename = "Staff")]
    pub staff: Option<Staff>,
}

// ---------------------------------------------------------------------------
// Main struct
// ---------------------------------------------------------------------------

/// A staff member in XPM.
#[derive(Debug, Clone, Deserialize)]
pub struct Staff {
    #[serde(rename = "UUID")]
    pub uuid: Uuid,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Email")]
    pub email: Option<String>,
    #[serde(rename = "Phone")]
    pub phone: Option<String>,
    #[serde(rename = "Mobile")]
    pub mobile: Option<String>,
    #[serde(rename = "Address")]
    pub address: Option<String>,
    #[serde(rename = "PayrollCode")]
    pub payroll_code: Option<String>,
}

// ---------------------------------------------------------------------------
// Request types
// ---------------------------------------------------------------------------

/// Request body for `POST staff.api/add`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "Staff")]
pub struct AddStaffRequest {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "Phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(rename = "Mobile", skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[serde(rename = "Email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "PayrollCode", skip_serializing_if = "Option::is_none")]
    pub payroll_code: Option<String>,
}

/// Request body for `PUT staff.api/update`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "Staff")]
pub struct UpdateStaffRequest {
    #[serde(rename = "UUID")]
    pub uuid: Uuid,
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "Phone", skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(rename = "Mobile", skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    #[serde(rename = "Email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "PayrollCode", skip_serializing_if = "Option::is_none")]
    pub payroll_code: Option<String>,
}

/// Request body for `POST staff.api/delete`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "Staff")]
pub struct DeleteStaffRequest {
    #[serde(rename = "UUID")]
    pub uuid: Uuid,
}

/// Request body for `POST staff.api/enable`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "Staff")]
pub struct EnableStaffRequest {
    #[serde(rename = "UUID")]
    pub uuid: Uuid,
}

/// Request body for `POST staff.api/disable`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "Staff")]
pub struct DisableStaffRequest {
    #[serde(rename = "UUID")]
    pub uuid: Uuid,
}

/// Request body for `POST staff.api/forgottenpassword`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename = "Staff")]
pub struct ForgottenPasswordRequest {
    #[serde(rename = "UUID")]
    pub uuid: Uuid,
}
