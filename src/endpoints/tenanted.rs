//! Contains convenient API handles that are bound to a specific tenant ID.

use crate::auth::TokenSet;
use crate::client::XeroClient;
use crate::error::XeroError;
use crate::models::accounting::{
    common::Allocation,
    contact::{CISSettings as ContactCISSettings, Contact},
    invoice::{Invoice, OnlineInvoice},
    organisation::{CISSettings as OrgCISSettings, Organisation, OrganisationAction},
    purchase_order::PurchaseOrder,
    quote::Quote,
    repeating_invoice::RepeatingInvoice,
    report::Report,
    tracking_category::{TrackingCategory, TrackingOption},
    *,
};
use crate::models::assets::{asset, asset_type, settings};
use crate::models::files::{association, file, folder};
use chrono::{DateTime, Utc};
use std::sync::Arc;
use uuid::Uuid;

// A macro to reduce boilerplate for the wrapper methods.
macro_rules! tenanted_wrapper {
    (
        $(#[$outer:meta])*
        $vis:vis struct $StructName:ident for $ApiType:ty {
            api_method: $api_method:ident,
            methods: [
                $(
                    $(#[$inner:meta])*
                    fn $method_name:ident (
                        $($param_name:ident: $param_type:ty),*
                    ) -> $return_type:ty;
                ),*
            ]
        }
    ) => {
        $(#[$outer])*
        #[derive(Debug, Clone)]
        $vis struct $StructName {
            client: XeroClient,
            tenant_id: Uuid,
            token_override: Option<Arc<TokenSet>>,
        }

        impl $StructName {
            pub(crate) fn new(client: XeroClient, tenant_id: Uuid) -> Self {
                Self { client, tenant_id, token_override: None }
            }

            pub(crate) fn with_token(client: XeroClient, tenant_id: Uuid, token: TokenSet) -> Self {
                Self { client, tenant_id, token_override: Some(Arc::new(token)) }
            }

            $(
                $(#[$inner])*
                #[allow(clippy::too_many_arguments)]
                pub async fn $method_name(&self, $($param_name: $param_type),*) -> $return_type {
                    let api_handle = self.client.$api_method();
                    let token_aware_handle = if let Some(token) = &self.token_override {
                        api_handle.with_token_override(token.clone())
                    } else {
                        api_handle
                    };
                    token_aware_handle.$method_name(self.tenant_id, $($param_name),*).await
                }
            )*
        }
    };
}

tenanted_wrapper! {
    /// A handle to the Accounting API endpoints, bound to a specific tenant.
    pub struct TenantedAccountingApi for AccountingApi {
        api_method: accounting,
        methods: [
            /// Retrieves the full chart of accounts or a specific account by its ID.
            fn get_accounts(account_id: Option<Uuid>, modified_after: Option<DateTime<Utc>>, where_filter: Option<String>, order_by: Option<String>) -> Result<Vec<account::Account>, XeroError>;,
            /// Creates one or more new accounts.
            fn create_accounts(accounts: Vec<account::Account>) -> Result<Vec<account::Account>, XeroError>;,
            /// Updates an existing account.
            fn update_account(account_id: Uuid, account_data: account::Account) -> Result<Vec<account::Account>, XeroError>;,
            /// Deletes an account.
            fn delete_account(account_id: Uuid) -> Result<(), XeroError>;,
            /// Attaches a file to an account.
            fn create_account_attachment_by_file_name(account_id: Uuid, file_name: String, body: Vec<u8>) -> Result<Vec<attachment::Attachment>, XeroError>;,
            /// Retrieves one or many bank transactions.
            fn get_bank_transactions(bank_transaction_id: Option<Uuid>, modified_after: Option<DateTime<Utc>>, where_filter: Option<String>, order_by: Option<String>, page: Option<u32>, page_size: Option<u32>) -> Result<Vec<bank_transaction::BankTransaction>, XeroError>;,
            /// Creates one or more new spend or receive money transactions.
            fn create_bank_transactions(transactions: Vec<bank_transaction::BankTransaction>, summarize_errors: Option<bool>) -> Result<Vec<bank_transaction::BankTransaction>, XeroError>;,
            /// Updates an existing spend or receive money transaction.
            fn update_bank_transaction(bank_transaction_id: Uuid, transaction_data: bank_transaction::BankTransaction) -> Result<Vec<bank_transaction::BankTransaction>, XeroError>;,
            /// Retrieves one or many bank transfers.
            fn get_bank_transfers(bank_transfer_id: Option<Uuid>, modified_after: Option<DateTime<Utc>>, where_filter: Option<String>, order_by: Option<String>) -> Result<Vec<bank_transfer::BankTransfer>, XeroError>;,
            /// Creates a new bank transfer.
            fn create_bank_transfer(transfer: bank_transfer::BankTransfer) -> Result<Vec<bank_transfer::BankTransfer>, XeroError>;,
            /// Retrieves one or many batch payments.
            fn get_batch_payments(batch_payment_id: Option<Uuid>, modified_after: Option<DateTime<Utc>>, where_filter: Option<String>, order_by: Option<String>) -> Result<Vec<batch_payment::BatchPayment>, XeroError>;,
            /// Creates a new batch payment.
            fn create_batch_payment(batch_payment: batch_payment::BatchPayment) -> Result<Vec<batch_payment::BatchPayment>, XeroError>;,
            /// Updates a batch payment status to DELETED.
            fn delete_batch_payment(batch_payment_id: Uuid) -> Result<Vec<batch_payment::BatchPayment>, XeroError>;,
            /// Retrieves a list of branding themes.
            fn get_branding_themes(branding_theme_id: Option<Uuid>) -> Result<Vec<branding_theme::BrandingTheme>, XeroError>;,
            /// Retrieves payment services for a branding theme.
            fn get_branding_theme_payment_services(branding_theme_id: Uuid) -> Result<Vec<payment_service::PaymentService>, XeroError>;,
            /// Applies a payment service to a branding theme.
            fn create_branding_theme_payment_service(branding_theme_id: Uuid, payment_service_id: Uuid) -> Result<Vec<payment_service::PaymentService>, XeroError>;,
            /// Retrieves one or many budgets.
            fn get_budgets(budget_id: Option<Uuid>, date_to: Option<String>, date_from: Option<String>) -> Result<Vec<budget::Budget>, XeroError>;,
            /// Retrieves one or many contacts.
            fn get_contacts(contact_id: Option<Uuid>, ids: Option<Vec<Uuid>>, modified_after: Option<DateTime<Utc>>, where_filter: Option<String>, order_by: Option<String>, page: Option<u32>, page_size: Option<u32>, include_archived: Option<bool>, summary_only: Option<bool>, search_term: Option<String>) -> Result<Vec<Contact>, XeroError>;,
            /// Creates one or more new contacts.
            fn create_contacts(contacts: Vec<Contact>) -> Result<Vec<Contact>, XeroError>;,
            /// Updates an existing contact.
            fn update_contact(contact_id: Uuid, contact_data: Contact) -> Result<Vec<Contact>, XeroError>;,
            /// Retrieves CIS settings for a contact (UK only).
            fn get_contact_cis_settings(contact_id: Uuid) -> Result<Vec<ContactCISSettings>, XeroError>;,
            /// Retrieves one or all contact groups.
            fn get_contact_groups(contact_group_id: Option<Uuid>, where_filter: Option<String>, order_by: Option<String>) -> Result<Vec<contact_group::ContactGroup>, XeroError>;,
            /// Creates a new contact group.
            fn create_contact_group(contact_group: contact_group::ContactGroup) -> Result<Vec<contact_group::ContactGroup>, XeroError>;,
            /// Updates a contact group.
            fn update_contact_group(contact_group_id: Uuid, contact_group_data: contact_group::ContactGroup) -> Result<Vec<contact_group::ContactGroup>, XeroError>;,
            /// Adds contacts to a contact group.
            fn add_contacts_to_group(contact_group_id: Uuid, contacts: Vec<Contact>) -> Result<Vec<Contact>, XeroError>;,
            /// Removes a specific contact from a contact group.
            fn remove_contact_from_group(contact_group_id: Uuid, contact_id: Uuid) -> Result<(), XeroError>;,
            /// Removes all contacts from a contact group.
            fn remove_all_contacts_from_group(contact_group_id: Uuid) -> Result<(), XeroError>;,
            /// Retrieves one or many credit notes.
            fn get_credit_notes(credit_note_id: Option<Uuid>, modified_after: Option<DateTime<Utc>>, where_filter: Option<String>, order_by: Option<String>, page: Option<u32>, page_size: Option<u32>) -> Result<Vec<credit_note::CreditNote>, XeroError>;,
            /// Creates one or more new credit notes.
            fn create_credit_notes(credit_notes: Vec<credit_note::CreditNote>, summarize_errors: Option<bool>) -> Result<Vec<credit_note::CreditNote>, XeroError>;,
            /// Updates an existing credit note.
            fn update_credit_note(credit_note_id: Uuid, credit_note_data: credit_note::CreditNote) -> Result<Vec<credit_note::CreditNote>, XeroError>;,
            /// Allocates a credit note to an invoice.
            fn allocate_credit_note(credit_note_id: Uuid, allocation: Allocation) -> Result<Vec<Allocation>, XeroError>;,
            /// Deletes a credit note allocation.
            fn delete_credit_note_allocation(credit_note_id: Uuid, allocation_id: Uuid) -> Result<(), XeroError>;,
            /// Retrieves currencies for your organisation.
            fn get_currencies(where_filter: Option<String>, order_by: Option<String>) -> Result<Vec<currency::Currency>, XeroError>;,
            /// Adds a new currency to your organisation.
            fn create_currency(currency: currency::Currency) -> Result<Vec<currency::Currency>, XeroError>;,
            /// Retrieves one or many employees.
            fn get_employees(employee_id: Option<Uuid>, modified_after: Option<DateTime<Utc>>, where_filter: Option<String>, order_by: Option<String>) -> Result<Vec<employee::Employee>, XeroError>;,
            /// Creates one or more new employees.
            fn create_employees(employees: Vec<employee::Employee>) -> Result<Vec<employee::Employee>, XeroError>;,
            /// Updates an existing employee.
            fn update_employee(employee_id: Uuid, employee_data: employee::Employee) -> Result<Vec<employee::Employee>, XeroError>;,
            /// Retrieves one or many expense claims.
            fn get_expense_claims(expense_claim_id: Option<Uuid>, modified_after: Option<DateTime<Utc>>, where_filter: Option<String>, order_by: Option<String>) -> Result<Vec<expense_claim::ExpenseClaim>, XeroError>;,
            /// Creates or updates one or many expense claims.
            fn create_or_update_expense_claims(claims: Vec<expense_claim::ExpenseClaim>, summarize_errors: Option<bool>) -> Result<Vec<expense_claim::ExpenseClaim>, XeroError>;,
            /// Retrieves the history of changes for a specific resource.
            fn get_history(endpoint: &str, guid: Uuid) -> Result<Vec<history::HistoryRecord>, XeroError>;,
            /// Adds a note to the history of a specific resource.
            fn create_history_note(endpoint: &str, guid: Uuid, details: String) -> Result<Vec<history::HistoryRecord>, XeroError>;,
            /// Retrieves one or many invoices.
            fn get_invoices(invoice_id: Option<Uuid>, invoice_numbers: Option<Vec<String>>, contact_ids: Option<Vec<Uuid>>, statuses: Option<Vec<String>>, modified_after: Option<DateTime<Utc>>, where_filter: Option<String>, order_by: Option<String>, page: Option<u32>, page_size: Option<u32>, summary_only: Option<bool>, search_term: Option<String>) -> Result<Vec<Invoice>, XeroError>;,
            /// Creates one or more new invoices.
            fn create_invoices(invoices: Vec<Invoice>, summarize_errors: Option<bool>) -> Result<Vec<Invoice>, XeroError>;,
            /// Updates an existing invoice.
            fn update_invoice(invoice_id: Uuid, invoice_data: Invoice) -> Result<Vec<Invoice>, XeroError>;,
            /// Retrieves the online invoice URL for a sales invoice.
            fn get_online_invoice_url(invoice_id: Uuid) -> Result<OnlineInvoice, XeroError>;,
            /// Emails a sales invoice from Xero.
            fn email_invoice(invoice_id: Uuid) -> Result<(), XeroError>;,
            /// Retrieves one or many items.
            fn get_items(item_id: Option<Uuid>, modified_after: Option<DateTime<Utc>>, where_filter: Option<String>, order_by: Option<String>) -> Result<Vec<item::Item>, XeroError>;,
            /// Creates one or more new items.
            fn create_items(items: Vec<item::Item>) -> Result<Vec<item::Item>, XeroError>;,
            /// Updates an existing item.
            fn update_item(item_id: Uuid, item_data: item::Item) -> Result<Vec<item::Item>, XeroError>;,
            /// Deletes an item.
            fn delete_item(item_id: Uuid) -> Result<(), XeroError>;,
            /// Retrieves journals.
            fn get_journals(offset: Option<u32>, payments_only: Option<bool>) -> Result<Vec<journal::Journal>, XeroError>;,
            /// Retrieves one or many linked transactions.
            fn get_linked_transactions(linked_transaction_id: Option<Uuid>, source_transaction_id: Option<Uuid>, contact_id: Option<Uuid>, status: Option<String>, target_transaction_id: Option<Uuid>, page: Option<u32>) -> Result<Vec<linked_transaction::LinkedTransaction>, XeroError>;,
            /// Creates or updates a linked transaction.
            fn create_or_update_linked_transaction(transaction: linked_transaction::LinkedTransaction) -> Result<Vec<linked_transaction::LinkedTransaction>, XeroError>;,
            /// Deletes a linked transaction.
            fn delete_linked_transaction(linked_transaction_id: Uuid) -> Result<(), XeroError>;,
            /// Retrieves one or many manual journals.
            fn get_manual_journals(manual_journal_id: Option<Uuid>, modified_after: Option<DateTime<Utc>>, where_filter: Option<String>, order_by: Option<String>, page: Option<u32>, page_size: Option<u32>) -> Result<Vec<manual_journal::ManualJournal>, XeroError>;,
            /// Creates or updates a manual journal.
            fn create_or_update_manual_journal(journal: manual_journal::ManualJournal) -> Result<Vec<manual_journal::ManualJournal>, XeroError>;,
            /// Retrieves information about the Xero organisation.
            fn get_organisation() -> Result<Vec<Organisation>, XeroError>;,
            /// Retrieves a list of key actions your app has permission to perform.
            fn get_organisation_actions() -> Result<Vec<OrganisationAction>, XeroError>;,
            /// Retrieves CIS settings for the organisation (UK only).
            fn get_organisation_cis_settings() -> Result<OrgCISSettings, XeroError>;,
            /// Retrieves one or many overpayments.
            fn get_overpayments(overpayment_id: Option<Uuid>, modified_after: Option<DateTime<Utc>>, where_filter: Option<String>, order_by: Option<String>, page: Option<u32>) -> Result<Vec<overpayment::Overpayment>, XeroError>;,
            /// Allocates an overpayment to an invoice.
            fn allocate_overpayment(overpayment_id: Uuid, allocation: Allocation) -> Result<Vec<Allocation>, XeroError>;,
            /// Deletes an overpayment allocation.
            fn delete_overpayment_allocation(overpayment_id: Uuid, allocation_id: Uuid) -> Result<(), XeroError>;,
            /// Retrieves one or many payments.
            fn get_payments(payment_id: Option<Uuid>, modified_after: Option<DateTime<Utc>>, where_filter: Option<String>, order_by: Option<String>, page: Option<u32>, page_size: Option<u32>) -> Result<Vec<payment::Payment>, XeroError>;,
            /// Creates one or more new payments.
            fn create_payments(payments: Vec<payment::Payment>, summarize_errors: Option<bool>) -> Result<Vec<payment::Payment>, XeroError>;,
            /// Deletes (reverses) a payment.
            fn delete_payment(payment_id: Uuid) -> Result<Vec<payment::Payment>, XeroError>;,
            /// Retrieves payment services.
            fn get_payment_services() -> Result<Vec<payment_service::PaymentService>, XeroError>;,
            /// Creates a new payment service.
            fn create_payment_service(service: payment_service::PaymentService) -> Result<Vec<payment_service::PaymentService>, XeroError>;,
            /// Retrieves one or many prepayments.
            fn get_prepayments(prepayment_id: Option<Uuid>, modified_after: Option<DateTime<Utc>>, where_filter: Option<String>, order_by: Option<String>, page: Option<u32>) -> Result<Vec<prepayment::Prepayment>, XeroError>;,
            /// Allocates a prepayment to an invoice.
            fn allocate_prepayment(prepayment_id: Uuid, allocation: Allocation) -> Result<Vec<Allocation>, XeroError>;,
            /// Deletes a prepayment allocation.
            fn delete_prepayment_allocation(prepayment_id: Uuid, allocation_id: Uuid) -> Result<(), XeroError>;,
            /// Retrieves one or many purchase orders.
            fn get_purchase_orders(purchase_order_id: Option<Uuid>, status: Option<String>, date_from: Option<String>, date_to: Option<String>, order_by: Option<String>, page: Option<u32>, page_size: Option<u32>) -> Result<Vec<PurchaseOrder>, XeroError>;,
            /// Creates or updates one or more purchase orders.
            fn create_or_update_purchase_orders(purchase_orders: Vec<PurchaseOrder>, summarize_errors: Option<bool>) -> Result<Vec<PurchaseOrder>, XeroError>;,
            /// Retrieves one or many quotes.
            fn get_quotes(quote_id: Option<Uuid>, quote_number: Option<String>, contact_id: Option<Uuid>, status: Option<String>, date_from: Option<String>, date_to: Option<String>, expiry_date_from: Option<String>, expiry_date_to: Option<String>, order_by: Option<String>, page: Option<u32>, page_size: Option<u32>) -> Result<Vec<Quote>, XeroError>;,
            /// Creates or updates one or many quotes.
            fn create_or_update_quotes(quotes: Vec<Quote>) -> Result<Vec<Quote>, XeroError>;,
            /// Retrieves one or many receipts.
            fn get_receipts(receipt_id: Option<Uuid>, modified_after: Option<DateTime<Utc>>, where_filter: Option<String>, order_by: Option<String>) -> Result<Vec<receipt::Receipt>, XeroError>;,
            /// Creates or updates one or many receipts.
            fn create_or_update_receipts(receipts: Vec<receipt::Receipt>, summarize_errors: Option<bool>) -> Result<Vec<receipt::Receipt>, XeroError>;,
            /// Retrieves one or many repeating invoice templates.
            fn get_repeating_invoices(repeating_invoice_id: Option<Uuid>, where_filter: Option<String>, order_by: Option<String>) -> Result<Vec<RepeatingInvoice>, XeroError>;,
            /// Creates or deletes one or more repeating invoice templates.
            fn create_or_delete_repeating_invoices(invoices: Vec<RepeatingInvoice>) -> Result<Vec<RepeatingInvoice>, XeroError>;,
            /// Retrieves a specific report.
            fn get_report(report_name: &str, params: Vec<(&str, &str)>) -> Result<Report, XeroError>;,
            /// Retrieves tax rates.
            fn get_tax_rates(where_filter: Option<String>, order_by: Option<String>) -> Result<Vec<tax_rate::TaxRate>, XeroError>;,
            /// Creates or updates a tax rate.
            fn create_or_update_tax_rate(tax_rate: tax_rate::TaxRate) -> Result<Vec<tax_rate::TaxRate>, XeroError>;,
            /// Retrieves tracking categories and their options.
            fn get_tracking_categories(tracking_category_id: Option<Uuid>, where_filter: Option<String>, order_by: Option<String>, include_archived: Option<bool>) -> Result<Vec<TrackingCategory>, XeroError>;,
            /// Creates a new tracking category.
            fn create_tracking_category(category: TrackingCategory) -> Result<Vec<TrackingCategory>, XeroError>;,
            /// Updates a tracking category.
            fn update_tracking_category(category_id: Uuid, name: String) -> Result<Vec<TrackingCategory>, XeroError>;,
            /// Deletes a tracking category.
            fn delete_tracking_category(category_id: Uuid) -> Result<(), XeroError>;,
            /// Creates a new option for a tracking category.
            fn create_tracking_option(category_id: Uuid, option: TrackingOption) -> Result<Vec<TrackingOption>, XeroError>;,
            /// Updates a tracking option.
            fn update_tracking_option(category_id: Uuid, option_id: Uuid, name: String) -> Result<Vec<TrackingOption>, XeroError>;,
            /// Deletes a tracking option.
            fn delete_tracking_option(category_id: Uuid, option_id: Uuid) -> Result<(), XeroError>;,
            /// Retrieves users for the organisation.
            fn get_users(user_id: Option<Uuid>, modified_after: Option<DateTime<Utc>>, where_filter: Option<String>, order_by: Option<String>) -> Result<Vec<user::User>, XeroError>;
        ]
    }
}

tenanted_wrapper! {
    /// A handle to the Assets API endpoints, bound to a specific tenant.
    pub struct TenantedAssetsApi for AssetsApi {
        api_method: assets,
        methods: [
            /// Retrieves a list of asset types.
            fn get_asset_types() -> Result<Vec<asset_type::AssetType>, XeroError>;,
            /// Creates a new asset type.
            fn create_asset_type(asset_type: asset_type::AssetType) -> Result<asset_type::AssetType, XeroError>;,
            /// Retrieves a list of assets.
            fn get_assets(status: asset::AssetStatus, page: Option<u32>, page_size: Option<u32>, order_by: Option<String>, sort_direction: Option<String>, filter_by: Option<String>) -> Result<Vec<asset::Asset>, XeroError>;,
            /// Retrieves a single asset by its ID.
            fn get_asset_by_id(asset_id: Uuid) -> Result<asset::Asset, XeroError>;,
            /// Creates a new draft fixed asset.
            fn create_asset(asset: asset::Asset) -> Result<asset::Asset, XeroError>;,
            /// Retrieves the organisation's fixed asset settings.
            fn get_asset_settings() -> Result<settings::Settings, XeroError>;
        ]
    }
}

tenanted_wrapper! {
    /// A handle to the Files API endpoints, bound to a specific tenant.
    pub struct TenantedFilesApi for FilesApi {
        api_method: files,
        methods: [
            /// Retrieves a list of files.
            fn get_files(page_size: Option<u32>, page: Option<u32>, sort: Option<String>, direction: Option<String>) -> Result<Vec<file::File>, XeroError>;,
            /// Retrieves a specific file by its ID.
            fn get_file_by_id(file_id: Uuid) -> Result<file::File, XeroError>;,
            /// Downloads the content of a specific file.
            fn get_file_content(file_id: Uuid) -> Result<Vec<u8>, XeroError>;,
            /// Uploads a file to the inbox.
            fn upload_file(file_name: String, body: Vec<u8>) -> Result<file::File, XeroError>;,
            /// Uploads a file to a specific folder.
            fn upload_file_to_folder(folder_id: Uuid, file_name: String, body: Vec<u8>) -> Result<file::File, XeroError>;,
            /// Updates a file's name or folder.
            fn update_file(file_id: Uuid, new_name: Option<String>, new_folder_id: Option<Uuid>) -> Result<file::File, XeroError>;,
            /// Deletes a file.
            fn delete_file(file_id: Uuid) -> Result<(), XeroError>;,
            /// Retrieves a list of all folders.
            fn get_folders(sort: Option<String>) -> Result<Vec<folder::Folder>, XeroError>;,
            /// Retrieves a specific folder by its ID.
            fn get_folder_by_id(folder_id: Uuid) -> Result<folder::Folder, XeroError>;,
            /// Creates a new folder.
            fn create_folder(name: String) -> Result<folder::Folder, XeroError>;,
            /// Updates a folder's name.
            fn update_folder(folder_id: Uuid, name: String) -> Result<folder::Folder, XeroError>;,
            /// Deletes a folder.
            fn delete_folder(folder_id: Uuid) -> Result<(), XeroError>;,
            /// Retrieves a list of associations for a particular file.
            fn get_file_associations(file_id: Uuid) -> Result<Vec<association::Association>, XeroError>;,
            /// Retrieves a list of associations for a particular object (e.g., an invoice).
            fn get_object_associations(object_id: Uuid) -> Result<Vec<association::Association>, XeroError>;,
            /// Retrieves a count of associations for a list of objects.
            fn get_associations_count(object_ids: Vec<Uuid>) -> Result<association::AssociationCount, XeroError>;,
            /// Creates an association between a file and an object.
            fn create_association(file_id: Uuid, association: association::Association) -> Result<association::Association, XeroError>;,
            /// Deletes an association.
            fn delete_association(file_id: Uuid, association_id: Uuid) -> Result<(), XeroError>;
        ]
    }
}
