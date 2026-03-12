use super::query::QueryParams;
use super::AccountingApi;
use crate::error::XeroError;
use crate::models::accounting::{account, attachment};
use chrono::{DateTime, Utc};
use reqwest::Method;
use uuid::Uuid;

/// Resource accessor for Accounts.
#[derive(Debug, Clone, Copy)]
pub struct AccountsResource<'a> {
    api: &'a AccountingApi,
}

impl<'a> AccountsResource<'a> {
    pub(crate) fn new(api: &'a AccountingApi) -> Self {
        Self { api }
    }

    /// Builds a request to list accounts.
    pub fn list(&self) -> AccountsListRequest<'a> {
        AccountsListRequest::new(self.api)
    }

    /// Retrieves a single account by ID.
    pub async fn get(&self, account_id: Uuid) -> Result<Vec<account::Account>, XeroError> {
        let path = format!("/Accounts/{account_id}");
        let resp: account::AccountsResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        Ok(resp.accounts)
    }

    /// Creates one or more new accounts.
    pub async fn create(
        &self,
        accounts: Vec<account::Account>,
    ) -> Result<Vec<account::Account>, XeroError> {
        let body = if accounts.len() == 1 {
            serde_json::to_value(accounts.into_iter().next().unwrap())?
        } else {
            serde_json::to_value(account::AccountsRequest { accounts })?
        };
        let resp: account::AccountsResponse = self
            .api
            .client
            .send_request(Method::PUT, "/Accounts", None, Some(body))
            .await?;
        Ok(resp.accounts)
    }

    /// Updates an existing account.
    pub async fn update(
        &self,
        account_id: Uuid,
        account_data: account::Account,
    ) -> Result<Vec<account::Account>, XeroError> {
        let path = format!("/Accounts/{account_id}");
        let resp: account::AccountsResponse = self
            .api
            .client
            .send_request(Method::POST, &path, None, Some(account_data))
            .await?;
        Ok(resp.accounts)
    }

    /// Deletes an account.
    pub async fn delete(&self, account_id: Uuid) -> Result<(), XeroError> {
        let path = format!("/Accounts/{account_id}");
        self.api
            .client
            .send_request_empty_response(Method::DELETE, &path, None::<()>)
            .await
    }

    /// Attaches a file to an account.
    pub async fn attach_by_file_name(
        &self,
        account_id: Uuid,
        file_name: String,
        body: Vec<u8>,
    ) -> Result<Vec<attachment::Attachment>, XeroError> {
        let encoded_file_name = file_name.replace('[', "%5B").replace(']', "%5D");
        let path = format!("/Accounts/{account_id}/Attachments/{encoded_file_name}");
        let content_type = "application/octet-stream";
        let resp: attachment::AttachmentsResponse = self
            .api
            .client
            .send_request_raw_body(Method::PUT, &path, content_type, body)
            .await?;
        Ok(resp.attachments)
    }
}

/// Builder for Accounts list requests.
#[derive(Debug, Clone)]
pub struct AccountsListRequest<'a> {
    api: &'a AccountingApi,
    where_filter: Option<String>,
    order_by: Option<String>,
}

impl<'a> AccountsListRequest<'a> {
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
    pub async fn send(self) -> Result<Vec<account::Account>, XeroError> {
        let mut query = QueryParams::default();
        query.push_opt_string("where", self.where_filter);
        query.push_opt_string("order", self.order_by);

        let resp: account::AccountsResponse = self
            .api
            .client
            .send_request(Method::GET, "/Accounts", query.as_slice(), None::<()>)
            .await?;
        Ok(resp.accounts)
    }
}

impl AccountingApi {
    /// Retrieves the full chart of accounts or a specific account by its ID.
    pub async fn get_accounts(
        &self,
        account_id: Option<Uuid>,
        _modified_after: Option<DateTime<Utc>>,
        where_filter: Option<String>,
        order_by: Option<String>,
    ) -> Result<Vec<account::Account>, XeroError> {
        if let Some(id) = account_id {
            self.accounts().get(id).await
        } else {
            let mut request = self.accounts().list();
            if let Some(filter) = where_filter {
                request = request.where_filter(filter);
            }
            if let Some(order) = order_by {
                request = request.order_by(order);
            }
            request.send().await
        }
    }

    /// Creates one or more new accounts.
    pub async fn create_accounts(
        &self,
        accounts: Vec<account::Account>,
    ) -> Result<Vec<account::Account>, XeroError> {
        self.accounts().create(accounts).await
    }

    /// Updates an existing account.
    pub async fn update_account(
        &self,
        account_id: Uuid,
        account_data: account::Account,
    ) -> Result<Vec<account::Account>, XeroError> {
        self.accounts().update(account_id, account_data).await
    }

    /// Deletes an account.
    pub async fn delete_account(&self, account_id: Uuid) -> Result<(), XeroError> {
        self.accounts().delete(account_id).await
    }

    /// Attaches a file to an account.
    pub async fn create_account_attachment_by_file_name(
        &self,
        account_id: Uuid,
        file_name: String,
        body: Vec<u8>,
    ) -> Result<Vec<attachment::Attachment>, XeroError> {
        self.accounts()
            .attach_by_file_name(account_id, file_name, body)
            .await
    }
}
