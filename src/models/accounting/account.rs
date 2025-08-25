//! Model for the Account resource.

use crate::util::xero_date_format_opt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
#[derive(Default)]
pub enum AccountType {
    #[default]
    Bank,
    Current,
    Currliab,
    Depreciatn,
    Directcosts,
    Equity,
    Expense,
    Fixed,
    Inventory,
    Liability,
    Noncurrent,
    Otherincome,
    Overheads,
    Prepayment,
    Revenue,
    Sales,
    Termliab,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum AccountStatus {
    Active,
    Archived,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum BankAccountType {
    Bank,
    Creditcard,
    Paypal,
    Unknown(String),
}

impl<'de> serde::Deserialize<'de> for BankAccountType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = BankAccountType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string, a map, or null for BankAccountType")
            }

            // This method handles string values like "BANK"
            fn visit_str<E>(self, value: &str) -> Result<BankAccountType, E>
            where
                E: serde::de::Error,
            {
                match value.to_ascii_uppercase().as_str() {
                    "BANK" => Ok(BankAccountType::Bank),
                    "CREDITCARD" => Ok(BankAccountType::Creditcard),
                    "PAYPAL" => Ok(BankAccountType::Paypal),
                    "" => Ok(BankAccountType::Unknown(String::new())),
                    other => Ok(BankAccountType::Unknown(other.to_string())),
                }
            }

            // This NEW method handles the empty JSON object `{}`
            fn visit_map<M>(self, mut map: M) -> Result<BankAccountType, M::Error>
            where
                M: serde::de::MapAccess<'de>,
            {
                // The API is sending an empty map for non-bank accounts.
                // We consume it and treat it as "Unknown".
                while let Some((_key, _value)) = map.next_entry::<serde_json::Value, serde_json::Value>()? {
                    // Do nothing, just consume the entries if any
                }
                Ok(BankAccountType::Unknown(String::new()))
            }

            // This NEW method handles JSON `null` values
            fn visit_unit<E>(self) -> Result<BankAccountType, E>
            where
                E: serde::de::Error,
            {
                Ok(BankAccountType::Unknown(String::new()))
            }
        }

        // Use the more flexible deserializer
        deserializer.deserialize_any(Visitor)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum AccountClass {
    Asset,
    Equity,
    Expense,
    Liability,
    Revenue,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Account {
    #[serde(rename = "AccountID", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    // Name is required for creation, but optional in summary views
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub account_type: Option<AccountType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AccountStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_account_type: Option<BankAccountType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_payments_to_account: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_in_expense_claims: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<AccountClass>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporting_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporting_code_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_attachments: Option<bool>,
    #[serde(with = "xero_date_format_opt", default, rename = "UpdatedDateUTC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_date_utc: Option<DateTime<Utc>>,
    #[serde(with = "xero_date_format_opt", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reporting_code_updated_utc: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_to_watchlist: Option<bool>,
}

// Wrapper for the response
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct AccountsResponse {
    pub accounts: Vec<Account>,
}

// Wrapper for the request
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct AccountsRequest {
    pub accounts: Vec<Account>,
}
