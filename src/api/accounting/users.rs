use super::query::QueryParams;
use super::AccountingApi;
use crate::error::XeroError;
use crate::models::accounting::user;
use chrono::{DateTime, Utc};
use reqwest::Method;
use uuid::Uuid;

/// Resource accessor for Users.
#[derive(Debug, Clone, Copy)]
pub struct UsersResource<'a> {
    api: &'a AccountingApi,
}

impl<'a> UsersResource<'a> {
    pub(crate) fn new(api: &'a AccountingApi) -> Self {
        Self { api }
    }

    /// Builds a request to list users.
    pub fn list(&self) -> UsersListRequest<'a> {
        UsersListRequest::new(self.api)
    }

    /// Retrieves a user by ID.
    pub async fn get(&self, user_id: Uuid) -> Result<Vec<user::User>, XeroError> {
        let path = format!("/Users/{user_id}");
        let resp: user::UsersResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        Ok(resp.users)
    }
}

/// Builder for Users list requests.
#[derive(Debug, Clone)]
pub struct UsersListRequest<'a> {
    api: &'a AccountingApi,
    where_filter: Option<String>,
    order_by: Option<String>,
}

impl<'a> UsersListRequest<'a> {
    fn new(api: &'a AccountingApi) -> Self {
        Self {
            api,
            where_filter: None,
            order_by: None,
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

    /// Executes the list request.
    pub async fn send(self) -> Result<Vec<user::User>, XeroError> {
        let mut query = QueryParams::default();
        query.push_opt_string("where", self.where_filter);
        query.push_opt_string("order", self.order_by);

        let resp: user::UsersResponse = self
            .api
            .client
            .send_request(Method::GET, "/Users", query.as_slice(), None::<()>)
            .await?;
        Ok(resp.users)
    }
}

impl AccountingApi {
    /// Retrieves users for the organisation.
    pub async fn get_users(
        &self,
        user_id: Option<Uuid>,
        _modified_after: Option<DateTime<Utc>>,
        where_filter: Option<String>,
        order_by: Option<String>,
    ) -> Result<Vec<user::User>, XeroError> {
        if let Some(id) = user_id {
            self.users().get(id).await
        } else {
            let mut request = self.users().list();
            if let Some(filter) = where_filter {
                request = request.where_filter(filter);
            }
            if let Some(order) = order_by {
                request = request.order_by(order);
            }
            request.send().await
        }
    }
}
