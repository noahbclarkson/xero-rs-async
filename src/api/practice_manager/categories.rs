//! Categories resource for the XPM Practice Manager API v3.1.

use super::PracticeManagerApi;
use crate::error::XeroError;
use crate::models::practice_manager::category::CategoriesResponse;
use reqwest::Method;

/// Resource accessor for XPM Categories.
#[derive(Debug, Clone, Copy)]
pub struct CategoriesResource<'a> {
    api: &'a PracticeManagerApi,
}

impl<'a> CategoriesResource<'a> {
    pub(crate) fn new(api: &'a PracticeManagerApi) -> Self {
        Self { api }
    }

    /// Returns a list of all categories.
    pub async fn list(&self) -> Result<CategoriesResponse, XeroError> {
        self.api
            .client
            .send_request_xml(Method::GET, "/category.api/list", None)
            .await
    }
}
