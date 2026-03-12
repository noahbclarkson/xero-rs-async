//! Costs resource for the XPM Practice Manager API v3.1.
//!
//! These are global cost definitions (not job-specific costs).

use super::PracticeManagerApi;
use crate::error::XeroError;
use crate::models::practice_manager::cost::{CostResponse, CostsResponse};
use reqwest::Method;
use uuid::Uuid;

/// Resource accessor for XPM Cost definitions.
#[derive(Debug, Clone, Copy)]
pub struct CostsResource<'a> {
    api: &'a PracticeManagerApi,
}

impl<'a> CostsResource<'a> {
    pub(crate) fn new(api: &'a PracticeManagerApi) -> Self {
        Self { api }
    }

    /// Returns a paginated list of all cost definitions.
    ///
    /// A maximum of 1000 items are returned per page. Continue incrementing
    /// `page` until the `Records` element is `0`.
    pub async fn list(&self, page: u32) -> Result<CostsResponse, XeroError> {
        let query = vec![("page".into(), page.to_string())];
        self.api
            .client
            .send_request_xml(Method::GET, "/cost.api/list", Some(query.as_slice()))
            .await
    }

    /// Retrieves detailed information for a specific cost definition.
    pub async fn get(&self, uuid: Uuid) -> Result<CostResponse, XeroError> {
        let path = format!("/cost.api/get/{uuid}");
        self.api
            .client
            .send_request_xml(Method::GET, &path, None)
            .await
    }

    /// Adds a new cost definition.
    pub async fn add(&self, xml_body: &str) -> Result<CostResponse, XeroError> {
        self.api
            .client
            .send_request_xml_with_body(Method::POST, "/cost.api/add", xml_body)
            .await
    }

    /// Updates an existing cost definition.
    pub async fn update(&self, xml_body: &str) -> Result<CostResponse, XeroError> {
        self.api
            .client
            .send_request_xml_with_body(Method::PUT, "/cost.api/update", xml_body)
            .await
    }

    /// Deletes a cost definition by UUID.
    pub async fn delete(&self, uuid: Uuid) -> Result<(), XeroError> {
        let xml_body = format!("<Cost><UUID>{uuid}</UUID></Cost>");
        self.api
            .client
            .send_request_xml_empty_response(Method::POST, "/cost.api/delete", Some(&xml_body))
            .await
    }

    /// Deletes all cost definitions.
    pub async fn delete_all(&self) -> Result<(), XeroError> {
        self.api
            .client
            .send_request_xml_empty_response(
                Method::POST,
                "/cost.api/deleteall",
                Some("<DeleteAll />"),
            )
            .await
    }
}
