//! Quote models for the XPM Practice Manager API v3.1.

use serde::Deserialize;

use super::common::{ClientRef, ContactRef};

// ---------------------------------------------------------------------------
// Response wrappers
// ---------------------------------------------------------------------------

/// Response for list/current/draft endpoints returning multiple quotes.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "Response")]
pub struct QuotesResponse {
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Quotes")]
    pub quotes: Option<QuoteList>,
}

/// Inner wrapper for `<Quotes>`.
#[derive(Debug, Clone, Deserialize)]
pub struct QuoteList {
    #[serde(rename = "Quote", default)]
    pub items: Vec<Quote>,
}

/// `GET quote.api/get/[quote number]` — single quote.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "Response")]
pub struct QuoteResponse {
    #[serde(rename = "Status")]
    pub status: String,
    #[serde(rename = "Quote")]
    pub quote: Option<Quote>,
}

// ---------------------------------------------------------------------------
// Main Quote struct
// ---------------------------------------------------------------------------

/// A quote in XPM.
#[derive(Debug, Clone, Deserialize)]
pub struct Quote {
    /// Human-readable quote number, e.g. "Q000123".
    #[serde(rename = "ID")]
    pub id: Option<String>,
    #[serde(rename = "Type")]
    pub quote_type: Option<String>,
    #[serde(rename = "State")]
    pub state: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "Budget")]
    pub budget: Option<String>,
    #[serde(rename = "OptionExplanation")]
    pub option_explanation: Option<String>,
    #[serde(rename = "Date")]
    pub date: Option<String>,
    #[serde(rename = "ValidDate")]
    pub valid_date: Option<String>,
    #[serde(rename = "EstimatedCost")]
    pub estimated_cost: Option<String>,
    #[serde(rename = "EstimatedCostTax")]
    pub estimated_cost_tax: Option<String>,
    #[serde(rename = "EstimatedCostIncludingTax")]
    pub estimated_cost_including_tax: Option<String>,
    #[serde(rename = "Amount")]
    pub amount: Option<String>,
    #[serde(rename = "AmountTax")]
    pub amount_tax: Option<String>,
    #[serde(rename = "AmountIncludingTax")]
    pub amount_including_tax: Option<String>,
    #[serde(rename = "Client")]
    pub client: Option<ClientRef>,
    #[serde(rename = "Contact")]
    pub contact: Option<ContactRef>,

    // Detailed fields
    #[serde(rename = "Tasks")]
    pub tasks: Option<QuoteTaskList>,
    #[serde(rename = "Costs")]
    pub costs: Option<QuoteCostList>,
    #[serde(rename = "Options")]
    pub options: Option<QuoteOptionList>,
}

// ---------------------------------------------------------------------------
// Nested list wrappers
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
pub struct QuoteTaskList {
    #[serde(rename = "Task", default)]
    pub items: Vec<QuoteTask>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct QuoteCostList {
    #[serde(rename = "Cost", default)]
    pub items: Vec<QuoteCost>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct QuoteOptionList {
    #[serde(rename = "Option", default)]
    pub items: Vec<QuoteOption>,
}

// ---------------------------------------------------------------------------
// Sub-structs
// ---------------------------------------------------------------------------

/// A task line item on a quote.
#[derive(Debug, Clone, Deserialize)]
pub struct QuoteTask {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "EstimatedMinutes")]
    pub estimated_minutes: Option<String>,
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

/// A cost line item on a quote.
#[derive(Debug, Clone, Deserialize)]
pub struct QuoteCost {
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

/// An optional cost/add-on on a quote.
#[derive(Debug, Clone, Deserialize)]
pub struct QuoteOption {
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "Note")]
    pub note: Option<String>,
    #[serde(rename = "Code")]
    pub code: Option<String>,
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
