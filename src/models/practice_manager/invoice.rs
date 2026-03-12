//! Invoice models for the XPM Practice Manager API v3.1.

use serde::Deserialize;
use uuid::Uuid;

use super::common::{ClientRef, ContactRef};

// ---------------------------------------------------------------------------
// Response wrappers
// ---------------------------------------------------------------------------

/// Response for list/current/draft endpoints returning multiple invoices.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "Response")]
pub struct InvoicesResponse {
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Invoices")]
    pub invoices: Option<InvoiceList>,
}

/// Inner wrapper for `<Invoices>`.
#[derive(Debug, Clone, Deserialize)]
pub struct InvoiceList {
    #[serde(rename = "Invoice", default)]
    pub items: Vec<Invoice>,
}

/// `GET invoice.api/get/[invoice number]` — single invoice.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "Response")]
pub struct InvoiceResponse {
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Invoice")]
    pub invoice: Option<Invoice>,
}

/// `GET invoice.api/payments/[invoice number]`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "Response")]
pub struct PaymentsResponse {
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Payments")]
    pub payments: Option<PaymentList>,
}

/// Inner wrapper for `<Payments>`.
#[derive(Debug, Clone, Deserialize)]
pub struct PaymentList {
    #[serde(rename = "Payment", default)]
    pub items: Vec<Payment>,
}

// ---------------------------------------------------------------------------
// Main Invoice struct
// ---------------------------------------------------------------------------

/// An invoice in XPM.
#[derive(Debug, Clone, Deserialize)]
pub struct Invoice {
    /// Human-readable invoice number, e.g. "I000123".
    #[serde(rename = "ID")]
    pub id: Option<String>,
    /// Some responses use `<InternalUUID>`, others `<UUID>`.
    #[serde(rename = "InternalUUID")]
    pub internal_uuid: Option<Uuid>,
    #[serde(rename = "UUID")]
    pub uuid: Option<Uuid>,
    #[serde(rename = "Type")]
    pub invoice_type: Option<String>,
    #[serde(rename = "Status")]
    pub status: Option<String>,
    #[serde(rename = "JobText")]
    pub job_text: Option<String>,
    #[serde(rename = "Date")]
    pub date: Option<String>,
    #[serde(rename = "DueDate")]
    pub due_date: Option<String>,
    #[serde(rename = "Amount")]
    pub amount: Option<String>,
    #[serde(rename = "AmountTax")]
    pub amount_tax: Option<String>,
    #[serde(rename = "AmountIncludingTax")]
    pub amount_including_tax: Option<String>,
    #[serde(rename = "AmountPaid")]
    pub amount_paid: Option<String>,
    #[serde(rename = "AmountOutstanding")]
    pub amount_outstanding: Option<String>,
    #[serde(rename = "Client")]
    pub client: Option<ClientRef>,
    #[serde(rename = "Contact")]
    pub contact: Option<ContactRef>,

    // Detailed invoice — job invoices nest under <Jobs>
    #[serde(rename = "Jobs")]
    pub jobs: Option<InvoiceJobList>,

    // Miscellaneous invoices have tasks/costs at the top level
    #[serde(rename = "Tasks")]
    pub tasks: Option<InvoiceTaskList>,
    #[serde(rename = "Costs")]
    pub costs: Option<InvoiceCostList>,
}

// ---------------------------------------------------------------------------
// Nested list wrappers
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
pub struct InvoiceJobList {
    #[serde(rename = "Job", default)]
    pub items: Vec<InvoiceJob>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InvoiceTaskList {
    #[serde(rename = "Task", default)]
    pub items: Vec<InvoiceTask>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InvoiceCostList {
    #[serde(rename = "Cost", default)]
    pub items: Vec<InvoiceCost>,
}

// ---------------------------------------------------------------------------
// Sub-structs
// ---------------------------------------------------------------------------

/// A job referenced inside an invoice.
#[derive(Debug, Clone, Deserialize)]
pub struct InvoiceJob {
    #[serde(rename = "ID")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "ClientOrderNumber")]
    pub client_order_number: Option<String>,
    #[serde(rename = "Tasks")]
    pub tasks: Option<InvoiceTaskList>,
    #[serde(rename = "Costs")]
    pub costs: Option<InvoiceCostList>,
}

/// A task line item on an invoice.
#[derive(Debug, Clone, Deserialize)]
pub struct InvoiceTask {
    #[serde(rename = "UUID")]
    pub uuid: Option<Uuid>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "Minutes")]
    pub minutes: Option<String>,
    #[serde(rename = "BillableRate")]
    pub billable_rate: Option<String>,
    #[serde(rename = "Billable")]
    pub billable: Option<String>,
    #[serde(rename = "Amount")]
    pub amount: Option<String>,
    #[serde(rename = "AmountTax")]
    pub amount_tax: Option<String>,
    #[serde(rename = "AmountIncludingTax")]
    pub amount_including_tax: Option<String>,
}

/// A cost line item on an invoice.
#[derive(Debug, Clone, Deserialize)]
pub struct InvoiceCost {
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "Note")]
    pub note: Option<String>,
    #[serde(rename = "Code")]
    pub code: Option<String>,
    #[serde(rename = "Billable")]
    pub billable: Option<String>,
    #[serde(rename = "Quantity")]
    pub quantity: Option<String>,
    #[serde(rename = "UnitCost")]
    pub unit_cost: Option<String>,
    #[serde(rename = "UnitPrice")]
    pub unit_price: Option<String>,
    #[serde(rename = "Amount")]
    pub amount: Option<String>,
    #[serde(rename = "AmountTax")]
    pub amount_tax: Option<String>,
    #[serde(rename = "AmountIncludingTax")]
    pub amount_including_tax: Option<String>,
}

/// A payment on an invoice.
#[derive(Debug, Clone, Deserialize)]
pub struct Payment {
    #[serde(rename = "Date")]
    pub date: Option<String>,
    #[serde(rename = "Amount")]
    pub amount: Option<String>,
    #[serde(rename = "Reference")]
    pub reference: Option<String>,
}
