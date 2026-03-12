//! Entry point for interacting with the Xero Accounting API.

use crate::auth::TokenSet;
use crate::client::XeroClient;
use crate::http::ApiClient;
use std::sync::Arc;
use uuid::Uuid;

pub mod accounts;
pub mod attachments;
pub mod bank_transactions;
pub mod bank_transfers;
pub mod batch_payments;
pub mod branding_themes;
pub mod budgets;
pub mod contact_groups;
pub mod contacts;
pub mod credit_notes;
pub mod currencies;
pub mod employees;
pub mod expense_claims;
pub mod history;
pub mod invoice_reminders;
pub mod invoices;
pub mod items;
pub mod journals;
pub mod linked_transactions;
pub mod manual_journals;
pub mod organisation;
pub mod overpayments;
pub mod payment_services;
pub mod payments;
pub mod prepayments;
pub mod purchase_orders;
pub mod quotes;
pub mod receipts;
pub mod repeating_invoices;
pub mod reports;
pub mod tax_rates;
pub mod tracking_categories;
pub mod users;

mod query;

const BASE_URL: &str = "https://api.xero.com/api.xro/2.0";

/// A handle to the Accounting API endpoints.
#[derive(Debug, Clone)]
pub struct AccountingApi {
    client: ApiClient,
}

impl AccountingApi {
    pub(crate) fn new(client: XeroClient, tenant_id: Uuid) -> Self {
        Self {
            client: ApiClient::new(
                BASE_URL,
                tenant_id,
                client.http_client.clone(),
                client.token_manager.clone(),
                client.rate_limiter.clone(),
            ),
        }
    }

    pub(crate) fn with_token_override(mut self, token: Arc<TokenSet>) -> Self {
        self.client = self.client.with_token_override(token);
        self
    }

    /// Access Accounts endpoints.
    #[must_use]
    pub fn accounts(&self) -> accounts::AccountsResource<'_> {
        accounts::AccountsResource::new(self)
    }

    /// Access Attachments endpoints.
    #[must_use]
    pub fn attachments(&self) -> attachments::AttachmentsResource<'_> {
        attachments::AttachmentsResource::new(self)
    }

    /// Access Bank Transactions endpoints.
    #[must_use]
    pub fn bank_transactions(&self) -> bank_transactions::BankTransactionsResource<'_> {
        bank_transactions::BankTransactionsResource::new(self)
    }

    /// Access Bank Transfers endpoints.
    #[must_use]
    pub fn bank_transfers(&self) -> bank_transfers::BankTransfersResource<'_> {
        bank_transfers::BankTransfersResource::new(self)
    }

    /// Access Batch Payments endpoints.
    #[must_use]
    pub fn batch_payments(&self) -> batch_payments::BatchPaymentsResource<'_> {
        batch_payments::BatchPaymentsResource::new(self)
    }

    /// Access Branding Themes endpoints.
    #[must_use]
    pub fn branding_themes(&self) -> branding_themes::BrandingThemesResource<'_> {
        branding_themes::BrandingThemesResource::new(self)
    }

    /// Access Budgets endpoints.
    #[must_use]
    pub fn budgets(&self) -> budgets::BudgetsResource<'_> {
        budgets::BudgetsResource::new(self)
    }

    /// Access Contacts endpoints.
    #[must_use]
    pub fn contacts(&self) -> contacts::ContactsResource<'_> {
        contacts::ContactsResource::new(self)
    }

    /// Access Contact Groups endpoints.
    #[must_use]
    pub fn contact_groups(&self) -> contact_groups::ContactGroupsResource<'_> {
        contact_groups::ContactGroupsResource::new(self)
    }

    /// Access Credit Notes endpoints.
    #[must_use]
    pub fn credit_notes(&self) -> credit_notes::CreditNotesResource<'_> {
        credit_notes::CreditNotesResource::new(self)
    }

    /// Access Currencies endpoints.
    #[must_use]
    pub fn currencies(&self) -> currencies::CurrenciesResource<'_> {
        currencies::CurrenciesResource::new(self)
    }

    /// Access Employees endpoints.
    #[must_use]
    pub fn employees(&self) -> employees::EmployeesResource<'_> {
        employees::EmployeesResource::new(self)
    }

    /// Access Expense Claims endpoints.
    #[must_use]
    pub fn expense_claims(&self) -> expense_claims::ExpenseClaimsResource<'_> {
        expense_claims::ExpenseClaimsResource::new(self)
    }

    /// Access History endpoints.
    #[must_use]
    pub fn history(&self) -> history::HistoryResource<'_> {
        history::HistoryResource::new(self)
    }

    /// Access Invoices endpoints.
    #[must_use]
    pub fn invoices(&self) -> invoices::InvoicesResource<'_> {
        invoices::InvoicesResource::new(self)
    }

    /// Access Invoice Reminders endpoints.
    #[must_use]
    pub fn invoice_reminders(&self) -> invoice_reminders::InvoiceRemindersResource<'_> {
        invoice_reminders::InvoiceRemindersResource::new(self)
    }

    /// Access Items endpoints.
    #[must_use]
    pub fn items(&self) -> items::ItemsResource<'_> {
        items::ItemsResource::new(self)
    }

    /// Access Journals endpoints.
    #[must_use]
    pub fn journals(&self) -> journals::JournalsResource<'_> {
        journals::JournalsResource::new(self)
    }

    /// Access Linked Transactions endpoints.
    #[must_use]
    pub fn linked_transactions(&self) -> linked_transactions::LinkedTransactionsResource<'_> {
        linked_transactions::LinkedTransactionsResource::new(self)
    }

    /// Access Manual Journals endpoints.
    #[must_use]
    pub fn manual_journals(&self) -> manual_journals::ManualJournalsResource<'_> {
        manual_journals::ManualJournalsResource::new(self)
    }

    /// Access Organisation endpoints.
    #[must_use]
    pub fn organisation(&self) -> organisation::OrganisationResource<'_> {
        organisation::OrganisationResource::new(self)
    }

    /// Access Overpayments endpoints.
    #[must_use]
    pub fn overpayments(&self) -> overpayments::OverpaymentsResource<'_> {
        overpayments::OverpaymentsResource::new(self)
    }

    /// Access Payments endpoints.
    #[must_use]
    pub fn payments(&self) -> payments::PaymentsResource<'_> {
        payments::PaymentsResource::new(self)
    }

    /// Access Payment Services endpoints.
    #[must_use]
    pub fn payment_services(&self) -> payment_services::PaymentServicesResource<'_> {
        payment_services::PaymentServicesResource::new(self)
    }

    /// Access Prepayments endpoints.
    #[must_use]
    pub fn prepayments(&self) -> prepayments::PrepaymentsResource<'_> {
        prepayments::PrepaymentsResource::new(self)
    }

    /// Access Purchase Orders endpoints.
    #[must_use]
    pub fn purchase_orders(&self) -> purchase_orders::PurchaseOrdersResource<'_> {
        purchase_orders::PurchaseOrdersResource::new(self)
    }

    /// Access Quotes endpoints.
    #[must_use]
    pub fn quotes(&self) -> quotes::QuotesResource<'_> {
        quotes::QuotesResource::new(self)
    }

    /// Access Receipts endpoints.
    #[must_use]
    pub fn receipts(&self) -> receipts::ReceiptsResource<'_> {
        receipts::ReceiptsResource::new(self)
    }

    /// Access Repeating Invoices endpoints.
    #[must_use]
    pub fn repeating_invoices(&self) -> repeating_invoices::RepeatingInvoicesResource<'_> {
        repeating_invoices::RepeatingInvoicesResource::new(self)
    }

    /// Access Reports endpoints.
    #[must_use]
    pub fn reports(&self) -> reports::ReportsResource<'_> {
        reports::ReportsResource::new(self)
    }

    /// Access Tax Rates endpoints.
    #[must_use]
    pub fn tax_rates(&self) -> tax_rates::TaxRatesResource<'_> {
        tax_rates::TaxRatesResource::new(self)
    }

    /// Access Tracking Categories endpoints.
    #[must_use]
    pub fn tracking_categories(&self) -> tracking_categories::TrackingCategoriesResource<'_> {
        tracking_categories::TrackingCategoriesResource::new(self)
    }

    /// Access Users endpoints.
    #[must_use]
    pub fn users(&self) -> users::UsersResource<'_> {
        users::UsersResource::new(self)
    }
}
