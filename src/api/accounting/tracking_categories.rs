use super::query::QueryParams;
use super::AccountingApi;
use crate::error::XeroError;
use crate::models::accounting::tracking_category;
use reqwest::Method;
use serde::Serialize;
use uuid::Uuid;

/// Resource accessor for Tracking Categories.
#[derive(Debug, Clone, Copy)]
pub struct TrackingCategoriesResource<'a> {
    api: &'a AccountingApi,
}

impl<'a> TrackingCategoriesResource<'a> {
    pub(crate) fn new(api: &'a AccountingApi) -> Self {
        Self { api }
    }

    /// Builds a request to list tracking categories.
    pub fn list(&self) -> TrackingCategoriesListRequest<'a> {
        TrackingCategoriesListRequest::new(self.api)
    }

    /// Retrieves a tracking category by ID.
    pub async fn get(
        &self,
        tracking_category_id: Uuid,
    ) -> Result<Vec<tracking_category::TrackingCategory>, XeroError> {
        let path = format!("/TrackingCategories/{tracking_category_id}");
        let resp: tracking_category::TrackingCategoriesResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        Ok(resp.tracking_categories)
    }

    /// Creates a new tracking category.
    pub async fn create(
        &self,
        category: tracking_category::TrackingCategory,
    ) -> Result<Vec<tracking_category::TrackingCategory>, XeroError> {
        let resp: tracking_category::TrackingCategoriesResponse = self
            .api
            .client
            .send_request(Method::PUT, "/TrackingCategories", None, Some(category))
            .await?;
        Ok(resp.tracking_categories)
    }

    /// Updates a tracking category name.
    pub async fn update(
        &self,
        category_id: Uuid,
        name: String,
    ) -> Result<Vec<tracking_category::TrackingCategory>, XeroError> {
        #[derive(Serialize)]
        struct UpdateRequest {
            #[serde(rename = "Name")]
            name: String,
        }
        let path = format!("/TrackingCategories/{category_id}");
        let body = UpdateRequest { name };
        let resp: tracking_category::TrackingCategoriesResponse = self
            .api
            .client
            .send_request(Method::POST, &path, None, Some(body))
            .await?;
        Ok(resp.tracking_categories)
    }

    /// Deletes a tracking category.
    pub async fn delete(&self, category_id: Uuid) -> Result<(), XeroError> {
        let path = format!("/TrackingCategories/{category_id}");
        self.api
            .client
            .send_request_empty_response(Method::DELETE, &path, None::<()>)
            .await
    }

    /// Creates a new option for a tracking category.
    pub async fn create_option(
        &self,
        category_id: Uuid,
        option: tracking_category::TrackingOption,
    ) -> Result<Vec<tracking_category::TrackingOption>, XeroError> {
        let path = format!("/TrackingCategories/{category_id}/Options");
        let resp: tracking_category::TrackingOptionsResponse = self
            .api
            .client
            .send_request(Method::PUT, &path, None, Some(option))
            .await?;
        Ok(resp.options)
    }

    /// Updates a tracking option.
    pub async fn update_option(
        &self,
        category_id: Uuid,
        option_id: Uuid,
        name: String,
    ) -> Result<Vec<tracking_category::TrackingOption>, XeroError> {
        #[derive(Serialize)]
        struct UpdateRequest {
            #[serde(rename = "Name")]
            name: String,
        }
        let path = format!("/TrackingCategories/{category_id}/Options/{option_id}");
        let body = UpdateRequest { name };
        let resp: tracking_category::TrackingOptionsResponse = self
            .api
            .client
            .send_request(Method::POST, &path, None, Some(body))
            .await?;
        Ok(resp.options)
    }

    /// Deletes a tracking option.
    pub async fn delete_option(&self, category_id: Uuid, option_id: Uuid) -> Result<(), XeroError> {
        let path = format!("/TrackingCategories/{category_id}/Options/{option_id}");
        self.api
            .client
            .send_request_empty_response(Method::DELETE, &path, None::<()>)
            .await
    }
}

/// Builder for Tracking Categories list requests.
#[derive(Debug, Clone)]
pub struct TrackingCategoriesListRequest<'a> {
    api: &'a AccountingApi,
    where_filter: Option<String>,
    order_by: Option<String>,
    include_archived: Option<bool>,
}

impl<'a> TrackingCategoriesListRequest<'a> {
    fn new(api: &'a AccountingApi) -> Self {
        Self {
            api,
            where_filter: None,
            order_by: None,
            include_archived: None,
        }
    }

    /// Filter using the `where` query parameter.
    pub fn where_filter(mut self, filter: impl Into<String>) -> Self {
        self.where_filter = Some(filter.into());
        self
    }

    /// Order by a field.
    pub fn order_by(mut self, order: impl Into<String>) -> Self {
        self.order_by = Some(order.into());
        self
    }

    /// Include archived tracking categories.
    pub fn include_archived(mut self, include_archived: bool) -> Self {
        self.include_archived = Some(include_archived);
        self
    }

    /// Executes the list request.
    pub async fn send(self) -> Result<Vec<tracking_category::TrackingCategory>, XeroError> {
        let mut query = QueryParams::default();
        query.push_opt_string("where", self.where_filter);
        query.push_opt_string("order", self.order_by);
        query.push_opt("includeArchived", self.include_archived);

        let resp: tracking_category::TrackingCategoriesResponse = self
            .api
            .client
            .send_request(
                Method::GET,
                "/TrackingCategories",
                query.as_slice(),
                None::<()>,
            )
            .await?;
        Ok(resp.tracking_categories)
    }
}

impl AccountingApi {
    /// Retrieves tracking categories and their options.
    pub async fn get_tracking_categories(
        &self,
        tracking_category_id: Option<Uuid>,
        where_filter: Option<String>,
        order_by: Option<String>,
        include_archived: Option<bool>,
    ) -> Result<Vec<tracking_category::TrackingCategory>, XeroError> {
        if let Some(id) = tracking_category_id {
            self.tracking_categories().get(id).await
        } else {
            let mut request = self.tracking_categories().list();
            if let Some(filter) = where_filter {
                request = request.where_filter(filter);
            }
            if let Some(order) = order_by {
                request = request.order_by(order);
            }
            if let Some(include_archived) = include_archived {
                request = request.include_archived(include_archived);
            }
            request.send().await
        }
    }

    /// Creates a new tracking category.
    pub async fn create_tracking_category(
        &self,
        category: tracking_category::TrackingCategory,
    ) -> Result<Vec<tracking_category::TrackingCategory>, XeroError> {
        self.tracking_categories().create(category).await
    }

    /// Updates a tracking category.
    pub async fn update_tracking_category(
        &self,
        category_id: Uuid,
        name: String,
    ) -> Result<Vec<tracking_category::TrackingCategory>, XeroError> {
        self.tracking_categories().update(category_id, name).await
    }

    /// Deletes a tracking category.
    pub async fn delete_tracking_category(&self, category_id: Uuid) -> Result<(), XeroError> {
        self.tracking_categories().delete(category_id).await
    }

    /// Creates a new option for a tracking category.
    pub async fn create_tracking_option(
        &self,
        category_id: Uuid,
        option: tracking_category::TrackingOption,
    ) -> Result<Vec<tracking_category::TrackingOption>, XeroError> {
        self.tracking_categories()
            .create_option(category_id, option)
            .await
    }

    /// Updates a tracking option.
    pub async fn update_tracking_option(
        &self,
        category_id: Uuid,
        option_id: Uuid,
        name: String,
    ) -> Result<Vec<tracking_category::TrackingOption>, XeroError> {
        self.tracking_categories()
            .update_option(category_id, option_id, name)
            .await
    }

    /// Deletes a tracking option.
    pub async fn delete_tracking_option(
        &self,
        category_id: Uuid,
        option_id: Uuid,
    ) -> Result<(), XeroError> {
        self.tracking_categories()
            .delete_option(category_id, option_id)
            .await
    }
}
