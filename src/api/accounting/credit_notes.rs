use super::query::QueryParams;
use super::AccountingApi;
use crate::error::XeroError;
use crate::models::accounting::common::Allocation;
use crate::models::accounting::credit_note;
use chrono::{DateTime, Utc};
use reqwest::Method;
use uuid::Uuid;

/// Resource accessor for Credit Notes.
#[derive(Debug, Clone, Copy)]
pub struct CreditNotesResource<'a> {
    api: &'a AccountingApi,
}

impl<'a> CreditNotesResource<'a> {
    pub(crate) fn new(api: &'a AccountingApi) -> Self {
        Self { api }
    }

    /// Builds a request to list credit notes.
    pub fn list(&self) -> CreditNotesListRequest<'a> {
        CreditNotesListRequest::new(self.api)
    }

    /// Retrieves a single credit note by ID.
    pub async fn get(
        &self,
        credit_note_id: Uuid,
    ) -> Result<Vec<credit_note::CreditNote>, XeroError> {
        let path = format!("/CreditNotes/{credit_note_id}");
        let resp: credit_note::CreditNotesResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        Ok(resp.credit_notes)
    }

    /// Creates one or more new credit notes.
    pub async fn create(
        &self,
        credit_notes: Vec<credit_note::CreditNote>,
        summarize_errors: Option<bool>,
    ) -> Result<Vec<credit_note::CreditNote>, XeroError> {
        let mut query = QueryParams::default();
        query.push_opt("summarizeErrors", summarize_errors);
        let body = if credit_notes.len() == 1 {
            serde_json::to_value(credit_notes.into_iter().next().unwrap())?
        } else {
            serde_json::to_value(credit_note::CreditNotesRequest { credit_notes })?
        };
        let resp: credit_note::CreditNotesResponse = self
            .api
            .client
            .send_request(Method::PUT, "/CreditNotes", query.as_slice(), Some(body))
            .await?;
        Ok(resp.credit_notes)
    }

    /// Updates an existing credit note.
    pub async fn update(
        &self,
        credit_note_id: Uuid,
        credit_note_data: credit_note::CreditNote,
    ) -> Result<Vec<credit_note::CreditNote>, XeroError> {
        let path = format!("/CreditNotes/{credit_note_id}");
        let resp: credit_note::CreditNotesResponse = self
            .api
            .client
            .send_request(Method::POST, &path, None, Some(credit_note_data))
            .await?;
        Ok(resp.credit_notes)
    }

    /// Allocates a credit note.
    pub async fn allocate(
        &self,
        credit_note_id: Uuid,
        allocations: Vec<Allocation>,
    ) -> Result<Vec<Allocation>, XeroError> {
        let path = format!("/CreditNotes/{credit_note_id}/Allocations");
        let body = serde_json::json!({ "Allocations": allocations });
        let resp: credit_note::AllocationsResponse = self
            .api
            .client
            .send_request(Method::PUT, &path, None, Some(body))
            .await?;
        Ok(resp.allocations)
    }

    /// Deletes a credit note allocation.
    pub async fn delete_allocation(
        &self,
        credit_note_id: Uuid,
        allocation_id: Uuid,
    ) -> Result<(), XeroError> {
        let path = format!("/CreditNotes/{credit_note_id}/Allocations/{allocation_id}");
        self.api
            .client
            .send_request_empty_response(Method::DELETE, &path, None::<()>)
            .await
    }
}

/// Builder for Credit Notes list requests.
#[derive(Debug, Clone)]
pub struct CreditNotesListRequest<'a> {
    api: &'a AccountingApi,
    where_filter: Option<String>,
    order_by: Option<String>,
    page: Option<u32>,
    page_size: Option<u32>,
}

impl<'a> CreditNotesListRequest<'a> {
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
    pub async fn send(self) -> Result<Vec<credit_note::CreditNote>, XeroError> {
        let mut query = QueryParams::default();
        query.push_opt_string("where", self.where_filter);
        query.push_opt_string("order", self.order_by);
        query.push_opt("page", self.page);
        query.push_opt("pageSize", self.page_size);

        let resp: credit_note::CreditNotesResponse = self
            .api
            .client
            .send_request(Method::GET, "/CreditNotes", query.as_slice(), None::<()>)
            .await?;
        Ok(resp.credit_notes)
    }
}

impl AccountingApi {
    /// Retrieves one or many credit notes.
    #[allow(clippy::too_many_arguments)]
    pub async fn get_credit_notes(
        &self,
        credit_note_id: Option<Uuid>,
        _modified_after: Option<DateTime<Utc>>,
        where_filter: Option<String>,
        order_by: Option<String>,
        page: Option<u32>,
        page_size: Option<u32>,
    ) -> Result<Vec<credit_note::CreditNote>, XeroError> {
        if let Some(id) = credit_note_id {
            self.credit_notes().get(id).await
        } else {
            let mut request = self.credit_notes().list();
            if let Some(filter) = where_filter {
                request = request.where_filter(filter);
            }
            if let Some(order) = order_by {
                request = request.order_by(order);
            }
            if let Some(page) = page {
                request = request.page(page);
            }
            if let Some(page_size) = page_size {
                request = request.page_size(page_size);
            }
            request.send().await
        }
    }

    /// Creates one or more new credit notes.
    pub async fn create_credit_notes(
        &self,
        credit_notes: Vec<credit_note::CreditNote>,
        summarize_errors: Option<bool>,
    ) -> Result<Vec<credit_note::CreditNote>, XeroError> {
        self.credit_notes()
            .create(credit_notes, summarize_errors)
            .await
    }

    /// Updates an existing credit note.
    pub async fn update_credit_note(
        &self,
        credit_note_id: Uuid,
        credit_note_data: credit_note::CreditNote,
    ) -> Result<Vec<credit_note::CreditNote>, XeroError> {
        self.credit_notes()
            .update(credit_note_id, credit_note_data)
            .await
    }

    /// Allocates a credit note.
    pub async fn allocate_credit_note(
        &self,
        credit_note_id: Uuid,
        allocations: Vec<Allocation>,
    ) -> Result<Vec<Allocation>, XeroError> {
        self.credit_notes()
            .allocate(credit_note_id, allocations)
            .await
    }

    /// Deletes a credit note allocation.
    pub async fn delete_credit_note_allocation(
        &self,
        credit_note_id: Uuid,
        allocation_id: Uuid,
    ) -> Result<(), XeroError> {
        self.credit_notes()
            .delete_allocation(credit_note_id, allocation_id)
            .await
    }
}
