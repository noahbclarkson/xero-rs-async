//! API resource for XPM Client Groups (Practice Manager v3.1).

use super::PracticeManagerApi;
use crate::error::XeroError;
use crate::models::practice_manager::client_group::{GroupResponse, GroupsResponse};
use reqwest::Method;
use uuid::Uuid;

/// Resource accessor for Practice Manager Client Groups.
#[derive(Debug, Clone, Copy)]
pub struct ClientGroupsResource<'a> {
    api: &'a PracticeManagerApi,
}

impl<'a> ClientGroupsResource<'a> {
    pub(crate) fn new(api: &'a PracticeManagerApi) -> Self {
        Self { api }
    }

    /// Return a list of all client groups.
    pub async fn list(&self) -> Result<GroupsResponse, XeroError> {
        self.api
            .client
            .send_request_xml(Method::GET, "/clientgroup.api/list", None)
            .await
    }

    /// Retrieve detailed information for a specific client group.
    pub async fn get(&self, uuid: Uuid) -> Result<GroupResponse, XeroError> {
        let path = format!("/clientgroup.api/get/{uuid}");
        self.api
            .client
            .send_request_xml(Method::GET, &path, None)
            .await
    }

    /// Add a new client group.
    ///
    /// The caller supplies the full `<Group>` XML body.
    pub async fn add(&self, xml_body: &str) -> Result<GroupResponse, XeroError> {
        self.api
            .client
            .send_request_xml_with_body(Method::POST, "/clientgroup.api/add", xml_body)
            .await
    }

    /// Manage the members of a client group (add/remove clients).
    ///
    /// The caller supplies the `<Group>` XML body containing `<add>` and/or
    /// `<remove>` elements.
    pub async fn update_members(&self, xml_body: &str) -> Result<GroupResponse, XeroError> {
        self.api
            .client
            .send_request_xml_with_body(Method::PUT, "/clientgroup.api/members", xml_body)
            .await
    }

    /// Delete a client group by UUID.
    pub async fn delete(&self, uuid: Uuid) -> Result<(), XeroError> {
        let body = format!("<Group><UUID>{uuid}</UUID></Group>");
        self.api
            .client
            .send_request_xml_empty_response(Method::POST, "/clientgroup.api/delete", Some(&body))
            .await
    }
}
