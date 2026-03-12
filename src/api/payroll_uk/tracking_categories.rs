use super::PayrollUkApi;
use crate::error::XeroError;
use crate::models::payroll_uk::tracking_category;
use reqwest::Method;

#[derive(Debug, Clone, Copy)]
pub struct TrackingCategoriesResource<'a> {
    api: &'a PayrollUkApi,
}

impl<'a> TrackingCategoriesResource<'a> {
    pub(crate) fn new(api: &'a PayrollUkApi) -> Self {
        Self { api }
    }

    pub fn list(&self) -> TrackingCategoriesListRequest<'a> {
        TrackingCategoriesListRequest::new(self.api)
    }
}

#[derive(Debug, Clone)]
pub struct TrackingCategoriesListRequest<'a> {
    api: &'a PayrollUkApi,
    page: Option<u32>,
}

impl<'a> TrackingCategoriesListRequest<'a> {
    fn new(api: &'a PayrollUkApi) -> Self {
        Self { api, page: None }
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    pub async fn send(self) -> Result<Vec<tracking_category::TrackingCategory>, XeroError> {
        let mut query = Vec::new();
        if let Some(page) = self.page {
            query.push(("page".to_string(), page.to_string()));
        }
        let resp: tracking_category::TrackingCategoryResponse = self
            .api
            .client
            .send_request(
                Method::GET,
                "/settings/trackingCategories",
                Some(&query),
                None::<()>,
            )
            .await?;
        Ok(resp.tracking_categories.unwrap_or_default())
    }
}
