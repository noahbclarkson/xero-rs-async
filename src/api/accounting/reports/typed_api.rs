use crate::error::XeroError;
use crate::models::accounting::report;
use chrono::NaiveDate;
use uuid::Uuid;

impl super::super::AccountingApi {
    /// Retrieves the Bank Summary report (typed).
    pub async fn get_bank_summary_report(
        &self,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
    ) -> Result<report::BankSummaryReport, XeroError> {
        self.reports().bank_summary_report(from_date, to_date).await
    }

    /// Retrieves the Balance Sheet report (typed).
    #[allow(clippy::too_many_arguments)]
    pub async fn get_balance_sheet_report(
        &self,
        date: Option<NaiveDate>,
        periods: Option<u32>,
        timeframe: Option<String>,
        tracking_option_id1: Option<Uuid>,
        tracking_option_id2: Option<Uuid>,
        standard_layout: Option<bool>,
        payments_only: Option<bool>,
    ) -> Result<report::BalanceSheetReport, XeroError> {
        self.reports()
            .balance_sheet_report(
                date,
                periods,
                timeframe,
                tracking_option_id1,
                tracking_option_id2,
                standard_layout,
                payments_only,
            )
            .await
    }

    /// Retrieves Aged Receivables by Contact (typed).
    pub async fn get_aged_receivables_by_contact_report(
        &self,
        contact_id: Option<Uuid>,
        date: Option<NaiveDate>,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
    ) -> Result<report::AgedReceivablesByContactReport, XeroError> {
        self.reports()
            .aged_receivables_by_contact_report(contact_id, date, from_date, to_date)
            .await
    }

    /// Retrieves Aged Payables by Contact (typed).
    pub async fn get_aged_payables_by_contact_report(
        &self,
        contact_id: Option<Uuid>,
        date: Option<NaiveDate>,
        from_date: Option<NaiveDate>,
        to_date: Option<NaiveDate>,
    ) -> Result<report::AgedPayablesByContactReport, XeroError> {
        self.reports()
            .aged_payables_by_contact_report(contact_id, date, from_date, to_date)
            .await
    }

    /// Retrieves the Trial Balance (typed).
    pub async fn get_trial_balance_report(
        &self,
        date: Option<NaiveDate>,
        payments_only: Option<bool>,
    ) -> Result<report::TrialBalanceReport, XeroError> {
        self.reports()
            .trial_balance_report(date, payments_only)
            .await
    }

    /// Retrieves the Profit and Loss report (typed).
    #[allow(clippy::too_many_arguments)]
    pub async fn get_profit_and_loss_report(
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
        self.reports()
            .profit_and_loss_report(
                from_date,
                to_date,
                periods,
                timeframe,
                tracking_category_id,
                tracking_option_id,
                tracking_category_id2,
                tracking_option_id2,
                standard_layout,
                payments_only,
            )
            .await
    }

    /// Retrieves the Budget Summary report (typed).
    pub async fn get_budget_summary_report(
        &self,
        date: Option<NaiveDate>,
        periods: Option<u32>,
        timeframe: Option<u32>,
    ) -> Result<report::BudgetSummaryReport, XeroError> {
        self.reports()
            .budget_summary_report(date, periods, timeframe)
            .await
    }

    /// Retrieves the Executive Summary report (typed).
    pub async fn get_executive_summary_report(
        &self,
        date: Option<NaiveDate>,
    ) -> Result<report::ExecutiveSummaryReport, XeroError> {
        self.reports().executive_summary_report(date).await
    }

    /// Retrieves a list of published BAS reports (typed).
    pub async fn get_published_bas_reports(&self) -> Result<Vec<report::BasReport>, XeroError> {
        self.reports().published_bas_reports().await
    }

    /// Retrieves a specific published BAS report (typed).
    pub async fn get_bas_report(&self, report_id: &str) -> Result<report::BasReport, XeroError> {
        self.reports().bas_report(report_id).await
    }

    /// Retrieves a list of published GST reports (typed).
    ///
    /// Returns New Zealand `GSTReturn` reports only. For AU clients use
    /// [`get_published_bas_reports`] or [`get_published_gst_or_bas_reports`].
    pub async fn get_published_gst_reports_typed(
        &self,
    ) -> Result<Vec<report::GstReport>, XeroError> {
        self.reports().published_gst_reports_typed().await
    }

    /// Retrieves all published GST/BAS reports for any jurisdiction.
    ///
    /// Returns the raw [`report::Report`] structs matching either `GSTReturn` (NZ)
    /// or `SalesTaxReturn` (AU/BAS), combined into a single list.
    pub async fn get_published_gst_or_bas_reports(&self) -> Result<Vec<report::Report>, XeroError> {
        self.reports().published_gst_or_bas_reports().await
    }

    /// Retrieves a specific published GST report (typed).
    ///
    /// Accepts both NZ (`GSTReturn`) and AU (`SalesTaxReturn`) report types.
    ///
    /// **Note:** Returns only summary metadata (empty rows). Use
    /// [`get_gst_return_for_period`] when you need actual row data.
    pub async fn get_gst_report_typed(
        &self,
        report_id: &str,
    ) -> Result<report::GstReport, XeroError> {
        self.reports().gst_report_typed(report_id).await
    }

    /// Fetches a GST/BAS report with full row data by report type and date range.
    ///
    /// This is the correct way to get actual GST return amounts.
    /// `report_type` is `"GSTReturn"` (NZ) or `"SalesTaxReturn"` (AU/BAS).
    pub async fn get_gst_return_for_period(
        &self,
        report_type: &str,
        from_date: chrono::NaiveDate,
        to_date: chrono::NaiveDate,
    ) -> Result<report::GstReport, XeroError> {
        self.reports()
            .gst_return_for_period(report_type, from_date, to_date)
            .await
    }

    /// Retrieves the 1099 report for a given tax year (typed).
    pub async fn get_ten_ninety_nine_report(
        &self,
        report_year: Option<i32>,
    ) -> Result<Vec<report::TenNinetyNineReport>, XeroError> {
        self.reports().ten_ninety_nine_report(report_year).await
    }
}
