use super::query::QueryParams;
use super::AccountingApi;
use crate::error::XeroError;
use crate::models::accounting::item;
use chrono::{DateTime, Utc};
use reqwest::Method;
use uuid::Uuid;

/// Resource accessor for Items.
#[derive(Debug, Clone, Copy)]
pub struct ItemsResource<'a> {
    api: &'a AccountingApi,
}

impl<'a> ItemsResource<'a> {
    pub(crate) fn new(api: &'a AccountingApi) -> Self {
        Self { api }
    }

    /// Builds a request to list items.
    pub fn list(&self) -> ItemsListRequest<'a> {
        ItemsListRequest::new(self.api)
    }

    /// Retrieves a single item by ID.
    pub async fn get(&self, item_id: Uuid) -> Result<Vec<item::Item>, XeroError> {
        let path = format!("/Items/{item_id}");
        let resp: item::ItemsResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        Ok(resp.items)
    }

    /// Creates one or more new items.
    pub async fn create(&self, items: Vec<item::Item>) -> Result<Vec<item::Item>, XeroError> {
        let body = if items.len() == 1 {
            serde_json::to_value(items.into_iter().next().unwrap())?
        } else {
            serde_json::to_value(item::ItemsRequest { items })?
        };
        let resp: item::ItemsResponse = self
            .api
            .client
            .send_request(Method::PUT, "/Items", None, Some(body))
            .await?;
        Ok(resp.items)
    }

    /// Updates an existing item.
    pub async fn update(
        &self,
        item_id: Uuid,
        item_data: item::Item,
    ) -> Result<Vec<item::Item>, XeroError> {
        let path = format!("/Items/{item_id}");
        let resp: item::ItemsResponse = self
            .api
            .client
            .send_request(Method::POST, &path, None, Some(item_data))
            .await?;
        Ok(resp.items)
    }

    /// Deletes an item.
    pub async fn delete(&self, item_id: Uuid) -> Result<(), XeroError> {
        let path = format!("/Items/{item_id}");
        self.api
            .client
            .send_request_empty_response(Method::DELETE, &path, None::<()>)
            .await
    }
}

/// Builder for Items list requests.
#[derive(Debug, Clone)]
pub struct ItemsListRequest<'a> {
    api: &'a AccountingApi,
    where_filter: Option<String>,
    order_by: Option<String>,
    page: Option<u32>,
}

impl<'a> ItemsListRequest<'a> {
    fn new(api: &'a AccountingApi) -> Self {
        Self {
            api,
            where_filter: None,
            order_by: None,
            page: None,
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

    /// Sets the page number.
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    /// Executes the list request.
    pub async fn send(self) -> Result<Vec<item::Item>, XeroError> {
        let mut query = QueryParams::default();
        query.push_opt_string("where", self.where_filter);
        query.push_opt_string("order", self.order_by);
        query.push_opt("page", self.page);

        let resp: item::ItemsResponse = self
            .api
            .client
            .send_request(Method::GET, "/Items", query.as_slice(), None::<()>)
            .await?;
        Ok(resp.items)
    }
}

impl AccountingApi {
    /// Retrieves one or many items.
    pub async fn get_items(
        &self,
        item_id: Option<Uuid>,
        _modified_after: Option<DateTime<Utc>>,
        where_filter: Option<String>,
        order_by: Option<String>,
    ) -> Result<Vec<item::Item>, XeroError> {
        if let Some(id) = item_id {
            self.items().get(id).await
        } else {
            let mut request = self.items().list();
            if let Some(filter) = where_filter {
                request = request.where_filter(filter);
            }
            if let Some(order) = order_by {
                request = request.order_by(order);
            }
            request.send().await
        }
    }

    /// Creates one or more new items.
    pub async fn create_items(&self, items: Vec<item::Item>) -> Result<Vec<item::Item>, XeroError> {
        self.items().create(items).await
    }

    /// Updates an existing item.
    pub async fn update_item(
        &self,
        item_id: Uuid,
        item_data: item::Item,
    ) -> Result<Vec<item::Item>, XeroError> {
        self.items().update(item_id, item_data).await
    }

    /// Deletes an item.
    pub async fn delete_item(&self, item_id: Uuid) -> Result<(), XeroError> {
        self.items().delete(item_id).await
    }
}
