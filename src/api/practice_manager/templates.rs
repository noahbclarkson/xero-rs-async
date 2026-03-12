//! Templates resource for the XPM Practice Manager API v3.1.

use super::PracticeManagerApi;
use crate::error::XeroError;
use crate::models::practice_manager::template::TemplatesResponse;
use reqwest::Method;

/// Resource accessor for XPM Templates.
#[derive(Debug, Clone, Copy)]
pub struct TemplatesResource<'a> {
    api: &'a PracticeManagerApi,
}

impl<'a> TemplatesResource<'a> {
    pub(crate) fn new(api: &'a PracticeManagerApi) -> Self {
        Self { api }
    }

    /// Returns a list of all job templates.
    pub async fn list(&self) -> Result<TemplatesResponse, XeroError> {
        self.api
            .client
            .send_request_xml(Method::GET, "/template.api/list", None)
            .await
    }
}
