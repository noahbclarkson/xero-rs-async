//! Model for Invoice Reminder settings.

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct InvoiceReminderSettings {
    pub enabled: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct InvoiceReminderSettingsResponse {
    pub invoice_reminders: Vec<InvoiceReminderSettings>,
}
