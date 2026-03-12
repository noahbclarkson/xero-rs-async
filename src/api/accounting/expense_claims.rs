use super::query::QueryParams;
use super::AccountingApi;
use crate::error::XeroError;
use crate::models::accounting::expense_claim;
use chrono::{DateTime, Utc};
use reqwest::Method;
use uuid::Uuid;

/// Resource accessor for Expense Claims.
#[derive(Debug, Clone, Copy)]
pub struct ExpenseClaimsResource<'a> {
    api: &'a AccountingApi,
}

impl<'a> ExpenseClaimsResource<'a> {
    pub(crate) fn new(api: &'a AccountingApi) -> Self {
        Self { api }
    }

    /// Builds a request to list expense claims.
    pub fn list(&self) -> ExpenseClaimsListRequest<'a> {
        ExpenseClaimsListRequest::new(self.api)
    }

    /// Retrieves a single expense claim by ID.
    pub async fn get(
        &self,
        expense_claim_id: Uuid,
    ) -> Result<Vec<expense_claim::ExpenseClaim>, XeroError> {
        let path = format!("/ExpenseClaims/{expense_claim_id}");
        let resp: expense_claim::ExpenseClaimsResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        Ok(resp.expense_claims)
    }

    /// Creates or updates one or many expense claims.
    pub async fn create_or_update(
        &self,
        claims: Vec<expense_claim::ExpenseClaim>,
        summarize_errors: Option<bool>,
    ) -> Result<Vec<expense_claim::ExpenseClaim>, XeroError> {
        let mut query = QueryParams::default();
        query.push_opt("summarizeErrors", summarize_errors);

        let body = if claims.len() == 1 {
            serde_json::to_value(claims.into_iter().next().unwrap())?
        } else {
            serde_json::to_value(expense_claim::ExpenseClaimsRequest {
                expense_claims: claims,
            })?
        };
        let resp: expense_claim::ExpenseClaimsResponse = self
            .api
            .client
            .send_request(Method::POST, "/ExpenseClaims", query.as_slice(), Some(body))
            .await?;
        Ok(resp.expense_claims)
    }
}

/// Builder for Expense Claims list requests.
#[derive(Debug, Clone)]
pub struct ExpenseClaimsListRequest<'a> {
    api: &'a AccountingApi,
    where_filter: Option<String>,
    order_by: Option<String>,
}

impl<'a> ExpenseClaimsListRequest<'a> {
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
    pub async fn send(self) -> Result<Vec<expense_claim::ExpenseClaim>, XeroError> {
        let mut query = QueryParams::default();
        query.push_opt_string("where", self.where_filter);
        query.push_opt_string("order", self.order_by);

        let resp: expense_claim::ExpenseClaimsResponse = self
            .api
            .client
            .send_request(Method::GET, "/ExpenseClaims", query.as_slice(), None::<()>)
            .await?;
        Ok(resp.expense_claims)
    }
}

impl AccountingApi {
    /// Retrieves one or many expense claims.
    pub async fn get_expense_claims(
        &self,
        expense_claim_id: Option<Uuid>,
        _modified_after: Option<DateTime<Utc>>,
        where_filter: Option<String>,
        order_by: Option<String>,
    ) -> Result<Vec<expense_claim::ExpenseClaim>, XeroError> {
        if let Some(id) = expense_claim_id {
            self.expense_claims().get(id).await
        } else {
            let mut request = self.expense_claims().list();
            if let Some(filter) = where_filter {
                request = request.where_filter(filter);
            }
            if let Some(order) = order_by {
                request = request.order_by(order);
            }
            request.send().await
        }
    }

    /// Creates or updates one or many expense claims.
    pub async fn create_or_update_expense_claims(
        &self,
        claims: Vec<expense_claim::ExpenseClaim>,
        summarize_errors: Option<bool>,
    ) -> Result<Vec<expense_claim::ExpenseClaim>, XeroError> {
        self.expense_claims()
            .create_or_update(claims, summarize_errors)
            .await
    }
}
