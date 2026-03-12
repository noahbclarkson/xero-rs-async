//! Custom Fields resource for the XPM Practice Manager API v3.1.

use super::PracticeManagerApi;
use crate::error::XeroError;
use crate::models::practice_manager::custom_field::{
    CustomFieldDefinitionResponse, CustomFieldDefinitionsResponse,
};
use reqwest::Method;
use uuid::Uuid;

/// Resource accessor for XPM Custom Field definitions.
#[derive(Debug, Clone, Copy)]
pub struct CustomFieldsResource<'a> {
    api: &'a PracticeManagerApi,
}

impl<'a> CustomFieldsResource<'a> {
    pub(crate) fn new(api: &'a PracticeManagerApi) -> Self {
        Self { api }
    }

    /// Retrieves detailed information for a specific custom field definition.
    pub async fn get(&self, uuid: Uuid) -> Result<CustomFieldDefinitionResponse, XeroError> {
        let path = format!("/customfield.api/get/{uuid}");
        self.api
            .client
            .send_request_xml(Method::GET, &path, None)
            .await
    }

    /// Returns a list of all custom field definitions.
    pub async fn list_definitions(&self) -> Result<CustomFieldDefinitionsResponse, XeroError> {
        self.api
            .client
            .send_request_xml(Method::GET, "/customfield.api/definition", None)
            .await
    }

    /// Adds a new custom field definition.
    pub async fn add(&self, xml_body: &str) -> Result<CustomFieldDefinitionResponse, XeroError> {
        self.api
            .client
            .send_request_xml_with_body(Method::POST, "/customfield.api/add", xml_body)
            .await
    }

    /// Updates an existing custom field definition.
    pub async fn update(&self, xml_body: &str) -> Result<CustomFieldDefinitionResponse, XeroError> {
        self.api
            .client
            .send_request_xml_with_body(Method::PUT, "/customfield.api/update", xml_body)
            .await
    }

    /// Deletes a custom field definition by UUID.
    pub async fn delete(&self, uuid: Uuid) -> Result<(), XeroError> {
        let xml_body =
            format!("<CustomFieldDefinition><UUID>{uuid}</UUID></CustomFieldDefinition>");
        self.api
            .client
            .send_request_xml_empty_response(
                Method::POST,
                "/customfield.api/delete",
                Some(&xml_body),
            )
            .await
    }
}
