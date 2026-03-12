use super::query::QueryParams;
use super::AccountingApi;
use crate::error::XeroError;
use crate::models::accounting::bank_transfer;
use chrono::{DateTime, Utc};
use reqwest::Method;
use uuid::Uuid;

/// Resource accessor for Bank Transfers.
#[derive(Debug, Clone, Copy)]
pub struct BankTransfersResource<'a> {
    api: &'a AccountingApi,
}

impl<'a> BankTransfersResource<'a> {
    pub(crate) fn new(api: &'a AccountingApi) -> Self {
        Self { api }
    }

    /// Builds a request to list bank transfers.
    pub fn list(&self) -> BankTransfersListRequest<'a> {
        BankTransfersListRequest::new(self.api)
    }

    /// Retrieves a bank transfer by ID.
    pub async fn get(
        &self,
        bank_transfer_id: Uuid,
    ) -> Result<Vec<bank_transfer::BankTransfer>, XeroError> {
        let path = format!("/BankTransfers/{bank_transfer_id}");
        let resp: bank_transfer::BankTransfersResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        Ok(resp.bank_transfers)
    }

    /// Creates a new bank transfer.
    pub async fn create(
        &self,
        bank_transfer: bank_transfer::BankTransfer,
    ) -> Result<Vec<bank_transfer::BankTransfer>, XeroError> {
        let resp: bank_transfer::BankTransfersResponse = self
            .api
            .client
            .send_request(Method::PUT, "/BankTransfers", None, Some(bank_transfer))
            .await?;
        Ok(resp.bank_transfers)
    }
}

/// Builder for Bank Transfers list requests.
#[derive(Debug, Clone)]
pub struct BankTransfersListRequest<'a> {
    api: &'a AccountingApi,
    where_filter: Option<String>,
    order_by: Option<String>,
    page: Option<u32>,
}

impl<'a> BankTransfersListRequest<'a> {
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
    pub async fn send(self) -> Result<Vec<bank_transfer::BankTransfer>, XeroError> {
        let mut query = QueryParams::default();
        query.push_opt_string("where", self.where_filter);
        query.push_opt_string("order", self.order_by);
        query.push_opt("page", self.page);

        let resp: bank_transfer::BankTransfersResponse = self
            .api
            .client
            .send_request(Method::GET, "/BankTransfers", query.as_slice(), None::<()>)
            .await?;
        Ok(resp.bank_transfers)
    }
}

impl AccountingApi {
    /// Retrieves one or many bank transfers.
    pub async fn get_bank_transfers(
        &self,
        bank_transfer_id: Option<Uuid>,
        _modified_after: Option<DateTime<Utc>>,
        where_filter: Option<String>,
        order_by: Option<String>,
    ) -> Result<Vec<bank_transfer::BankTransfer>, XeroError> {
        if let Some(id) = bank_transfer_id {
            self.bank_transfers().get(id).await
        } else {
            let mut request = self.bank_transfers().list();
            if let Some(filter) = where_filter {
                request = request.where_filter(filter);
            }
            if let Some(order) = order_by {
                request = request.order_by(order);
            }
            request.send().await
        }
    }

    /// Creates a new bank transfer.
    pub async fn create_bank_transfer(
        &self,
        bank_transfer: bank_transfer::BankTransfer,
    ) -> Result<Vec<bank_transfer::BankTransfer>, XeroError> {
        self.bank_transfers().create(bank_transfer).await
    }
}
