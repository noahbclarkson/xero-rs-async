use super::AccountingApi;
use crate::error::XeroError;
use crate::models::accounting::attachment::{Attachment, AttachmentsResponse};
use reqwest::Method;
use uuid::Uuid;

/// Supported Accounting endpoints for attachments.
#[derive(Debug, Clone, Copy)]
pub enum AttachmentEndpoint {
    Invoices,
    Receipts,
    CreditNotes,
    RepeatingInvoices,
    BankTransactions,
    BankTransfers,
    Contacts,
    Accounts,
    ManualJournals,
    PurchaseOrders,
    Quotes,
}

impl AttachmentEndpoint {
    fn as_str(self) -> &'static str {
        match self {
            AttachmentEndpoint::Invoices => "Invoices",
            AttachmentEndpoint::Receipts => "Receipts",
            AttachmentEndpoint::CreditNotes => "CreditNotes",
            AttachmentEndpoint::RepeatingInvoices => "RepeatingInvoices",
            AttachmentEndpoint::BankTransactions => "BankTransactions",
            AttachmentEndpoint::BankTransfers => "BankTransfers",
            AttachmentEndpoint::Contacts => "Contacts",
            AttachmentEndpoint::Accounts => "Accounts",
            AttachmentEndpoint::ManualJournals => "ManualJournals",
            AttachmentEndpoint::PurchaseOrders => "PurchaseOrders",
            AttachmentEndpoint::Quotes => "Quotes",
        }
    }
}

/// Resource accessor for Attachments.
#[derive(Debug, Clone, Copy)]
pub struct AttachmentsResource<'a> {
    api: &'a AccountingApi,
}

impl<'a> AttachmentsResource<'a> {
    pub(crate) fn new(api: &'a AccountingApi) -> Self {
        Self { api }
    }

    /// Retrieves attachments for a specific resource.
    pub async fn list(
        &self,
        endpoint: AttachmentEndpoint,
        parent_id: Uuid,
    ) -> Result<Vec<Attachment>, XeroError> {
        let path = format!("/{}/{}/Attachments", endpoint.as_str(), parent_id);
        let resp: AttachmentsResponse = self
            .api
            .client
            .send_request(Method::GET, &path, None, None::<()>)
            .await?;
        Ok(resp.attachments)
    }

    /// Downloads the raw content of an attachment.
    pub async fn download(
        &self,
        endpoint: AttachmentEndpoint,
        parent_id: Uuid,
        filename: &str,
    ) -> Result<Vec<u8>, XeroError> {
        let path = format!(
            "/{}/{}/Attachments/{}",
            endpoint.as_str(),
            parent_id,
            filename
        );
        self.api
            .client
            .send_request_bytes(Method::GET, &path, None)
            .await
    }

    /// Uploads an attachment using POST.
    pub async fn upload_post<B>(
        &self,
        endpoint: AttachmentEndpoint,
        parent_id: Uuid,
        filename: &str,
        content_type: &str,
        body: B,
        include_online: Option<bool>,
    ) -> Result<Vec<Attachment>, XeroError>
    where
        B: Into<reqwest::Body>,
    {
        self.upload(
            Method::POST,
            endpoint,
            parent_id,
            filename,
            content_type,
            body,
            include_online,
        )
        .await
    }

    /// Uploads an attachment using PUT.
    pub async fn upload_put<B>(
        &self,
        endpoint: AttachmentEndpoint,
        parent_id: Uuid,
        filename: &str,
        content_type: &str,
        body: B,
        include_online: Option<bool>,
    ) -> Result<Vec<Attachment>, XeroError>
    where
        B: Into<reqwest::Body>,
    {
        self.upload(
            Method::PUT,
            endpoint,
            parent_id,
            filename,
            content_type,
            body,
            include_online,
        )
        .await
    }

    async fn upload<B>(
        &self,
        method: Method,
        endpoint: AttachmentEndpoint,
        parent_id: Uuid,
        filename: &str,
        content_type: &str,
        body: B,
        include_online: Option<bool>,
    ) -> Result<Vec<Attachment>, XeroError>
    where
        B: Into<reqwest::Body>,
    {
        let mut path = format!(
            "/{}/{}/Attachments/{}",
            endpoint.as_str(),
            parent_id,
            filename
        );
        if let Some(include_online) = include_online {
            path.push_str(&format!(
                "?IncludeOnline={}",
                if include_online { "true" } else { "false" }
            ));
        }

        let resp: AttachmentsResponse = self
            .api
            .client
            .send_request_raw_body(method, &path, content_type, body)
            .await?;
        Ok(resp.attachments)
    }
}

impl AccountingApi {
    /// Retrieves attachments for a specific resource.
    pub async fn get_attachments(
        &self,
        endpoint: AttachmentEndpoint,
        parent_id: Uuid,
    ) -> Result<Vec<Attachment>, XeroError> {
        self.attachments().list(endpoint, parent_id).await
    }

    /// Downloads an attachment's raw content.
    pub async fn download_attachment(
        &self,
        endpoint: AttachmentEndpoint,
        parent_id: Uuid,
        filename: &str,
    ) -> Result<Vec<u8>, XeroError> {
        self.attachments()
            .download(endpoint, parent_id, filename)
            .await
    }

    /// Uploads an attachment using POST.
    pub async fn upload_attachment_post<B>(
        &self,
        endpoint: AttachmentEndpoint,
        parent_id: Uuid,
        filename: &str,
        content_type: &str,
        body: B,
        include_online: Option<bool>,
    ) -> Result<Vec<Attachment>, XeroError>
    where
        B: Into<reqwest::Body>,
    {
        self.attachments()
            .upload_post(
                endpoint,
                parent_id,
                filename,
                content_type,
                body,
                include_online,
            )
            .await
    }

    /// Uploads an attachment using PUT.
    pub async fn upload_attachment_put<B>(
        &self,
        endpoint: AttachmentEndpoint,
        parent_id: Uuid,
        filename: &str,
        content_type: &str,
        body: B,
        include_online: Option<bool>,
    ) -> Result<Vec<Attachment>, XeroError>
    where
        B: Into<reqwest::Body>,
    {
        self.attachments()
            .upload_put(
                endpoint,
                parent_id,
                filename,
                content_type,
                body,
                include_online,
            )
            .await
    }
}
