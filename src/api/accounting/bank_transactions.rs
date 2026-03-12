use super::query::QueryParams;
use super::AccountingApi;
use crate::error::XeroError;
use crate::models::accounting::bank_transaction;
use chrono::{DateTime, Utc};
use reqwest::Method;
use uuid::Uuid;

/// Resource accessor for Bank Transactions.
#[derive(Debug, Clone, Copy)]
pub struct BankTransactionsResource<'a> {
    api: &'a AccountingApi,
}

impl<'a> BankTransactionsResource<'a> {
    pub(crate) fn new(api: &'a AccountingApi) -> Self {
        Self { api }
    }

    /// Builds a request to list bank transactions.
    pub fn list(&self) -> BankTransactionsListRequest<'a> {
        BankTransactionsListRequest::new(self.api)
    }

    /// Retrieves a bank transaction by ID.
    pub async fn get(
        &self,
        bank_transaction_id: Uuid,
    ) -> Result<Vec<bank_transaction::BankTransaction>, XeroError> {
        let path = format!("/BankTransactions/{bank_transaction_id}");
        let resp: bank_transaction::BankTransactionsResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        Ok(resp.bank_transactions)
    }

    /// Creates one or more new spend or receive money transactions.
    pub async fn create(
        &self,
        transactions: Vec<bank_transaction::BankTransaction>,
        summarize_errors: Option<bool>,
    ) -> Result<Vec<bank_transaction::BankTransaction>, XeroError> {
        let mut query = QueryParams::default();
        query.push_opt("summarizeErrors", summarize_errors);

        let body = if transactions.len() == 1 {
            serde_json::to_value(transactions.into_iter().next().unwrap())?
        } else {
            serde_json::to_value(bank_transaction::BankTransactionsRequest {
                bank_transactions: transactions,
            })?
        };
        let resp: bank_transaction::BankTransactionsResponse = self
            .api
            .client
            .send_request(
                Method::PUT,
                "/BankTransactions",
                query.as_slice(),
                Some(body),
            )
            .await?;
        Ok(resp.bank_transactions)
    }

    /// Updates an existing spend or receive money transaction.
    pub async fn update(
        &self,
        bank_transaction_id: Uuid,
        transaction_data: bank_transaction::BankTransaction,
    ) -> Result<Vec<bank_transaction::BankTransaction>, XeroError> {
        let path = format!("/BankTransactions/{bank_transaction_id}");
        let resp: bank_transaction::BankTransactionsResponse = self
            .api
            .client
            .send_request(Method::POST, &path, None, Some(transaction_data))
            .await?;
        Ok(resp.bank_transactions)
    }
}

/// Builder for Bank Transactions list requests.
#[derive(Debug, Clone)]
pub struct BankTransactionsListRequest<'a> {
    api: &'a AccountingApi,
    where_filter: Option<String>,
    order_by: Option<String>,
    page: Option<u32>,
    page_size: Option<u32>,
}

impl<'a> BankTransactionsListRequest<'a> {
    fn new(api: &'a AccountingApi) -> Self {
        Self {
            api,
            where_filter: None,
            order_by: None,
            page: None,
            page_size: None,
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

    /// Sets the page size.
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// Executes the list request.
    pub async fn send(self) -> Result<Vec<bank_transaction::BankTransaction>, XeroError> {
        let mut query = QueryParams::default();
        query.push_opt_string("where", self.where_filter);
        query.push_opt_string("order", self.order_by);
        query.push_opt("page", self.page);
        query.push_opt("pageSize", self.page_size);

        let resp: bank_transaction::BankTransactionsResponse = self
            .api
            .client
            .send_request(
                Method::GET,
                "/BankTransactions",
                query.as_slice(),
                None::<()>,
            )
            .await?;
        Ok(resp.bank_transactions)
    }
}

impl AccountingApi {
    /// Retrieves one or many bank transactions.
    #[allow(clippy::too_many_arguments)]
    pub async fn get_bank_transactions(
        &self,
        bank_transaction_id: Option<Uuid>,
        _modified_after: Option<DateTime<Utc>>,
        where_filter: Option<String>,
        order_by: Option<String>,
        page: Option<u32>,
        page_size: Option<u32>,
    ) -> Result<Vec<bank_transaction::BankTransaction>, XeroError> {
        if let Some(id) = bank_transaction_id {
            self.bank_transactions().get(id).await
        } else {
            let mut request = self.bank_transactions().list();
            if let Some(filter) = where_filter {
                request = request.where_filter(filter);
            }
            if let Some(order) = order_by {
                request = request.order_by(order);
            }
            if let Some(p) = page {
                request = request.page(p);
            }
            if let Some(ps) = page_size {
                request = request.page_size(ps);
            }
            request.send().await
        }
    }

    /// Creates one or more new spend or receive money transactions.
    pub async fn create_bank_transactions(
        &self,
        transactions: Vec<bank_transaction::BankTransaction>,
        summarize_errors: Option<bool>,
    ) -> Result<Vec<bank_transaction::BankTransaction>, XeroError> {
        self.bank_transactions()
            .create(transactions, summarize_errors)
            .await
    }

    /// Updates an existing spend or receive money transaction.
    pub async fn update_bank_transaction(
        &self,
        bank_transaction_id: Uuid,
        transaction_data: bank_transaction::BankTransaction,
    ) -> Result<Vec<bank_transaction::BankTransaction>, XeroError> {
        self.bank_transactions()
            .update(bank_transaction_id, transaction_data)
            .await
    }
}
