use super::super::query::QueryParams;
use super::ReportsResource;
use crate::error::XeroError;
use crate::models::accounting::report;
use chrono::NaiveDate;
use reqwest::Method;
use uuid::Uuid;

impl<'a> ReportsResource<'a> {
    /// Retrieves the Bank Summary report.
    pub async fn bank_summary(
        &self,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
    ) -> Result<report::Report, XeroError> {
        let mut params = QueryParams::default();
        if let Some(from_date) = from_date {
            params.push_string("fromDate", from_date.format("%Y-%m-%d").to_string());
        }
        if let Some(to_date) = to_date {
            params.push_string("toDate", to_date.format("%Y-%m-%d").to_string());
        }
        let params = params.as_slice().unwrap_or_default().to_vec();
        self.get("BankSummary", params).await
    }

    /// Retrieves Aged Receivables by Contact.
    pub async fn aged_receivables_by_contact(
        &self,
        contact_id: Option<Uuid>,
        date: Option<NaiveDate>,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
    ) -> Result<report::Report, XeroError> {
        let mut params = QueryParams::default();
        if let Some(contact_id) = contact_id {
            params.push_string("contactID", contact_id.to_string());
        }
        if let Some(date) = date {
            params.push_string("date", date.format("%Y-%m-%d").to_string());
        }
        if let Some(from_date) = from_date {
            params.push_string("fromDate", from_date.format("%Y-%m-%d").to_string());
        }
        if let Some(to_date) = to_date {
            params.push_string("toDate", to_date.format("%Y-%m-%d").to_string());
        }
        let params = params.as_slice().unwrap_or_default().to_vec();
        self.get("AgedReceivablesByContact", params).await
    }

    /// Retrieves Aged Payables by Contact.
    pub async fn aged_payables_by_contact(
        &self,
        contact_id: Option<Uuid>,
        date: Option<NaiveDate>,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
    ) -> Result<report::Report, XeroError> {
        let mut params = QueryParams::default();
        if let Some(contact_id) = contact_id {
            params.push_string("contactID", contact_id.to_string());
        }
        if let Some(date) = date {
            params.push_string("date", date.format("%Y-%m-%d").to_string());
        }
        if let Some(from_date) = from_date {
            params.push_string("fromDate", from_date.format("%Y-%m-%d").to_string());
        }
        if let Some(to_date) = to_date {
            params.push_string("toDate", to_date.format("%Y-%m-%d").to_string());
        }
        let params = params.as_slice().unwrap_or_default().to_vec();
        self.get("AgedPayablesByContact", params).await
    }

    /// Retrieves the Trial Balance.
    pub async fn trial_balance(
        &self,
        date: Option<NaiveDate>,
        payments_only: Option<bool>,
    ) -> Result<report::Report, XeroError> {
        let mut params = QueryParams::default();
        if let Some(date) = date {
            params.push_string("date", date.format("%Y-%m-%d").to_string());
        }
        params.push_opt("paymentsOnly", payments_only);
        let params = params.as_slice().unwrap_or_default().to_vec();
        self.get("TrialBalance", params).await
    }

    /// Retrieves a list of published GST reports.
    pub async fn published_gst_reports(&self) -> Result<Vec<report::Report>, XeroError> {
        let resp: report::ReportsResponse = self
            .api
            .client
            .send_request(Method::GET, "/Reports", None, None::<()>)
            .await?;
        Ok(resp
            .reports
            .into_iter()
            .filter(|r| r.report_type.as_deref() == Some("GSTReturn"))
            .collect())
    }

    /// Retrieves a specific published GST Report by ID.
    pub async fn gst_report(&self, report_id: &str) -> Result<report::Report, XeroError> {
        self.get(report_id, vec![]).await
    }
}

impl super::super::AccountingApi {
    /// Retrieves a specific report.
    pub async fn get_report(
        &self,
        report_name: &str,
        params: Vec<(&str, &str)>,
    ) -> Result<report::Report, XeroError> {
        let params = params
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        self.reports().get(report_name, params).await
    }

    /// Retrieves the Bank Summary report.
    pub async fn get_bank_summary(
        &self,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
    ) -> Result<report::Report, XeroError> {
        self.reports().bank_summary(from_date, to_date).await
    }

    /// Retrieves Aged Receivables by Contact.
    pub async fn get_aged_receivables_by_contact(
        &self,
        contact_id: Option<Uuid>,
        date: Option<NaiveDate>,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
    ) -> Result<report::Report, XeroError> {
        self.reports()
            .aged_receivables_by_contact(contact_id, date, from_date, to_date)
            .await
    }

    /// Retrieves Aged Payables by Contact.
    pub async fn get_aged_payables_by_contact(
        &self,
        contact_id: Option<Uuid>,
        date: Option<NaiveDate>,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
    ) -> Result<report::Report, XeroError> {
        self.reports()
            .aged_payables_by_contact(contact_id, date, from_date, to_date)
            .await
    }

    /// Retrieves the Trial Balance.
    pub async fn get_trial_balance(
        &self,
        date: Option<NaiveDate>,
        payments_only: Option<bool>,
    ) -> Result<report::Report, XeroError> {
        self.reports().trial_balance(date, payments_only).await
    }

    /// Retrieves a list of published GST reports.
    pub async fn get_published_gst_reports(&self) -> Result<Vec<report::Report>, XeroError> {
        self.reports().published_gst_reports().await
    }

    /// Retrieves a specific published GST Report by ID.
    pub async fn get_gst_report(&self, report_id: &str) -> Result<report::Report, XeroError> {
        self.reports().gst_report(report_id).await
    }
}
