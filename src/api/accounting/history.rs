use super::AccountingApi;
use crate::error::XeroError;
use crate::models::accounting::history;
use reqwest::Method;
use serde::Serialize;
use uuid::Uuid;

/// Resource accessor for History and Notes.
#[derive(Debug, Clone, Copy)]
pub struct HistoryResource<'a> {
    api: &'a AccountingApi,
}

impl<'a> HistoryResource<'a> {
    pub(crate) fn new(api: &'a AccountingApi) -> Self {
        Self { api }
    }

    /// Retrieves the history of changes for a specific resource.
    pub async fn list(
        &self,
        endpoint: &str,
        guid: Uuid,
    ) -> Result<Vec<history::HistoryRecord>, XeroError> {
        let path = format!("/{endpoint}/{guid}/history");
        let resp: history::HistoryRecordsResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        Ok(resp.history_records)
    }

    /// Adds a note to the history of a specific resource.
    pub async fn add_note(
        &self,
        endpoint: &str,
        guid: Uuid,
        details: String,
    ) -> Result<Vec<history::HistoryRecord>, XeroError> {
        #[derive(Serialize)]
        #[serde(rename_all = "PascalCase")]
        struct HistoryNote {
            details: String,
        }

        #[derive(Serialize)]
        #[serde(rename_all = "PascalCase")]
        struct HistoryNoteRequest {
            history_records: Vec<HistoryNote>,
        }

        let path = format!("/{endpoint}/{guid}/history");
        let body = HistoryNoteRequest {
            history_records: vec![HistoryNote { details }],
        };
        let resp: history::HistoryRecordsResponse = self
            .api
            .client
            .send_request(Method::PUT, &path, None, Some(body))
            .await?;
        Ok(resp.history_records)
    }
}

impl AccountingApi {
    /// Retrieves the history of changes for a specific resource.
    pub async fn get_history(
        &self,
        endpoint: &str,
        guid: Uuid,
    ) -> Result<Vec<history::HistoryRecord>, XeroError> {
        self.history().list(endpoint, guid).await
    }

    /// Adds a note to the history of a specific resource.
    pub async fn create_history_note(
        &self,
        endpoint: &str,
        guid: Uuid,
        details: String,
    ) -> Result<Vec<history::HistoryRecord>, XeroError> {
        self.history().add_note(endpoint, guid, details).await
    }
}
