use super::super::query::QueryParams;
use super::ReportsResource;
use crate::error::XeroError;
use crate::models::accounting::report;
use chrono::NaiveDate;
use reqwest::StatusCode;
use uuid::Uuid;

impl<'a> ReportsResource<'a> {
    /// Retrieves the Bank Summary report (typed).
    pub async fn bank_summary_report(
        &self,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
    ) -> Result<report::BankSummaryReport, XeroError> {
        let report = self.bank_summary(from_date, to_date).await?;
        report::BankSummaryReport::try_from_report(report).map_err(report_type_error)
    }

    /// Retrieves the Balance Sheet report (typed).
    pub async fn balance_sheet_report(
        &self,
        date: Option<NaiveDate>,
        periods: Option<u32>,
        timeframe: Option<String>,
        tracking_option_id1: Option<Uuid>,
        tracking_option_id2: Option<Uuid>,
        standard_layout: Option<bool>,
        payments_only: Option<bool>,
    ) -> Result<report::BalanceSheetReport, XeroError> {
        let mut params = QueryParams::default();
        if let Some(date) = date {
            params.push_string("date", date.format("%Y-%m-%d").to_string());
        }
        params.push_opt("periods", periods);
        params.push_opt_string("timeframe", timeframe);
        if let Some(id) = tracking_option_id1 {
            params.push_string("trackingOptionID1", id.to_string());
        }
        if let Some(id) = tracking_option_id2 {
            params.push_string("trackingOptionID2", id.to_string());
        }
        params.push_opt("standardLayout", standard_layout);
        params.push_opt("paymentsOnly", payments_only);
        let params = params.as_slice().unwrap_or_default().to_vec();
        let report = self.get("BalanceSheet", params).await?;
        report::BalanceSheetReport::try_from_report(report).map_err(report_type_error)
    }

    /// Retrieves Aged Receivables by Contact (typed).
    pub async fn aged_receivables_by_contact_report(
        &self,
        contact_id: Option<Uuid>,
        date: Option<NaiveDate>,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
    ) -> Result<report::AgedReceivablesByContactReport, XeroError> {
        let report = self
            .aged_receivables_by_contact(contact_id, date, from_date, to_date)
            .await?;
        report::AgedReceivablesByContactReport::try_from_report(report).map_err(report_type_error)
    }

    /// Retrieves Aged Payables by Contact (typed).
    pub async fn aged_payables_by_contact_report(
        &self,
        contact_id: Option<Uuid>,
        date: Option<NaiveDate>,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
    ) -> Result<report::AgedPayablesByContactReport, XeroError> {
        let report = self
            .aged_payables_by_contact(contact_id, date, from_date, to_date)
            .await?;
        report::AgedPayablesByContactReport::try_from_report(report).map_err(report_type_error)
    }

    /// Retrieves the Trial Balance (typed).
    pub async fn trial_balance_report(
        &self,
        date: Option<NaiveDate>,
        payments_only: Option<bool>,
    ) -> Result<report::TrialBalanceReport, XeroError> {
        let report = self.trial_balance(date, payments_only).await?;
        report::TrialBalanceReport::try_from_report(report).map_err(report_type_error)
    }

    /// Retrieves the Profit and Loss report (typed).
    #[allow(clippy::too_many_arguments)]
    pub async fn profit_and_loss_report(
        &self,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
        periods: Option<u32>,
        timeframe: Option<String>,
        tracking_category_id: Option<Uuid>,
        tracking_option_id: Option<Uuid>,
        tracking_category_id2: Option<Uuid>,
        tracking_option_id2: Option<Uuid>,
        standard_layout: Option<bool>,
        payments_only: Option<bool>,
    ) -> Result<report::ProfitAndLossReport, XeroError> {
        let mut params = QueryParams::default();
        if let Some(from_date) = from_date {
            params.push_string("fromDate", from_date.format("%Y-%m-%d").to_string());
        }
        if let Some(to_date) = to_date {
            params.push_string("toDate", to_date.format("%Y-%m-%d").to_string());
        }
        params.push_opt("periods", periods);
        params.push_opt_string("timeframe", timeframe);
        if let Some(id) = tracking_category_id {
            params.push_string("trackingCategoryID", id.to_string());
        }
        if let Some(id) = tracking_option_id {
            params.push_string("trackingOptionID", id.to_string());
        }
        if let Some(id) = tracking_category_id2 {
            params.push_string("trackingCategoryID2", id.to_string());
        }
        if let Some(id) = tracking_option_id2 {
            params.push_string("trackingOptionID2", id.to_string());
        }
        params.push_opt("standardLayout", standard_layout);
        params.push_opt("paymentsOnly", payments_only);
        let params = params.as_slice().unwrap_or_default().to_vec();
        let report = self.get("ProfitAndLoss", params).await?;
        report::ProfitAndLossReport::try_from_report(report).map_err(report_type_error)
    }

    /// Retrieves the Budget Summary report (typed).
    pub async fn budget_summary_report(
        &self,
        date: Option<NaiveDate>,
        periods: Option<u32>,
        timeframe: Option<u32>,
    ) -> Result<report::BudgetSummaryReport, XeroError> {
        let mut params = QueryParams::default();
        if let Some(date) = date {
            params.push_string("date", date.format("%Y-%m-%d").to_string());
        }
        params.push_opt("periods", periods);
        params.push_opt("timeframe", timeframe);
        let params = params.as_slice().unwrap_or_default().to_vec();
        let report = self.get("BudgetSummary", params).await?;
        report::BudgetSummaryReport::try_from_report(report).map_err(report_type_error)
    }

    /// Retrieves the Executive Summary report (typed).
    pub async fn executive_summary_report(
        &self,
        date: Option<NaiveDate>,
    ) -> Result<report::ExecutiveSummaryReport, XeroError> {
        let mut params = QueryParams::default();
        if let Some(date) = date {
            params.push_string("date", date.format("%Y-%m-%d").to_string());
        }
        let params = params.as_slice().unwrap_or_default().to_vec();
        let report = self.get("ExecutiveSummary", params).await?;
        report::ExecutiveSummaryReport::try_from_report(report).map_err(report_type_error)
    }

    /// Retrieves a list of published BAS reports (typed).
    pub async fn published_bas_reports(&self) -> Result<Vec<report::BasReport>, XeroError> {
        let resp: report::ReportsResponse = self
            .api
            .client
            .send_request(reqwest::Method::GET, "/Reports", None, None::<()>)
            .await?;
        resp.reports
            .into_iter()
            .filter(|r| r.report_type.as_deref() == Some("SalesTaxReturn"))
            .map(report::BasReport::try_from_report)
            .map(|r| r.map_err(report_type_error))
            .collect()
    }

    /// Retrieves a specific published BAS Report by ID (typed).
    pub async fn bas_report(&self, report_id: &str) -> Result<report::BasReport, XeroError> {
        let report = self.get(report_id, vec![]).await?;
        report::BasReport::try_from_report(report).map_err(report_type_error)
    }

    /// Retrieves a list of published GST reports (typed).
    ///
    /// This returns New Zealand `GSTReturn` reports only. For Australian BAS reports
    /// (`SalesTaxReturn`), use [`published_bas_reports`] or
    /// [`published_gst_or_bas_reports`].
    pub async fn published_gst_reports_typed(&self) -> Result<Vec<report::GstReport>, XeroError> {
        let resp: report::ReportsResponse = self
            .api
            .client
            .send_request(reqwest::Method::GET, "/Reports", None, None::<()>)
            .await?;
        resp.reports
            .into_iter()
            .filter(|r| r.report_type.as_deref() == Some("GSTReturn"))
            .map(report::GstReport::try_from_report)
            .map(|r| r.map_err(report_type_error))
            .collect()
    }

    /// Retrieves all published GST/BAS reports regardless of jurisdiction.
    ///
    /// Returns a unified list of the underlying [`report::Report`] structs (not the
    /// typed wrappers) so callers can inspect both NZ (`GSTReturn`) and AU
    /// (`SalesTaxReturn`) returns without needing two separate calls.
    pub async fn published_gst_or_bas_reports(&self) -> Result<Vec<report::Report>, XeroError> {
        let resp: report::ReportsResponse = self
            .api
            .client
            .send_request(reqwest::Method::GET, "/Reports", None, None::<()>)
            .await?;
        Ok(resp
            .reports
            .into_iter()
            .filter(|r| {
                matches!(
                    r.report_type.as_deref(),
                    Some("GSTReturn") | Some("SalesTaxReturn")
                )
            })
            .collect())
    }

    /// Retrieves a specific published GST Report by ID (typed).
    ///
    /// Works for both NZ (`GSTReturn`) and AU (`SalesTaxReturn`) reports. When the
    /// report belongs to an Australian organisation, Xero returns type
    /// `"SalesTaxReturn"` rather than `"GSTReturn"`, so this method relaxes the
    /// type check to accept either.
    ///
    /// **Note:** `GET /Reports/{numericId}` returns only summary metadata with empty
    /// rows. Prefer [`gst_return_for_period`] when you need actual row data.
    pub async fn gst_report_typed(&self, report_id: &str) -> Result<report::GstReport, XeroError> {
        let report = self.get(report_id, vec![]).await?;
        // Accept both NZ GSTReturn and AU SalesTaxReturn — both contain the same
        // GST/BAS net balance data we need for reconciliation.
        if report.report_type.as_deref() == Some("SalesTaxReturn") {
            // Re-wrap as GstReport by relaxing the type check.
            // The underlying Report data structure is identical for both types.
            Ok(report::GstReport(report))
        } else {
            report::GstReport::try_from_report(report).map_err(report_type_error)
        }
    }

    /// Fetches a GST/BAS report by type and date range, returning full row data.
    ///
    /// This is the correct way to get GST return amounts. Calling
    /// `GET /Reports/{numericId}` only returns metadata (empty rows); you must call
    /// `GET /Reports/GSTReturn?fromDate=X&toDate=Y` to retrieve the actual data.
    ///
    /// `report_type` should be `"GSTReturn"` (NZ) or `"SalesTaxReturn"` (AU).
    pub async fn gst_return_for_period(
        &self,
        report_type: &str,
        from_date: NaiveDate,
        to_date: NaiveDate,
    ) -> Result<report::GstReport, XeroError> {
        let mut params = QueryParams::default();
        params.push_string("fromDate", from_date.format("%Y-%m-%d").to_string());
        params.push_string("toDate", to_date.format("%Y-%m-%d").to_string());
        let params = params.as_slice().unwrap_or_default().to_vec();
        let report = self.get(report_type, params).await?;
        if report.report_type.as_deref() == Some("SalesTaxReturn") {
            Ok(report::GstReport(report))
        } else {
            report::GstReport::try_from_report(report).map_err(report_type_error)
        }
    }

    /// Retrieves the 1099 report for a given tax year (typed).
    pub async fn ten_ninety_nine_report(
        &self,
        report_year: Option<i32>,
    ) -> Result<Vec<report::TenNinetyNineReport>, XeroError> {
        let mut params = QueryParams::default();
        if let Some(year) = report_year {
            params.push_string("reportYear", year.to_string());
        }
        let resp: report::ReportsResponse = self
            .api
            .client
            .send_request(
                reqwest::Method::GET,
                "/Reports/TenNinetyNine",
                params.as_slice(),
                None::<()>,
            )
            .await?;
        resp.reports
            .into_iter()
            .map(report::TenNinetyNineReport::try_from_report)
            .map(|r| r.map_err(report_type_error))
            .collect()
    }
}

fn report_type_error(message: String) -> XeroError {
    XeroError::Api {
        status: StatusCode::UNPROCESSABLE_ENTITY,
        message,
    }
}
