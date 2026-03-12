use super::AccountingApi;
use crate::error::XeroError;
use crate::models::accounting::invoice_reminder;
use reqwest::Method;

/// Resource accessor for Invoice Reminder settings.
#[derive(Debug, Clone, Copy)]
pub struct InvoiceRemindersResource<'a> {
    api: &'a AccountingApi,
}

impl<'a> InvoiceRemindersResource<'a> {
    pub(crate) fn new(api: &'a AccountingApi) -> Self {
        Self { api }
    }

    /// Retrieves invoice reminder settings.
    pub async fn settings(
        &self,
    ) -> Result<Vec<invoice_reminder::InvoiceReminderSettings>, XeroError> {
        let resp: invoice_reminder::InvoiceReminderSettingsResponse = self
            .api
            .client
            .send_request(Method::GET, "/InvoiceReminders/Settings", None, None::<()>)
            .await?;
        Ok(resp.invoice_reminders)
    }
}

impl AccountingApi {
    /// Retrieves invoice reminder settings.
    pub async fn get_invoice_reminder_settings(
        &self,
    ) -> Result<Vec<invoice_reminder::InvoiceReminderSettings>, XeroError> {
        self.invoice_reminders().settings().await
    }
}
