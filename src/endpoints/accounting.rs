//! Entry point for interacting with the Xero Accounting API.

use crate::client::XeroClient;
use crate::error::XeroError;
use crate::models::accounting::common::Allocation;
use crate::models::accounting::contact::{CISSettings, CISSettingsResponse};
use crate::models::accounting::{
    account, attachment, bank_transaction, bank_transfer, batch_payment, branding_theme, budget,
    contact, contact_group, credit_note, currency, employee, expense_claim, history, invoice, item,
    journal, linked_transaction, manual_journal, organisation, overpayment, payment,
    payment_service, prepayment, purchase_order, quote, receipt, repeating_invoice, report,
    tax_rate, tracking_category, user,
};
use chrono::{DateTime, Utc};
use log::{debug, error, trace};
use reqwest::Method;
use serde::de::DeserializeOwned;
use serde::Serialize;
use uuid::Uuid;

const BASE_URL: &str = "https://api.xero.com/api.xro/2.0";

/// A handle to the Accounting API endpoints.
#[derive(Debug, Clone)]
pub struct AccountingApi {
    client: XeroClient,
}

/// Private helper methods for the Accounting API.
impl AccountingApi {
    async fn send_request<R, B>(
        &self,
        method: Method,
        tenant_id: Uuid,
        path: &str,
        query: Option<&[(String, String)]>,
        body: Option<B>,
    ) -> Result<R, XeroError>
    where
        R: DeserializeOwned,
        B: Serialize,
    {
        let url = format!("{}{}", BASE_URL, path);
        debug!("Sending API request: {} {}", method, url);
        if let Some(q) = &query {
            trace!("Request query: {:?}", q);
        }
        if body.is_some() {
            trace!("Request has a JSON body.");
        }

        let mut builder = self
            .client
            .http_client
            .request(method, &url)
            .bearer_auth(self.client.token_manager.get_access_token().await?)
            .header("xero-tenant-id", tenant_id.to_string())
            .header("Accept", "application/json");

        if let Some(q) = query {
            builder = builder.query(q);
        }
        if let Some(b) = body {
            builder = builder.json(&b);
        }

        let _permit = self.client.rate_limiter.acquire_permit(tenant_id).await?;
        trace!("Rate limiter permit acquired for tenant {}", tenant_id);
        let response = builder.send().await?;

        if response.status().is_success() {
            trace!("API request successful with status: {}", response.status());
            // Read the response body to text first
            let response_text = response.text().await?;
            // Then attempt to deserialize, logging the raw text on failure
            serde_json::from_str::<R>(&response_text).map_err(|e| {
                error!("Failed to deserialize JSON response from {}: {}", url, e);
                debug!(
                    "Raw JSON response that failed to parse:\n---\n{}\n---",
                    // Take first 10,000 characters for brevity
                    response_text.chars().take(10_000).collect::<String>()
                );
                XeroError::from(e)
            })
        } else {
            let status = response.status();
            let message = response.text().await?;
            error!(
                "API request failed with status: {}. Message: {}",
                status, message
            );
            Err(XeroError::Api { status, message })
        }
    }

    async fn send_request_empty_response<B>(
        &self,
        method: Method,
        tenant_id: Uuid,
        path: &str,
        body: Option<B>,
    ) -> Result<(), XeroError>
    where
        B: Serialize,
    {
        let url = format!("{}{}", BASE_URL, path);
        let mut builder = self
            .client
            .http_client
            .request(method, &url)
            .bearer_auth(self.client.token_manager.get_access_token().await?)
            .header("xero-tenant-id", tenant_id.to_string());

        if let Some(b) = body {
            builder = builder.json(&b);
        }

        let _permit = self.client.rate_limiter.acquire_permit(tenant_id).await?;
        let response = builder.send().await?;

        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status();
            let message = response.text().await?;
            Err(XeroError::Api { status, message })
        }
    }

    async fn send_request_raw_body<B>(
        &self,
        method: Method,
        tenant_id: Uuid,
        path: &str,
        content_type: &str,
        body: B,
    ) -> Result<Vec<attachment::Attachment>, XeroError>
    where
        B: Into<reqwest::Body>,
    {
        let url = format!("{}{}", BASE_URL, path);
        let builder = self
            .client
            .http_client
            .request(method, &url)
            .bearer_auth(self.client.token_manager.get_access_token().await?)
            .header("xero-tenant-id", tenant_id.to_string())
            .header("Accept", "application/json")
            .header("Content-Type", content_type)
            .body(body);

        let _permit = self.client.rate_limiter.acquire_permit(tenant_id).await?;
        let response = builder.send().await?;

        if response.status().is_success() {
            let response_text = response.text().await?;
            serde_json::from_str::<attachment::AttachmentsResponse>(&response_text)
                .map(|r| r.attachments)
                .map_err(|e| {
                    error!(
                        "Failed to deserialize attachment response from {}: {}",
                        url, e
                    );
                    debug!(
                        "Raw JSON response for attachment that failed to parse:\n---\n{}\n---",
                        response_text.chars().take(10_000).collect::<String>()
                    );
                    XeroError::from(e)
                })
        } else {
            let status = response.status();
            let message = response.text().await?;
            Err(XeroError::Api { status, message })
        }
    }
}

impl AccountingApi {
    pub(crate) fn new(client: XeroClient) -> Self {
        Self { client }
    }

    // --- Accounts ---
    /// Retrieves the full chart of accounts or a specific account by its ID.
    pub async fn get_accounts(
        &self,
        tenant_id: Uuid,
        account_id: Option<Uuid>,
        _modified_after: Option<DateTime<Utc>>,
        where_filter: Option<String>,
        order_by: Option<String>,
    ) -> Result<Vec<account::Account>, XeroError> {
        let path = if let Some(id) = account_id {
            format!("/Accounts/{}", id)
        } else {
            "/Accounts".to_string()
        };
        let mut query = Vec::new();
        if let Some(filter) = where_filter {
            query.push(("where".to_string(), filter));
        }
        if let Some(order) = order_by {
            query.push(("order".to_string(), order));
        }
        // TODO: Handle If-Modified-Since header
        let resp: account::AccountsResponse = self
            .send_request(Method::GET, tenant_id, &path, Some(&query), None::<()>)
            .await?;
        Ok(resp.accounts)
    }

    /// Creates one or more new accounts.
    pub async fn create_accounts(
        &self,
        tenant_id: Uuid,
        accounts: Vec<account::Account>,
    ) -> Result<Vec<account::Account>, XeroError> {
        let body = if accounts.len() == 1 {
            serde_json::to_value(accounts.into_iter().next().unwrap())?
        } else {
            serde_json::to_value(account::AccountsRequest { accounts })?
        };
        let resp: account::AccountsResponse = self
            .send_request(Method::PUT, tenant_id, "/Accounts", None, Some(body))
            .await?;
        Ok(resp.accounts)
    }

    /// Updates an existing account.
    pub async fn update_account(
        &self,
        tenant_id: Uuid,
        account_id: Uuid,
        account_data: account::Account,
    ) -> Result<Vec<account::Account>, XeroError> {
        let path = format!("/Accounts/{}", account_id);
        let resp: account::AccountsResponse = self
            .send_request(Method::POST, tenant_id, &path, None, Some(account_data))
            .await?;
        Ok(resp.accounts)
    }

    /// Deletes an account.
    pub async fn delete_account(&self, tenant_id: Uuid, account_id: Uuid) -> Result<(), XeroError> {
        let path = format!("/Accounts/{}", account_id);
        self.send_request_empty_response(Method::DELETE, tenant_id, &path, None::<()>)
            .await
    }

    /// Attaches a file to an account.
    pub async fn create_account_attachment_by_file_name(
        &self,
        tenant_id: Uuid,
        account_id: Uuid,
        file_name: String,
        body: Vec<u8>,
    ) -> Result<Vec<attachment::Attachment>, XeroError> {
        // Custom encoding as per API documentation note
        let encoded_file_name = file_name.replace('[', "%5B").replace(']', "%5D");
        let path = format!("/Accounts/{}/Attachments/{}", account_id, encoded_file_name);
        let content_type = "application/octet-stream";
        self.send_request_raw_body(Method::PUT, tenant_id, &path, content_type, body)
            .await
    }

    // --- Bank Transactions ---
    /// Retrieves one or many bank transactions.
    #[allow(clippy::too_many_arguments)]
    pub async fn get_bank_transactions(
        &self,
        tenant_id: Uuid,
        bank_transaction_id: Option<Uuid>,
        _modified_after: Option<DateTime<Utc>>,
        where_filter: Option<String>,
        order_by: Option<String>,
        page: Option<u32>,
        page_size: Option<u32>,
    ) -> Result<Vec<bank_transaction::BankTransaction>, XeroError> {
        let path = if let Some(id) = bank_transaction_id {
            format!("/BankTransactions/{}", id)
        } else {
            "/BankTransactions".to_string()
        };
        let mut query = Vec::new();
        if let Some(filter) = where_filter {
            query.push(("where".to_string(), filter));
        }
        if let Some(order) = order_by {
            query.push(("order".to_string(), order));
        }
        if let Some(p) = page {
            query.push(("page".to_string(), p.to_string()));
        }
        if let Some(ps) = page_size {
            query.push(("pageSize".to_string(), ps.to_string()));
        }
        let resp: bank_transaction::BankTransactionsResponse = self
            .send_request(Method::GET, tenant_id, &path, Some(&query), None::<()>)
            .await?;
        Ok(resp.bank_transactions)
    }

    /// Creates one or more new spend or receive money transactions.
    pub async fn create_bank_transactions(
        &self,
        tenant_id: Uuid,
        transactions: Vec<bank_transaction::BankTransaction>,
        summarize_errors: Option<bool>,
    ) -> Result<Vec<bank_transaction::BankTransaction>, XeroError> {
        let mut query = Vec::new();
        if let Some(s) = summarize_errors {
            query.push(("summarizeErrors".to_string(), s.to_string()));
        }
        let body = if transactions.len() == 1 {
            serde_json::to_value(transactions.into_iter().next().unwrap())?
        } else {
            serde_json::to_value(bank_transaction::BankTransactionsRequest {
                bank_transactions: transactions,
            })?
        };
        let resp: bank_transaction::BankTransactionsResponse = self
            .send_request(
                Method::PUT,
                tenant_id,
                "/BankTransactions",
                Some(&query),
                Some(body),
            )
            .await?;
        Ok(resp.bank_transactions)
    }

    /// Updates an existing spend or receive money transaction.
    pub async fn update_bank_transaction(
        &self,
        tenant_id: Uuid,
        bank_transaction_id: Uuid,
        transaction_data: bank_transaction::BankTransaction,
    ) -> Result<Vec<bank_transaction::BankTransaction>, XeroError> {
        let path = format!("/BankTransactions/{}", bank_transaction_id);
        let resp: bank_transaction::BankTransactionsResponse = self
            .send_request(Method::POST, tenant_id, &path, None, Some(transaction_data))
            .await?;
        Ok(resp.bank_transactions)
    }

    // --- Bank Transfers ---
    /// Retrieves one or many bank transfers.
    pub async fn get_bank_transfers(
        &self,
        tenant_id: Uuid,
        bank_transfer_id: Option<Uuid>,
        _modified_after: Option<DateTime<Utc>>,
        where_filter: Option<String>,
        order_by: Option<String>,
    ) -> Result<Vec<bank_transfer::BankTransfer>, XeroError> {
        let path = if let Some(id) = bank_transfer_id {
            format!("/BankTransfers/{}", id)
        } else {
            "/BankTransfers".to_string()
        };
        let mut query = Vec::new();
        if let Some(filter) = where_filter {
            query.push(("where".to_string(), filter));
        }
        if let Some(order) = order_by {
            query.push(("order".to_string(), order));
        }
        let resp: bank_transfer::BankTransfersResponse = self
            .send_request(Method::GET, tenant_id, &path, Some(&query), None::<()>)
            .await?;
        Ok(resp.bank_transfers)
    }

    /// Creates a new bank transfer.
    pub async fn create_bank_transfer(
        &self,
        tenant_id: Uuid,
        transfer: bank_transfer::BankTransfer,
    ) -> Result<Vec<bank_transfer::BankTransfer>, XeroError> {
        let resp: bank_transfer::BankTransfersResponse = self
            .send_request(
                Method::PUT,
                tenant_id,
                "/BankTransfers",
                None,
                Some(bank_transfer::BankTransfersRequest {
                    bank_transfers: vec![transfer],
                }),
            )
            .await?;
        Ok(resp.bank_transfers)
    }

    // --- Batch Payments ---
    /// Retrieves one or many batch payments.
    pub async fn get_batch_payments(
        &self,
        tenant_id: Uuid,
        batch_payment_id: Option<Uuid>,
        _modified_after: Option<DateTime<Utc>>,
        where_filter: Option<String>,
        order_by: Option<String>,
    ) -> Result<Vec<batch_payment::BatchPayment>, XeroError> {
        let path = if let Some(id) = batch_payment_id {
            format!("/BatchPayments/{}", id)
        } else {
            "/BatchPayments".to_string()
        };
        let mut query = Vec::new();
        if let Some(filter) = where_filter {
            query.push(("where".to_string(), filter));
        }
        if let Some(order) = order_by {
            query.push(("order".to_string(), order));
        }
        let resp: batch_payment::BatchPaymentsResponse = self
            .send_request(Method::GET, tenant_id, &path, Some(&query), None::<()>)
            .await?;
        Ok(resp.batch_payments)
    }

    /// Creates a new batch payment.
    pub async fn create_batch_payment(
        &self,
        tenant_id: Uuid,
        batch_payment: batch_payment::BatchPayment,
    ) -> Result<Vec<batch_payment::BatchPayment>, XeroError> {
        let resp: batch_payment::BatchPaymentsResponse = self
            .send_request(
                Method::PUT,
                tenant_id,
                "/BatchPayments",
                None,
                Some(batch_payment),
            )
            .await?;
        Ok(resp.batch_payments)
    }

    /// Updates a batch payment status to DELETED.
    pub async fn delete_batch_payment(
        &self,
        tenant_id: Uuid,
        batch_payment_id: Uuid,
    ) -> Result<Vec<batch_payment::BatchPayment>, XeroError> {
        #[derive(Serialize)]
        #[serde(rename_all = "PascalCase")]
        struct DeleteRequest {
            status: batch_payment::BatchPaymentStatus,
        }
        let path = format!("/BatchPayments/{}", batch_payment_id);
        let body = DeleteRequest {
            status: batch_payment::BatchPaymentStatus::Deleted,
        };
        let resp: batch_payment::BatchPaymentsResponse = self
            .send_request(Method::POST, tenant_id, &path, None, Some(body))
            .await?;
        Ok(resp.batch_payments)
    }

    // --- Branding Themes ---
    /// Retrieves a list of branding themes.
    pub async fn get_branding_themes(
        &self,
        tenant_id: Uuid,
        branding_theme_id: Option<Uuid>,
    ) -> Result<Vec<branding_theme::BrandingTheme>, XeroError> {
        let path = if let Some(id) = branding_theme_id {
            format!("/BrandingThemes/{}", id)
        } else {
            "/BrandingThemes".to_string()
        };
        let resp: branding_theme::BrandingThemesResponse = self
            .send_request(Method::GET, tenant_id, &path, None, None::<()>)
            .await?;
        Ok(resp.branding_themes)
    }

    /// Retrieves payment services for a branding theme.
    pub async fn get_branding_theme_payment_services(
        &self,
        tenant_id: Uuid,
        branding_theme_id: Uuid,
    ) -> Result<Vec<payment_service::PaymentService>, XeroError> {
        let path = format!("/BrandingThemes/{}/PaymentServices", branding_theme_id);
        let resp: payment_service::PaymentServicesResponse = self
            .send_request(Method::GET, tenant_id, &path, None, None::<()>)
            .await?;
        Ok(resp.payment_services)
    }

    /// Applies a payment service to a branding theme.
    pub async fn create_branding_theme_payment_service(
        &self,
        tenant_id: Uuid,
        branding_theme_id: Uuid,
        payment_service_id: Uuid,
    ) -> Result<Vec<payment_service::PaymentService>, XeroError> {
        #[derive(Serialize)]
        struct RequestBody {
            #[serde(rename = "PaymentServiceID")]
            id: Uuid,
        }
        #[derive(Serialize)]
        struct RequestWrapper {
            #[serde(rename = "PaymentServices")]
            services: Vec<RequestBody>,
        }
        let path = format!("/BrandingThemes/{}/PaymentServices", branding_theme_id);
        let body = RequestWrapper {
            services: vec![RequestBody {
                id: payment_service_id,
            }],
        };
        let resp: payment_service::PaymentServicesResponse = self
            .send_request(Method::POST, tenant_id, &path, None, Some(body))
            .await?;
        Ok(resp.payment_services)
    }

    // --- Budgets ---
    /// Retrieves one or many budgets.
    pub async fn get_budgets(
        &self,
        tenant_id: Uuid,
        budget_id: Option<Uuid>,
        date_to: Option<String>,
        date_from: Option<String>,
    ) -> Result<Vec<budget::Budget>, XeroError> {
        let path = if let Some(id) = budget_id {
            format!("/Budgets/{}", id)
        } else {
            "/Budgets".to_string()
        };
        let mut query = Vec::new();
        if let Some(d) = date_to {
            query.push(("DateTo".to_string(), d));
        }
        if let Some(d) = date_from {
            query.push(("DateFrom".to_string(), d));
        }
        let resp: budget::BudgetsResponse = self
            .send_request(Method::GET, tenant_id, &path, Some(&query), None::<()>)
            .await?;
        Ok(resp.budgets)
    }

    // --- Contacts ---
    /// Retrieves one or many contacts.
    #[allow(clippy::too_many_arguments)]
    pub async fn get_contacts(
        &self,
        tenant_id: Uuid,
        contact_id: Option<Uuid>,
        ids: Option<Vec<Uuid>>,
        _modified_after: Option<DateTime<Utc>>,
        where_filter: Option<String>,
        order_by: Option<String>,
        page: Option<u32>,
        page_size: Option<u32>,
        include_archived: Option<bool>,
        summary_only: Option<bool>,
        search_term: Option<String>,
    ) -> Result<Vec<contact::Contact>, XeroError> {
        let path = if let Some(id) = contact_id {
            format!("/Contacts/{}", id)
        } else {
            "/Contacts".to_string()
        };
        let mut query = Vec::new();
        if let Some(i) = ids {
            query.push((
                "IDs".to_string(),
                i.iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
            ));
        }
        if let Some(filter) = where_filter {
            query.push(("where".to_string(), filter));
        }
        if let Some(order) = order_by {
            query.push(("order".to_string(), order));
        }
        if let Some(p) = page {
            query.push(("page".to_string(), p.to_string()));
        }
        if let Some(ps) = page_size {
            query.push(("pageSize".to_string(), ps.to_string()));
        }
        if let Some(ia) = include_archived {
            query.push(("includeArchived".to_string(), ia.to_string()));
        }
        if let Some(so) = summary_only {
            query.push(("summaryOnly".to_string(), so.to_string()));
        }
        if let Some(st) = search_term {
            query.push(("searchTerm".to_string(), st));
        }
        let resp: contact::ContactsResponse = self
            .send_request(Method::GET, tenant_id, &path, Some(&query), None::<()>)
            .await?;
        Ok(resp.contacts)
    }

    /// Creates one or more new contacts.
    pub async fn create_contacts(
        &self,
        tenant_id: Uuid,
        contacts: Vec<contact::Contact>,
    ) -> Result<Vec<contact::Contact>, XeroError> {
        let body = if contacts.len() == 1 {
            serde_json::to_value(contacts.into_iter().next().unwrap())?
        } else {
            serde_json::to_value(contact::ContactsRequest { contacts })?
        };
        let resp: contact::ContactsResponse = self
            .send_request(Method::PUT, tenant_id, "/Contacts", None, Some(body))
            .await?;
        Ok(resp.contacts)
    }

    /// Updates an existing contact.
    pub async fn update_contact(
        &self,
        tenant_id: Uuid,
        contact_id: Uuid,
        contact_data: contact::Contact,
    ) -> Result<Vec<contact::Contact>, XeroError> {
        let path = format!("/Contacts/{}", contact_id);
        let resp: contact::ContactsResponse = self
            .send_request(Method::POST, tenant_id, &path, None, Some(contact_data))
            .await?;
        Ok(resp.contacts)
    }

    /// Retrieves CIS settings for a contact (UK only).
    pub async fn get_contact_cis_settings(
        &self,
        tenant_id: Uuid,
        contact_id: Uuid,
    ) -> Result<Vec<CISSettings>, XeroError> {
        let path = format!("/Contacts/{}/CISSettings", contact_id);
        let resp: CISSettingsResponse = self
            .send_request(Method::GET, tenant_id, &path, None, None::<()>)
            .await?;
        Ok(resp.cis_settings)
    }

    // --- Contact Groups ---
    /// Retrieves one or all contact groups.
    pub async fn get_contact_groups(
        &self,
        tenant_id: Uuid,
        contact_group_id: Option<Uuid>,
        where_filter: Option<String>,
        order_by: Option<String>,
    ) -> Result<Vec<contact_group::ContactGroup>, XeroError> {
        let path = if let Some(id) = contact_group_id {
            format!("/ContactGroups/{}", id)
        } else {
            "/ContactGroups".to_string()
        };
        let mut query = Vec::new();
        if let Some(filter) = where_filter {
            query.push(("where".to_string(), filter));
        }
        if let Some(order) = order_by {
            query.push(("order".to_string(), order));
        }
        let resp: contact_group::ContactGroupsResponse = self
            .send_request(Method::GET, tenant_id, &path, Some(&query), None::<()>)
            .await?;
        Ok(resp.contact_groups)
    }

    /// Creates a new contact group.
    pub async fn create_contact_group(
        &self,
        tenant_id: Uuid,
        contact_group: contact_group::ContactGroup,
    ) -> Result<Vec<contact_group::ContactGroup>, XeroError> {
        let resp: contact_group::ContactGroupsResponse = self
            .send_request(
                Method::PUT,
                tenant_id,
                "/ContactGroups",
                None,
                Some(contact_group),
            )
            .await?;
        Ok(resp.contact_groups)
    }

    /// Updates a contact group.
    pub async fn update_contact_group(
        &self,
        tenant_id: Uuid,
        contact_group_id: Uuid,
        contact_group_data: contact_group::ContactGroup,
    ) -> Result<Vec<contact_group::ContactGroup>, XeroError> {
        let path = format!("/ContactGroups/{}", contact_group_id);
        let resp: contact_group::ContactGroupsResponse = self
            .send_request(
                Method::POST,
                tenant_id,
                &path,
                None,
                Some(contact_group_data),
            )
            .await?;
        Ok(resp.contact_groups)
    }

    /// Adds contacts to a contact group.
    pub async fn add_contacts_to_group(
        &self,
        tenant_id: Uuid,
        contact_group_id: Uuid,
        contacts: Vec<contact::Contact>,
    ) -> Result<Vec<contact::Contact>, XeroError> {
        let path = format!("/ContactGroups/{}/Contacts", contact_group_id);
        let body = contact::ContactsRequest { contacts };
        let resp: contact::ContactsResponse = self
            .send_request(Method::PUT, tenant_id, &path, None, Some(body))
            .await?;
        Ok(resp.contacts)
    }

    /// Removes a specific contact from a contact group.
    pub async fn remove_contact_from_group(
        &self,
        tenant_id: Uuid,
        contact_group_id: Uuid,
        contact_id: Uuid,
    ) -> Result<(), XeroError> {
        let path = format!(
            "/ContactGroups/{}/Contacts/{}",
            contact_group_id, contact_id
        );
        self.send_request_empty_response(Method::DELETE, tenant_id, &path, None::<()>)
            .await
    }

    /// Removes all contacts from a contact group.
    pub async fn remove_all_contacts_from_group(
        &self,
        tenant_id: Uuid,
        contact_group_id: Uuid,
    ) -> Result<(), XeroError> {
        let path = format!("/ContactGroups/{}/Contacts", contact_group_id);
        self.send_request_empty_response(Method::DELETE, tenant_id, &path, None::<()>)
            .await
    }

    // --- Credit Notes ---
    /// Retrieves one or many credit notes.
    #[allow(clippy::too_many_arguments)]
    pub async fn get_credit_notes(
        &self,
        tenant_id: Uuid,
        credit_note_id: Option<Uuid>,
        _modified_after: Option<DateTime<Utc>>,
        where_filter: Option<String>,
        order_by: Option<String>,
        page: Option<u32>,
        page_size: Option<u32>,
    ) -> Result<Vec<credit_note::CreditNote>, XeroError> {
        let path = if let Some(id) = credit_note_id {
            format!("/CreditNotes/{}", id)
        } else {
            "/CreditNotes".to_string()
        };
        let mut query = Vec::new();
        if let Some(filter) = where_filter {
            query.push(("where".to_string(), filter));
        }
        if let Some(order) = order_by {
            query.push(("order".to_string(), order));
        }
        if let Some(p) = page {
            query.push(("page".to_string(), p.to_string()));
        }
        if let Some(ps) = page_size {
            query.push(("pageSize".to_string(), ps.to_string()));
        }
        let resp: credit_note::CreditNotesResponse = self
            .send_request(Method::GET, tenant_id, &path, Some(&query), None::<()>)
            .await?;
        Ok(resp.credit_notes)
    }

    /// Creates one or more new credit notes.
    pub async fn create_credit_notes(
        &self,
        tenant_id: Uuid,
        credit_notes: Vec<credit_note::CreditNote>,
        summarize_errors: Option<bool>,
    ) -> Result<Vec<credit_note::CreditNote>, XeroError> {
        let mut query = Vec::new();
        if let Some(s) = summarize_errors {
            query.push(("summarizeErrors".to_string(), s.to_string()));
        }
        let body = if credit_notes.len() == 1 {
            serde_json::to_value(credit_notes.into_iter().next().unwrap())?
        } else {
            serde_json::to_value(credit_note::CreditNotesRequest { credit_notes })?
        };
        let resp: credit_note::CreditNotesResponse = self
            .send_request(
                Method::PUT,
                tenant_id,
                "/CreditNotes",
                Some(&query),
                Some(body),
            )
            .await?;
        Ok(resp.credit_notes)
    }

    /// Updates an existing credit note.
    pub async fn update_credit_note(
        &self,
        tenant_id: Uuid,
        credit_note_id: Uuid,
        credit_note_data: credit_note::CreditNote,
    ) -> Result<Vec<credit_note::CreditNote>, XeroError> {
        let path = format!("/CreditNotes/{}", credit_note_id);
        let resp: credit_note::CreditNotesResponse = self
            .send_request(Method::POST, tenant_id, &path, None, Some(credit_note_data))
            .await?;
        Ok(resp.credit_notes)
    }

    /// Allocates a credit note to an invoice.
    pub async fn allocate_credit_note(
        &self,
        tenant_id: Uuid,
        credit_note_id: Uuid,
        allocation: Allocation,
    ) -> Result<Vec<Allocation>, XeroError> {
        let path = format!("/CreditNotes/{}/Allocations", credit_note_id);
        let resp: credit_note::AllocationsResponse = self
            .send_request(Method::PUT, tenant_id, &path, None, Some(allocation))
            .await?;
        Ok(resp.allocations)
    }

    /// Deletes a credit note allocation.
    pub async fn delete_credit_note_allocation(
        &self,
        tenant_id: Uuid,
        credit_note_id: Uuid,
        allocation_id: Uuid,
    ) -> Result<(), XeroError> {
        let path = format!(
            "/CreditNotes/{}/Allocations/{}",
            credit_note_id, allocation_id
        );
        self.send_request_empty_response(Method::DELETE, tenant_id, &path, None::<()>)
            .await
    }

    // --- Currencies ---
    /// Retrieves currencies for your organisation.
    pub async fn get_currencies(
        &self,
        tenant_id: Uuid,
        where_filter: Option<String>,
        order_by: Option<String>,
    ) -> Result<Vec<currency::Currency>, XeroError> {
        let mut query = Vec::new();
        if let Some(filter) = where_filter {
            query.push(("where".to_string(), filter));
        }
        if let Some(order) = order_by {
            query.push(("order".to_string(), order));
        }
        let resp: currency::CurrenciesResponse = self
            .send_request(
                Method::GET,
                tenant_id,
                "/Currencies",
                Some(&query),
                None::<()>,
            )
            .await?;
        Ok(resp.currencies)
    }

    /// Adds a new currency to your organisation.
    pub async fn create_currency(
        &self,
        tenant_id: Uuid,
        currency: currency::Currency,
    ) -> Result<Vec<currency::Currency>, XeroError> {
        let resp: currency::CurrenciesResponse = self
            .send_request(Method::PUT, tenant_id, "/Currencies", None, Some(currency))
            .await?;
        Ok(resp.currencies)
    }

    // --- Employees ---
    /// Retrieves one or many employees.
    pub async fn get_employees(
        &self,
        tenant_id: Uuid,
        employee_id: Option<Uuid>,
        _modified_after: Option<DateTime<Utc>>,
        where_filter: Option<String>,
        order_by: Option<String>,
    ) -> Result<Vec<employee::Employee>, XeroError> {
        let path = if let Some(id) = employee_id {
            format!("/Employees/{}", id)
        } else {
            "/Employees".to_string()
        };
        let mut query = Vec::new();
        if let Some(filter) = where_filter {
            query.push(("where".to_string(), filter));
        }
        if let Some(order) = order_by {
            query.push(("order".to_string(), order));
        }
        let resp: employee::EmployeesResponse = self
            .send_request(Method::GET, tenant_id, &path, Some(&query), None::<()>)
            .await?;
        Ok(resp.employees)
    }

    /// Creates one or more new employees.
    pub async fn create_employees(
        &self,
        tenant_id: Uuid,
        employees: Vec<employee::Employee>,
    ) -> Result<Vec<employee::Employee>, XeroError> {
        let body = if employees.len() == 1 {
            serde_json::to_value(employees.into_iter().next().unwrap())?
        } else {
            serde_json::to_value(employee::EmployeesRequest { employees })?
        };
        let resp: employee::EmployeesResponse = self
            .send_request(Method::PUT, tenant_id, "/Employees", None, Some(body))
            .await?;
        Ok(resp.employees)
    }

    /// Updates an existing employee.
    pub async fn update_employee(
        &self,
        tenant_id: Uuid,
        employee_id: Uuid,
        employee_data: employee::Employee,
    ) -> Result<Vec<employee::Employee>, XeroError> {
        let path = format!("/Employees/{}", employee_id);
        let resp: employee::EmployeesResponse = self
            .send_request(Method::POST, tenant_id, &path, None, Some(employee_data))
            .await?;
        Ok(resp.employees)
    }

    // --- Expense Claims (Deprecated) ---
    /// Retrieves one or many expense claims.
    pub async fn get_expense_claims(
        &self,
        tenant_id: Uuid,
        expense_claim_id: Option<Uuid>,
        _modified_after: Option<DateTime<Utc>>,
        where_filter: Option<String>,
        order_by: Option<String>,
    ) -> Result<Vec<expense_claim::ExpenseClaim>, XeroError> {
        let path = if let Some(id) = expense_claim_id {
            format!("/ExpenseClaims/{}", id)
        } else {
            "/ExpenseClaims".to_string()
        };
        let mut query = Vec::new();
        if let Some(filter) = where_filter {
            query.push(("where".to_string(), filter));
        }
        if let Some(order) = order_by {
            query.push(("order".to_string(), order));
        }
        let resp: expense_claim::ExpenseClaimsResponse = self
            .send_request(Method::GET, tenant_id, &path, Some(&query), None::<()>)
            .await?;
        Ok(resp.expense_claims)
    }

    /// Creates or updates one or many expense claims.
    pub async fn create_or_update_expense_claims(
        &self,
        tenant_id: Uuid,
        claims: Vec<expense_claim::ExpenseClaim>,
        summarize_errors: Option<bool>,
    ) -> Result<Vec<expense_claim::ExpenseClaim>, XeroError> {
        let mut query = Vec::new();
        if let Some(s) = summarize_errors {
            query.push(("summarizeErrors".to_string(), s.to_string()));
        }
        let body = if claims.len() == 1 {
            serde_json::to_value(claims.into_iter().next().unwrap())?
        } else {
            serde_json::to_value(expense_claim::ExpenseClaimsRequest {
                expense_claims: claims,
            })?
        };
        let resp: expense_claim::ExpenseClaimsResponse = self
            .send_request(
                Method::POST,
                tenant_id,
                "/ExpenseClaims",
                Some(&query),
                Some(body),
            )
            .await?;
        Ok(resp.expense_claims)
    }

    // --- History & Notes ---
    /// Retrieves the history of changes for a specific resource.
    pub async fn get_history(
        &self,
        tenant_id: Uuid,
        endpoint: &str,
        guid: Uuid,
    ) -> Result<Vec<history::HistoryRecord>, XeroError> {
        let path = format!("/{}/{}/history", endpoint, guid);
        let resp: history::HistoryRecordsResponse = self
            .send_request(Method::GET, tenant_id, &path, None, None::<()>)
            .await?;
        Ok(resp.history_records)
    }

    /// Adds a note to the history of a specific resource.
    pub async fn create_history_note(
        &self,
        tenant_id: Uuid,
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

        let path = format!("/{}/{}/history", endpoint, guid);
        let body = HistoryNoteRequest {
            history_records: vec![HistoryNote { details }],
        };
        let resp: history::HistoryRecordsResponse = self
            .send_request(Method::PUT, tenant_id, &path, None, Some(body))
            .await?;
        Ok(resp.history_records)
    }

    // --- Invoices ---
    /// Retrieves one or many invoices.
    #[allow(clippy::too_many_arguments)]
    pub async fn get_invoices(
        &self,
        tenant_id: Uuid,
        invoice_id: Option<Uuid>,
        invoice_numbers: Option<Vec<String>>,
        contact_ids: Option<Vec<Uuid>>,
        statuses: Option<Vec<String>>,
        _modified_after: Option<DateTime<Utc>>,
        where_filter: Option<String>,
        order_by: Option<String>,
        page: Option<u32>,
        page_size: Option<u32>,
        summary_only: Option<bool>,
        search_term: Option<String>,
    ) -> Result<Vec<invoice::Invoice>, XeroError> {
        let path = if let Some(id) = invoice_id {
            format!("/Invoices/{}", id)
        } else {
            "/Invoices".to_string()
        };
        let mut query = Vec::new();
        if let Some(nums) = invoice_numbers {
            query.push(("InvoiceNumbers".to_string(), nums.join(",")));
        }
        if let Some(cids) = contact_ids {
            query.push((
                "ContactIDs".to_string(),
                cids.iter()
                    .map(Uuid::to_string)
                    .collect::<Vec<_>>()
                    .join(","),
            ));
        }
        if let Some(stats) = statuses {
            query.push(("Statuses".to_string(), stats.join(",")));
        }
        if let Some(filter) = where_filter {
            query.push(("where".to_string(), filter));
        }
        if let Some(order) = order_by {
            query.push(("order".to_string(), order));
        }
        if let Some(p) = page {
            query.push(("page".to_string(), p.to_string()));
        }
        if let Some(ps) = page_size {
            query.push(("pageSize".to_string(), ps.to_string()));
        }
        if let Some(so) = summary_only {
            query.push(("summaryOnly".to_string(), so.to_string()));
        }
        if let Some(st) = search_term {
            query.push(("SearchTerm".to_string(), st));
        }
        // TODO: Handle If-Modified-Since header
        let resp: invoice::InvoicesResponse = self
            .send_request(Method::GET, tenant_id, &path, Some(&query), None::<()>)
            .await?;
        Ok(resp.invoices)
    }

    /// Creates one or more new invoices.
    pub async fn create_invoices(
        &self,
        tenant_id: Uuid,
        invoices: Vec<invoice::Invoice>,
        summarize_errors: Option<bool>,
    ) -> Result<Vec<invoice::Invoice>, XeroError> {
        let mut query = Vec::new();
        if let Some(s) = summarize_errors {
            query.push(("summarizeErrors".to_string(), s.to_string()));
        }
        let body = if invoices.len() == 1 {
            serde_json::to_value(invoices.into_iter().next().unwrap())?
        } else {
            serde_json::to_value(invoice::InvoicesRequest { invoices })?
        };
        let resp: invoice::InvoicesResponse = self
            .send_request(
                Method::PUT,
                tenant_id,
                "/Invoices",
                Some(&query),
                Some(body),
            )
            .await?;
        Ok(resp.invoices)
    }

    /// Updates an existing invoice.
    pub async fn update_invoice(
        &self,
        tenant_id: Uuid,
        invoice_id: Uuid,
        invoice_data: invoice::Invoice,
    ) -> Result<Vec<invoice::Invoice>, XeroError> {
        let path = format!("/Invoices/{}", invoice_id);
        let resp: invoice::InvoicesResponse = self
            .send_request(Method::POST, tenant_id, &path, None, Some(invoice_data))
            .await?;
        Ok(resp.invoices)
    }

    /// Retrieves the online invoice URL for a sales invoice.
    pub async fn get_online_invoice_url(
        &self,
        tenant_id: Uuid,
        invoice_id: Uuid,
    ) -> Result<invoice::OnlineInvoice, XeroError> {
        let path = format!("/Invoices/{}/OnlineInvoice", invoice_id);
        let mut resp: invoice::OnlineInvoicesResponse = self
            .send_request(Method::GET, tenant_id, &path, None, None::<()>)
            .await?;
        resp.online_invoices.pop().ok_or(XeroError::Api {
            status: reqwest::StatusCode::NOT_FOUND,
            message: "OnlineInvoice not found in response".to_string(),
        })
    }

    /// Emails a sales invoice from Xero.
    pub async fn email_invoice(&self, tenant_id: Uuid, invoice_id: Uuid) -> Result<(), XeroError> {
        let path = format!("/Invoices/{}/Email", invoice_id);
        self.send_request_empty_response(Method::POST, tenant_id, &path, None::<()>)
            .await
    }

    // --- Items ---
    /// Retrieves one or many items.
    pub async fn get_items(
        &self,
        tenant_id: Uuid,
        item_id: Option<Uuid>,
        _modified_after: Option<DateTime<Utc>>,
        where_filter: Option<String>,
        order_by: Option<String>,
    ) -> Result<Vec<item::Item>, XeroError> {
        let path = if let Some(id) = item_id {
            format!("/Items/{}", id)
        } else {
            "/Items".to_string()
        };
        let mut query = Vec::new();
        if let Some(filter) = where_filter {
            query.push(("where".to_string(), filter));
        }
        if let Some(order) = order_by {
            query.push(("order".to_string(), order));
        }
        // TODO: Handle If-Modified-Since header
        let resp: item::ItemsResponse = self
            .send_request(Method::GET, tenant_id, &path, Some(&query), None::<()>)
            .await?;
        Ok(resp.items)
    }

    /// Creates one or more new items.
    pub async fn create_items(
        &self,
        tenant_id: Uuid,
        items: Vec<item::Item>,
    ) -> Result<Vec<item::Item>, XeroError> {
        let body = if items.len() == 1 {
            serde_json::to_value(items.into_iter().next().unwrap())?
        } else {
            serde_json::to_value(item::ItemsRequest { items })?
        };
        let resp: item::ItemsResponse = self
            .send_request(Method::PUT, tenant_id, "/Items", None, Some(body))
            .await?;
        Ok(resp.items)
    }

    /// Updates an existing item.
    pub async fn update_item(
        &self,
        tenant_id: Uuid,
        item_id: Uuid,
        item_data: item::Item,
    ) -> Result<Vec<item::Item>, XeroError> {
        let path = format!("/Items/{}", item_id);
        let resp: item::ItemsResponse = self
            .send_request(Method::POST, tenant_id, &path, None, Some(item_data))
            .await?;
        Ok(resp.items)
    }

    /// Deletes an item.
    pub async fn delete_item(&self, tenant_id: Uuid, item_id: Uuid) -> Result<(), XeroError> {
        let path = format!("/Items/{}", item_id);
        self.send_request_empty_response(Method::DELETE, tenant_id, &path, None::<()>)
            .await
    }

    // --- Journals ---
    /// Retrieves journals.
    pub async fn get_journals(
        &self,
        tenant_id: Uuid,
        offset: Option<u32>,
        payments_only: Option<bool>,
    ) -> Result<Vec<journal::Journal>, XeroError> {
        let mut query = Vec::new();
        if let Some(o) = offset {
            query.push(("offset".to_string(), o.to_string()));
        }
        if let Some(po) = payments_only {
            query.push(("paymentsOnly".to_string(), po.to_string()));
        }
        let resp: journal::JournalsResponse = self
            .send_request(
                Method::GET,
                tenant_id,
                "/Journals",
                Some(&query),
                None::<()>,
            )
            .await?;
        Ok(resp.journals)
    }

    // --- Linked Transactions ---
    /// Retrieves one or many linked transactions.
    #[allow(clippy::too_many_arguments)]
    pub async fn get_linked_transactions(
        &self,
        tenant_id: Uuid,
        linked_transaction_id: Option<Uuid>,
        source_transaction_id: Option<Uuid>,
        contact_id: Option<Uuid>,
        status: Option<String>,
        target_transaction_id: Option<Uuid>,
        page: Option<u32>,
    ) -> Result<Vec<linked_transaction::LinkedTransaction>, XeroError> {
        let path = if let Some(id) = linked_transaction_id {
            format!("/LinkedTransactions/{}", id)
        } else {
            "/LinkedTransactions".to_string()
        };
        let mut query = Vec::new();
        if let Some(id) = source_transaction_id {
            query.push(("SourceTransactionID".to_string(), id.to_string()));
        }
        if let Some(id) = contact_id {
            query.push(("ContactID".to_string(), id.to_string()));
        }
        if let Some(s) = status {
            query.push(("Status".to_string(), s));
        }
        if let Some(id) = target_transaction_id {
            query.push(("TargetTransactionID".to_string(), id.to_string()));
        }
        if let Some(p) = page {
            query.push(("page".to_string(), p.to_string()));
        }
        let resp: linked_transaction::LinkedTransactionsResponse = self
            .send_request(Method::GET, tenant_id, &path, Some(&query), None::<()>)
            .await?;
        Ok(resp.linked_transactions)
    }

    /// Creates or updates a linked transaction.
    pub async fn create_or_update_linked_transaction(
        &self,
        tenant_id: Uuid,
        transaction: linked_transaction::LinkedTransaction,
    ) -> Result<Vec<linked_transaction::LinkedTransaction>, XeroError> {
        let path = if let Some(id) = transaction.linked_transaction_id {
            format!("/LinkedTransactions/{}", id)
        } else {
            "/LinkedTransactions".to_string()
        };
        let resp: linked_transaction::LinkedTransactionsResponse = self
            .send_request(Method::POST, tenant_id, &path, None, Some(transaction))
            .await?;
        Ok(resp.linked_transactions)
    }

    /// Deletes a linked transaction.
    pub async fn delete_linked_transaction(
        &self,
        tenant_id: Uuid,
        linked_transaction_id: Uuid,
    ) -> Result<(), XeroError> {
        let path = format!("/LinkedTransactions/{}", linked_transaction_id);
        self.send_request_empty_response(Method::DELETE, tenant_id, &path, None::<()>)
            .await
    }

    // --- Manual Journals ---
    /// Retrieves one or many manual journals.
    #[allow(clippy::too_many_arguments)]
    pub async fn get_manual_journals(
        &self,
        tenant_id: Uuid,
        manual_journal_id: Option<Uuid>,
        _modified_after: Option<DateTime<Utc>>,
        where_filter: Option<String>,
        order_by: Option<String>,
        page: Option<u32>,
        page_size: Option<u32>,
    ) -> Result<Vec<manual_journal::ManualJournal>, XeroError> {
        let path = if let Some(id) = manual_journal_id {
            format!("/ManualJournals/{}", id)
        } else {
            "/ManualJournals".to_string()
        };
        let mut query = Vec::new();
        if let Some(filter) = where_filter {
            query.push(("where".to_string(), filter));
        }
        if let Some(order) = order_by {
            query.push(("order".to_string(), order));
        }
        if let Some(p) = page {
            query.push(("page".to_string(), p.to_string()));
        }
        if let Some(ps) = page_size {
            query.push(("pageSize".to_string(), ps.to_string()));
        }
        let resp: manual_journal::ManualJournalsResponse = self
            .send_request(Method::GET, tenant_id, &path, Some(&query), None::<()>)
            .await?;
        Ok(resp.manual_journals)
    }

    /// Creates or updates a manual journal.
    pub async fn create_or_update_manual_journal(
        &self,
        tenant_id: Uuid,
        journal: manual_journal::ManualJournal,
    ) -> Result<Vec<manual_journal::ManualJournal>, XeroError> {
        let path = if let Some(id) = journal.manual_journal_id {
            format!("/ManualJournals/{}", id)
        } else {
            "/ManualJournals".to_string()
        };
        let method = if journal.manual_journal_id.is_some() {
            Method::POST
        } else {
            Method::PUT
        };
        let resp: manual_journal::ManualJournalsResponse = self
            .send_request(method, tenant_id, &path, None, Some(journal))
            .await?;
        Ok(resp.manual_journals)
    }

    // --- Organisation ---
    /// Retrieves information about the Xero organisation.
    pub async fn get_organisation(
        &self,
        tenant_id: Uuid,
    ) -> Result<Vec<organisation::Organisation>, XeroError> {
        let resp: organisation::OrganisationsResponse = self
            .send_request(Method::GET, tenant_id, "/Organisation", None, None::<()>)
            .await?;
        Ok(resp.organisations)
    }

    /// Retrieves a list of key actions your app has permission to perform.
    pub async fn get_organisation_actions(
        &self,
        tenant_id: Uuid,
    ) -> Result<Vec<organisation::OrganisationAction>, XeroError> {
        let resp: organisation::ActionsResponse = self
            .send_request(
                Method::GET,
                tenant_id,
                "/Organisation/Actions",
                None,
                None::<()>,
            )
            .await?;
        Ok(resp.actions)
    }

    /// Retrieves CIS settings for the organisation (UK only).
    pub async fn get_organisation_cis_settings(
        &self,
        tenant_id: Uuid,
    ) -> Result<organisation::CISSettings, XeroError> {
        let path = "/Organisation/CISSettings";
        self.send_request(Method::GET, tenant_id, path, None, None::<()>)
            .await
    }

    // --- Overpayments ---
    /// Retrieves one or many overpayments.
    pub async fn get_overpayments(
        &self,
        tenant_id: Uuid,
        overpayment_id: Option<Uuid>,
        _modified_after: Option<DateTime<Utc>>,
        where_filter: Option<String>,
        order_by: Option<String>,
        page: Option<u32>,
    ) -> Result<Vec<overpayment::Overpayment>, XeroError> {
        let path = if let Some(id) = overpayment_id {
            format!("/Overpayments/{}", id)
        } else {
            "/Overpayments".to_string()
        };
        let mut query = Vec::new();
        if let Some(filter) = where_filter {
            query.push(("where".to_string(), filter));
        }
        if let Some(order) = order_by {
            query.push(("order".to_string(), order));
        }
        if let Some(p) = page {
            query.push(("page".to_string(), p.to_string()));
        }
        let resp: overpayment::OverpaymentsResponse = self
            .send_request(Method::GET, tenant_id, &path, Some(&query), None::<()>)
            .await?;
        Ok(resp.overpayments)
    }

    /// Allocates an overpayment to an invoice.
    pub async fn allocate_overpayment(
        &self,
        tenant_id: Uuid,
        overpayment_id: Uuid,
        allocation: Allocation,
    ) -> Result<Vec<Allocation>, XeroError> {
        let path = format!("/Overpayments/{}/Allocations", overpayment_id);
        let resp: credit_note::AllocationsResponse = self
            .send_request(Method::PUT, tenant_id, &path, None, Some(allocation))
            .await?;
        Ok(resp.allocations)
    }

    /// Deletes an overpayment allocation.
    pub async fn delete_overpayment_allocation(
        &self,
        tenant_id: Uuid,
        overpayment_id: Uuid,
        allocation_id: Uuid,
    ) -> Result<(), XeroError> {
        let path = format!(
            "/Overpayments/{}/Allocations/{}",
            overpayment_id, allocation_id
        );
        self.send_request_empty_response(Method::DELETE, tenant_id, &path, None::<()>)
            .await
    }

    // --- Payments ---
    /// Retrieves one or many payments.
    #[allow(clippy::too_many_arguments)]
    pub async fn get_payments(
        &self,
        tenant_id: Uuid,
        payment_id: Option<Uuid>,
        _modified_after: Option<DateTime<Utc>>,
        where_filter: Option<String>,
        order_by: Option<String>,
        page: Option<u32>,
        page_size: Option<u32>,
    ) -> Result<Vec<payment::Payment>, XeroError> {
        let path = if let Some(id) = payment_id {
            format!("/Payments/{}", id)
        } else {
            "/Payments".to_string()
        };
        let mut query = Vec::new();
        if let Some(filter) = where_filter {
            query.push(("where".to_string(), filter));
        }
        if let Some(order) = order_by {
            query.push(("order".to_string(), order));
        }
        if let Some(p) = page {
            query.push(("page".to_string(), p.to_string()));
        }
        if let Some(ps) = page_size {
            query.push(("pageSize".to_string(), ps.to_string()));
        }
        let resp: payment::PaymentsResponse = self
            .send_request(Method::GET, tenant_id, &path, Some(&query), None::<()>)
            .await?;
        Ok(resp.payments)
    }

    /// Creates one or more new payments.
    pub async fn create_payments(
        &self,
        tenant_id: Uuid,
        payments: Vec<payment::Payment>,
        summarize_errors: Option<bool>,
    ) -> Result<Vec<payment::Payment>, XeroError> {
        let mut query = Vec::new();
        if let Some(s) = summarize_errors {
            query.push(("summarizeErrors".to_string(), s.to_string()));
        }
        let body = if payments.len() == 1 {
            serde_json::to_value(payments.into_iter().next().unwrap())?
        } else {
            serde_json::to_value(payment::PaymentsRequest { payments })?
        };
        let resp: payment::PaymentsResponse = self
            .send_request(
                Method::PUT,
                tenant_id,
                "/Payments",
                Some(&query),
                Some(body),
            )
            .await?;
        Ok(resp.payments)
    }

    /// Deletes (reverses) a payment.
    pub async fn delete_payment(
        &self,
        tenant_id: Uuid,
        payment_id: Uuid,
    ) -> Result<Vec<payment::Payment>, XeroError> {
        #[derive(Serialize)]
        #[serde(rename_all = "PascalCase")]
        struct DeleteRequest {
            status: payment::PaymentStatus,
        }
        let path = format!("/Payments/{}", payment_id);
        let body = DeleteRequest {
            status: payment::PaymentStatus::Deleted,
        };
        let resp: payment::PaymentsResponse = self
            .send_request(Method::POST, tenant_id, &path, None, Some(body))
            .await?;
        Ok(resp.payments)
    }

    // --- Payment Services ---
    /// Retrieves payment services.
    pub async fn get_payment_services(
        &self,
        tenant_id: Uuid,
    ) -> Result<Vec<payment_service::PaymentService>, XeroError> {
        let resp: payment_service::PaymentServicesResponse = self
            .send_request(Method::GET, tenant_id, "/PaymentServices", None, None::<()>)
            .await?;
        Ok(resp.payment_services)
    }

    /// Creates a new payment service.
    pub async fn create_payment_service(
        &self,
        tenant_id: Uuid,
        service: payment_service::PaymentService,
    ) -> Result<Vec<payment_service::PaymentService>, XeroError> {
        let resp: payment_service::PaymentServicesResponse = self
            .send_request(
                Method::PUT,
                tenant_id,
                "/PaymentServices",
                None,
                Some(service),
            )
            .await?;
        Ok(resp.payment_services)
    }

    // --- Prepayments ---
    /// Retrieves one or many prepayments.
    pub async fn get_prepayments(
        &self,
        tenant_id: Uuid,
        prepayment_id: Option<Uuid>,
        _modified_after: Option<DateTime<Utc>>,
        where_filter: Option<String>,
        order_by: Option<String>,
        page: Option<u32>,
    ) -> Result<Vec<prepayment::Prepayment>, XeroError> {
        let path = if let Some(id) = prepayment_id {
            format!("/Prepayments/{}", id)
        } else {
            "/Prepayments".to_string()
        };
        let mut query = Vec::new();
        if let Some(filter) = where_filter {
            query.push(("where".to_string(), filter));
        }
        if let Some(order) = order_by {
            query.push(("order".to_string(), order));
        }
        if let Some(p) = page {
            query.push(("page".to_string(), p.to_string()));
        }
        let resp: prepayment::PrepaymentsResponse = self
            .send_request(Method::GET, tenant_id, &path, Some(&query), None::<()>)
            .await?;
        Ok(resp.prepayments)
    }

    /// Allocates a prepayment to an invoice.
    pub async fn allocate_prepayment(
        &self,
        tenant_id: Uuid,
        prepayment_id: Uuid,
        allocation: Allocation,
    ) -> Result<Vec<Allocation>, XeroError> {
        let path = format!("/Prepayments/{}/Allocations", prepayment_id);
        let resp: credit_note::AllocationsResponse = self
            .send_request(Method::PUT, tenant_id, &path, None, Some(allocation))
            .await?;
        Ok(resp.allocations)
    }

    /// Deletes a prepayment allocation.
    pub async fn delete_prepayment_allocation(
        &self,
        tenant_id: Uuid,
        prepayment_id: Uuid,
        allocation_id: Uuid,
    ) -> Result<(), XeroError> {
        let path = format!(
            "/Prepayments/{}/Allocations/{}",
            prepayment_id, allocation_id
        );
        self.send_request_empty_response(Method::DELETE, tenant_id, &path, None::<()>)
            .await
    }

    // --- Purchase Orders ---
    /// Retrieves one or many purchase orders.
    #[allow(clippy::too_many_arguments)]
    pub async fn get_purchase_orders(
        &self,
        tenant_id: Uuid,
        purchase_order_id: Option<Uuid>,
        status: Option<String>,
        date_from: Option<String>,
        date_to: Option<String>,
        order_by: Option<String>,
        page: Option<u32>,
        page_size: Option<u32>,
    ) -> Result<Vec<purchase_order::PurchaseOrder>, XeroError> {
        let path = if let Some(id) = purchase_order_id {
            format!("/PurchaseOrders/{}", id)
        } else {
            "/PurchaseOrders".to_string()
        };
        let mut query = Vec::new();
        if let Some(s) = status {
            query.push(("status".to_string(), s));
        }
        if let Some(df) = date_from {
            query.push(("DateFrom".to_string(), df));
        }
        if let Some(dt) = date_to {
            query.push(("DateTo".to_string(), dt));
        }
        if let Some(order) = order_by {
            query.push(("order".to_string(), order));
        }
        if let Some(p) = page {
            query.push(("page".to_string(), p.to_string()));
        }
        if let Some(ps) = page_size {
            query.push(("pageSize".to_string(), ps.to_string()));
        }
        let resp: purchase_order::PurchaseOrdersResponse = self
            .send_request(Method::GET, tenant_id, &path, Some(&query), None::<()>)
            .await?;
        Ok(resp.purchase_orders)
    }

    /// Creates or updates one or more purchase orders.
    pub async fn create_or_update_purchase_orders(
        &self,
        tenant_id: Uuid,
        purchase_orders: Vec<purchase_order::PurchaseOrder>,
        summarize_errors: Option<bool>,
    ) -> Result<Vec<purchase_order::PurchaseOrder>, XeroError> {
        let mut query = Vec::new();
        if let Some(s) = summarize_errors {
            query.push(("summarizeErrors".to_string(), s.to_string()));
        }
        let body = if purchase_orders.len() == 1 {
            serde_json::to_value(purchase_orders.into_iter().next().unwrap())?
        } else {
            serde_json::to_value(purchase_order::PurchaseOrdersRequest { purchase_orders })?
        };
        let resp: purchase_order::PurchaseOrdersResponse = self
            .send_request(
                Method::POST,
                tenant_id,
                "/PurchaseOrders",
                Some(&query),
                Some(body),
            )
            .await?;
        Ok(resp.purchase_orders)
    }

    // --- Quotes ---
    /// Retrieves one or many quotes.
    #[allow(clippy::too_many_arguments)]
    pub async fn get_quotes(
        &self,
        tenant_id: Uuid,
        quote_id: Option<Uuid>,
        quote_number: Option<String>,
        contact_id: Option<Uuid>,
        status: Option<String>,
        date_from: Option<String>,
        date_to: Option<String>,
        expiry_date_from: Option<String>,
        expiry_date_to: Option<String>,
        order_by: Option<String>,
        page: Option<u32>,
        page_size: Option<u32>,
    ) -> Result<Vec<quote::Quote>, XeroError> {
        let path = if let Some(id) = quote_id {
            format!("/Quotes/{}", id)
        } else {
            "/Quotes".to_string()
        };
        let mut query = Vec::new();
        if let Some(qn) = quote_number {
            query.push(("QuoteNumber".to_string(), qn));
        }
        if let Some(cid) = contact_id {
            query.push(("ContactID".to_string(), cid.to_string()));
        }
        if let Some(s) = status {
            query.push(("status".to_string(), s));
        }
        if let Some(df) = date_from {
            query.push(("DateFrom".to_string(), df));
        }
        if let Some(dt) = date_to {
            query.push(("DateTo".to_string(), dt));
        }
        if let Some(edf) = expiry_date_from {
            query.push(("ExpiryDateFrom".to_string(), edf));
        }
        if let Some(edt) = expiry_date_to {
            query.push(("ExpiryDateTo".to_string(), edt));
        }
        if let Some(order) = order_by {
            query.push(("order".to_string(), order));
        }
        if let Some(p) = page {
            query.push(("page".to_string(), p.to_string()));
        }
        if let Some(ps) = page_size {
            query.push(("pageSize".to_string(), ps.to_string()));
        }
        let resp: quote::QuotesResponse = self
            .send_request(Method::GET, tenant_id, &path, Some(&query), None::<()>)
            .await?;
        Ok(resp.quotes)
    }

    /// Creates or updates one or many quotes.
    pub async fn create_or_update_quotes(
        &self,
        tenant_id: Uuid,
        quotes: Vec<quote::Quote>,
    ) -> Result<Vec<quote::Quote>, XeroError> {
        let body = if quotes.len() == 1 {
            serde_json::to_value(quotes.into_iter().next().unwrap())?
        } else {
            serde_json::to_value(quote::QuotesRequest { quotes })?
        };
        let resp: quote::QuotesResponse = self
            .send_request(Method::POST, tenant_id, "/Quotes", None, Some(body))
            .await?;
        Ok(resp.quotes)
    }

    // --- Receipts (Deprecated) ---
    /// Retrieves one or many receipts.
    pub async fn get_receipts(
        &self,
        tenant_id: Uuid,
        receipt_id: Option<Uuid>,
        _modified_after: Option<DateTime<Utc>>,
        where_filter: Option<String>,
        order_by: Option<String>,
    ) -> Result<Vec<receipt::Receipt>, XeroError> {
        let path = if let Some(id) = receipt_id {
            format!("/Receipts/{}", id)
        } else {
            "/Receipts".to_string()
        };
        let mut query = Vec::new();
        if let Some(filter) = where_filter {
            query.push(("where".to_string(), filter));
        }
        if let Some(order) = order_by {
            query.push(("order".to_string(), order));
        }
        let resp: receipt::ReceiptsResponse = self
            .send_request(Method::GET, tenant_id, &path, Some(&query), None::<()>)
            .await?;
        Ok(resp.receipts)
    }

    /// Creates or updates one or many receipts.
    pub async fn create_or_update_receipts(
        &self,
        tenant_id: Uuid,
        receipts: Vec<receipt::Receipt>,
        summarize_errors: Option<bool>,
    ) -> Result<Vec<receipt::Receipt>, XeroError> {
        let mut query = Vec::new();
        if let Some(s) = summarize_errors {
            query.push(("summarizeErrors".to_string(), s.to_string()));
        }
        let body = if receipts.len() == 1 {
            serde_json::to_value(receipts.into_iter().next().unwrap())?
        } else {
            serde_json::to_value(receipt::ReceiptsRequest { receipts })?
        };
        let resp: receipt::ReceiptsResponse = self
            .send_request(
                Method::POST,
                tenant_id,
                "/Receipts",
                Some(&query),
                Some(body),
            )
            .await?;
        Ok(resp.receipts)
    }

    // --- Repeating Invoices ---
    /// Retrieves one or many repeating invoice templates.
    pub async fn get_repeating_invoices(
        &self,
        tenant_id: Uuid,
        repeating_invoice_id: Option<Uuid>,
        where_filter: Option<String>,
        order_by: Option<String>,
    ) -> Result<Vec<repeating_invoice::RepeatingInvoice>, XeroError> {
        let path = if let Some(id) = repeating_invoice_id {
            format!("/RepeatingInvoices/{}", id)
        } else {
            "/RepeatingInvoices".to_string()
        };
        let mut query = Vec::new();
        if let Some(filter) = where_filter {
            query.push(("where".to_string(), filter));
        }
        if let Some(order) = order_by {
            query.push(("order".to_string(), order));
        }
        let resp: repeating_invoice::RepeatingInvoicesResponse = self
            .send_request(Method::GET, tenant_id, &path, Some(&query), None::<()>)
            .await?;
        Ok(resp.repeating_invoices)
    }

    /// Creates or deletes one or more repeating invoice templates.
    pub async fn create_or_delete_repeating_invoices(
        &self,
        tenant_id: Uuid,
        invoices: Vec<repeating_invoice::RepeatingInvoice>,
    ) -> Result<Vec<repeating_invoice::RepeatingInvoice>, XeroError> {
        let body = if invoices.len() == 1 {
            serde_json::to_value(invoices.into_iter().next().unwrap())?
        } else {
            serde_json::to_value(repeating_invoice::RepeatingInvoicesRequest {
                repeating_invoices: invoices,
            })?
        };
        let resp: repeating_invoice::RepeatingInvoicesResponse = self
            .send_request(
                Method::POST,
                tenant_id,
                "/RepeatingInvoices",
                None,
                Some(body),
            )
            .await?;
        Ok(resp.repeating_invoices)
    }

    // --- Reports ---
    /// Retrieves a specific report.
    pub async fn get_report(
        &self,
        tenant_id: Uuid,
        report_name: &str,
        params: Vec<(&str, &str)>,
    ) -> Result<report::Report, XeroError> {
        let path = format!("/Reports/{}", report_name);
        let query: Vec<(String, String)> = params
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        let resp: report::ReportsResponse = self
            .send_request(Method::GET, tenant_id, &path, Some(&query), None::<()>)
            .await?;
        resp.reports
            .into_iter()
            .next()
            .ok_or_else(|| XeroError::Api {
                status: reqwest::StatusCode::NOT_FOUND,
                message: "Report not found in response".to_string(),
            })
    }

    // --- Tax Rates ---
    /// Retrieves tax rates.
    pub async fn get_tax_rates(
        &self,
        tenant_id: Uuid,
        where_filter: Option<String>,
        order_by: Option<String>,
    ) -> Result<Vec<tax_rate::TaxRate>, XeroError> {
        let mut query = Vec::new();
        if let Some(filter) = where_filter {
            query.push(("where".to_string(), filter));
        }
        if let Some(order) = order_by {
            query.push(("order".to_string(), order));
        }
        let resp: tax_rate::TaxRatesResponse = self
            .send_request(
                Method::GET,
                tenant_id,
                "/TaxRates",
                Some(&query),
                None::<()>,
            )
            .await?;
        Ok(resp.tax_rates)
    }

    /// Creates or updates a tax rate.
    pub async fn create_or_update_tax_rate(
        &self,
        tenant_id: Uuid,
        tax_rate: tax_rate::TaxRate,
    ) -> Result<Vec<tax_rate::TaxRate>, XeroError> {
        let resp: tax_rate::TaxRatesResponse = self
            .send_request(Method::POST, tenant_id, "/TaxRates", None, Some(tax_rate))
            .await?;
        Ok(resp.tax_rates)
    }

    // --- Tracking Categories ---
    /// Retrieves tracking categories and their options.
    pub async fn get_tracking_categories(
        &self,
        tenant_id: Uuid,
        tracking_category_id: Option<Uuid>,
        where_filter: Option<String>,
        order_by: Option<String>,
        include_archived: Option<bool>,
    ) -> Result<Vec<tracking_category::TrackingCategory>, XeroError> {
        let path = if let Some(id) = tracking_category_id {
            format!("/TrackingCategories/{}", id)
        } else {
            "/TrackingCategories".to_string()
        };
        let mut query = Vec::new();
        if let Some(filter) = where_filter {
            query.push(("where".to_string(), filter));
        }
        if let Some(order) = order_by {
            query.push(("order".to_string(), order));
        }
        if let Some(ia) = include_archived {
            query.push(("includeArchived".to_string(), ia.to_string()));
        }
        let resp: tracking_category::TrackingCategoriesResponse = self
            .send_request(Method::GET, tenant_id, &path, Some(&query), None::<()>)
            .await?;
        Ok(resp.tracking_categories)
    }

    /// Creates a new tracking category.
    pub async fn create_tracking_category(
        &self,
        tenant_id: Uuid,
        category: tracking_category::TrackingCategory,
    ) -> Result<Vec<tracking_category::TrackingCategory>, XeroError> {
        let resp: tracking_category::TrackingCategoriesResponse = self
            .send_request(
                Method::PUT,
                tenant_id,
                "/TrackingCategories",
                None,
                Some(category),
            )
            .await?;
        Ok(resp.tracking_categories)
    }

    /// Updates a tracking category.
    pub async fn update_tracking_category(
        &self,
        tenant_id: Uuid,
        category_id: Uuid,
        name: String,
    ) -> Result<Vec<tracking_category::TrackingCategory>, XeroError> {
        #[derive(Serialize)]
        struct UpdateRequest {
            #[serde(rename = "Name")]
            name: String,
        }
        let path = format!("/TrackingCategories/{}", category_id);
        let body = UpdateRequest { name };
        let resp: tracking_category::TrackingCategoriesResponse = self
            .send_request(Method::POST, tenant_id, &path, None, Some(body))
            .await?;
        Ok(resp.tracking_categories)
    }

    /// Deletes a tracking category.
    pub async fn delete_tracking_category(
        &self,
        tenant_id: Uuid,
        category_id: Uuid,
    ) -> Result<(), XeroError> {
        let path = format!("/TrackingCategories/{}", category_id);
        self.send_request_empty_response(Method::DELETE, tenant_id, &path, None::<()>)
            .await
    }

    /// Creates a new option for a tracking category.
    pub async fn create_tracking_option(
        &self,
        tenant_id: Uuid,
        category_id: Uuid,
        option: tracking_category::TrackingOption,
    ) -> Result<Vec<tracking_category::TrackingOption>, XeroError> {
        let path = format!("/TrackingCategories/{}/Options", category_id);
        let resp: tracking_category::TrackingOptionsResponse = self
            .send_request(Method::PUT, tenant_id, &path, None, Some(option))
            .await?;
        Ok(resp.options)
    }

    /// Updates a tracking option.
    pub async fn update_tracking_option(
        &self,
        tenant_id: Uuid,
        category_id: Uuid,
        option_id: Uuid,
        name: String,
    ) -> Result<Vec<tracking_category::TrackingOption>, XeroError> {
        #[derive(Serialize)]
        struct UpdateRequest {
            #[serde(rename = "Name")]
            name: String,
        }
        let path = format!("/TrackingCategories/{}/Options/{}", category_id, option_id);
        let body = UpdateRequest { name };
        let resp: tracking_category::TrackingOptionsResponse = self
            .send_request(Method::POST, tenant_id, &path, None, Some(body))
            .await?;
        Ok(resp.options)
    }

    /// Deletes a tracking option.
    pub async fn delete_tracking_option(
        &self,
        tenant_id: Uuid,
        category_id: Uuid,
        option_id: Uuid,
    ) -> Result<(), XeroError> {
        let path = format!("/TrackingCategories/{}/Options/{}", category_id, option_id);
        self.send_request_empty_response(Method::DELETE, tenant_id, &path, None::<()>)
            .await
    }

    // --- Users ---
    /// Retrieves users for the organisation.
    pub async fn get_users(
        &self,
        tenant_id: Uuid,
        user_id: Option<Uuid>,
        _modified_after: Option<DateTime<Utc>>,
        where_filter: Option<String>,
        order_by: Option<String>,
    ) -> Result<Vec<user::User>, XeroError> {
        let path = if let Some(id) = user_id {
            format!("/Users/{}", id)
        } else {
            "/Users".to_string()
        };
        let mut query = Vec::new();
        if let Some(filter) = where_filter {
            query.push(("where".to_string(), filter));
        }
        if let Some(order) = order_by {
            query.push(("order".to_string(), order));
        }
        let resp: user::UsersResponse = self
            .send_request(Method::GET, tenant_id, &path, Some(&query), None::<()>)
            .await?;
        Ok(resp.users)
    }
}
