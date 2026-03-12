//! API resource for XPM Staff (Practice Manager v3.1).

use super::PracticeManagerApi;
use crate::error::XeroError;
use crate::models::practice_manager::staff::{StaffListResponse, StaffResponse};
use reqwest::Method;
use uuid::Uuid;

/// Resource accessor for Practice Manager Staff.
#[derive(Debug, Clone, Copy)]
pub struct StaffResource<'a> {
    api: &'a PracticeManagerApi,
}

impl<'a> StaffResource<'a> {
    pub(crate) fn new(api: &'a PracticeManagerApi) -> Self {
        Self { api }
    }

    /// Return a list of all staff members.
    pub async fn list(&self) -> Result<StaffListResponse, XeroError> {
        self.api
            .client
            .send_request_xml(Method::GET, "/staff.api/list", None)
            .await
    }

    /// Retrieve details for a specific staff member.
    pub async fn get(&self, uuid: Uuid) -> Result<StaffResponse, XeroError> {
        let path = format!("/staff.api/get/{uuid}");
        self.api
            .client
            .send_request_xml(Method::GET, &path, None)
            .await
    }

    /// Add a new staff member.
    ///
    /// The caller supplies the full `<Staff>` XML body.
    pub async fn add(&self, xml_body: &str) -> Result<StaffResponse, XeroError> {
        self.api
            .client
            .send_request_xml_with_body(Method::POST, "/staff.api/add", xml_body)
            .await
    }

    /// Update a staff member's details.
    ///
    /// The caller supplies the full `<Staff>` XML body (must include `<UUID>`).
    pub async fn update(&self, xml_body: &str) -> Result<StaffResponse, XeroError> {
        self.api
            .client
            .send_request_xml_with_body(Method::PUT, "/staff.api/update", xml_body)
            .await
    }

    /// Delete a staff member by UUID.
    pub async fn delete(&self, uuid: Uuid) -> Result<(), XeroError> {
        let body = format!("<Staff><UUID>{uuid}</UUID></Staff>");
        self.api
            .client
            .send_request_xml_empty_response(Method::POST, "/staff.api/delete", Some(&body))
            .await
    }

    /// Enable a staff member so they can log into Practice Manager.
    pub async fn enable(&self, uuid: Uuid) -> Result<(), XeroError> {
        let body = format!("<Staff><UUID>{uuid}</UUID><Security></Security></Staff>");
        self.api
            .client
            .send_request_xml_empty_response(Method::POST, "/staff.api/enable", Some(&body))
            .await
    }

    /// Disable a staff member so they can no longer log into Practice Manager.
    pub async fn disable(&self, uuid: Uuid) -> Result<(), XeroError> {
        let body = format!("<Staff><UUID>{uuid}</UUID></Staff>");
        self.api
            .client
            .send_request_xml_empty_response(Method::POST, "/staff.api/disable", Some(&body))
            .await
    }

    /// Reset a staff member's password. They will receive an email to reset it.
    pub async fn forgotten_password(&self, uuid: Uuid) -> Result<(), XeroError> {
        let body = format!("<Staff><UUID>{uuid}</UUID></Staff>");
        self.api
            .client
            .send_request_xml_empty_response(
                Method::POST,
                "/staff.api/forgottenpassword",
                Some(&body),
            )
            .await
    }
}
