use super::query::QueryParams;
use super::AccountingApi;
use crate::error::XeroError;
use crate::models::accounting::linked_transaction;
use reqwest::Method;
use uuid::Uuid;

/// Resource accessor for Linked Transactions.
#[derive(Debug, Clone, Copy)]
pub struct LinkedTransactionsResource<'a> {
    api: &'a AccountingApi,
}

impl<'a> LinkedTransactionsResource<'a> {
    pub(crate) fn new(api: &'a AccountingApi) -> Self {
        Self { api }
    }

    /// Builds a request to list linked transactions.
    pub fn list(&self) -> LinkedTransactionsListRequest<'a> {
        LinkedTransactionsListRequest::new(self.api)
    }

    /// Retrieves a linked transaction by ID.
    pub async fn get(
        &self,
        linked_transaction_id: Uuid,
    ) -> Result<Vec<linked_transaction::LinkedTransaction>, XeroError> {
        let path = format!("/LinkedTransactions/{linked_transaction_id}");
        let resp: linked_transaction::LinkedTransactionsResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        Ok(resp.linked_transactions)
    }

    /// Creates or updates a linked transaction.
    pub async fn create_or_update(
        &self,
        linked_transaction: linked_transaction::LinkedTransaction,
    ) -> Result<Vec<linked_transaction::LinkedTransaction>, XeroError> {
        let path = if let Some(id) = linked_transaction.linked_transaction_id {
            format!("/LinkedTransactions/{id}")
        } else {
            "/LinkedTransactions".to_string()
        };
        let resp: linked_transaction::LinkedTransactionsResponse = self
            .api
            .client
            .send_request(Method::POST, &path, None, Some(linked_transaction))
            .await?;
        Ok(resp.linked_transactions)
    }

    /// Deletes a linked transaction.
    pub async fn delete(&self, linked_transaction_id: Uuid) -> Result<(), XeroError> {
        let path = format!("/LinkedTransactions/{linked_transaction_id}");
        self.api
            .client
            .send_request_empty_response(Method::DELETE, &path, None::<()>)
            .await
    }
}

/// Builder for Linked Transactions list requests.
#[derive(Debug, Clone)]
pub struct LinkedTransactionsListRequest<'a> {
    api: &'a AccountingApi,
    source_transaction_id: Option<Uuid>,
    contact_id: Option<Uuid>,
    status: Option<String>,
    target_transaction_id: Option<Uuid>,
    page: Option<u32>,
}

impl<'a> LinkedTransactionsListRequest<'a> {
    fn new(api: &'a AccountingApi) -> Self {
        Self {
            api,
            source_transaction_id: None,
            contact_id: None,
            status: None,
            target_transaction_id: None,
            page: None,
        }
    }

    /// Filter by source transaction ID.
    pub fn source_transaction_id(mut self, id: Uuid) -> Self {
        self.source_transaction_id = Some(id);
        self
    }

    /// Filter by contact ID.
    pub fn contact_id(mut self, id: Uuid) -> Self {
        self.contact_id = Some(id);
        self
    }

    /// Filter by status.
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    /// Filter by target transaction ID.
    pub fn target_transaction_id(mut self, id: Uuid) -> Self {
        self.target_transaction_id = Some(id);
        self
    }

    /// Sets the page number.
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    /// Executes the list request.
    pub async fn send(self) -> Result<Vec<linked_transaction::LinkedTransaction>, XeroError> {
        let mut query = QueryParams::default();
        query.push_opt("SourceTransactionID", self.source_transaction_id);
        query.push_opt("ContactID", self.contact_id);
        query.push_opt_string("Status", self.status);
        query.push_opt("TargetTransactionID", self.target_transaction_id);
        query.push_opt("page", self.page);

        let resp: linked_transaction::LinkedTransactionsResponse = self
            .api
            .client
            .send_request(
                Method::GET,
                "/LinkedTransactions",
                query.as_slice(),
                None::<()>,
            )
            .await?;
        Ok(resp.linked_transactions)
    }
}

impl AccountingApi {
    /// Retrieves one or many linked transactions.
    #[allow(clippy::too_many_arguments)]
    pub async fn get_linked_transactions(
        &self,
        linked_transaction_id: Option<Uuid>,
        source_transaction_id: Option<Uuid>,
        contact_id: Option<Uuid>,
        status: Option<String>,
        target_transaction_id: Option<Uuid>,
        page: Option<u32>,
    ) -> Result<Vec<linked_transaction::LinkedTransaction>, XeroError> {
        if let Some(id) = linked_transaction_id {
            self.linked_transactions().get(id).await
        } else {
            let mut request = self.linked_transactions().list();
            if let Some(id) = source_transaction_id {
                request = request.source_transaction_id(id);
            }
            if let Some(id) = contact_id {
                request = request.contact_id(id);
            }
            if let Some(status) = status {
                request = request.status(status);
            }
            if let Some(id) = target_transaction_id {
                request = request.target_transaction_id(id);
            }
            if let Some(page) = page {
                request = request.page(page);
            }
            request.send().await
        }
    }

    /// Creates or updates a linked transaction.
    pub async fn create_or_update_linked_transaction(
        &self,
        linked_transaction: linked_transaction::LinkedTransaction,
    ) -> Result<Vec<linked_transaction::LinkedTransaction>, XeroError> {
        self.linked_transactions()
            .create_or_update(linked_transaction)
            .await
    }

    /// Deletes a linked transaction.
    pub async fn delete_linked_transaction(
        &self,
        linked_transaction_id: Uuid,
    ) -> Result<(), XeroError> {
        self.linked_transactions()
            .delete(linked_transaction_id)
            .await
    }
}
