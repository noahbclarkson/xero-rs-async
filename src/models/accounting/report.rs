//! Model for the Report resource.

use crate::util::xero_date_format_opt;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Report {
    #[serde(rename = "ReportID", default)]
    pub report_id: Option<String>,
    pub report_name: String,
    #[serde(rename = "ReportType", default)]
    pub report_type: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub report_titles: Vec<String>,
    pub report_date: String,
    #[serde(with = "xero_date_format_opt", default, rename = "UpdatedDateUTC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_date_utc: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rows: Vec<Box<ReportRow>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<ReportField>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attributes: Vec<ReportAttribute>,
    // For 1099 Report
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contacts: Vec<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ReportRow {
    pub row_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cells: Vec<Box<ReportCell>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rows: Vec<Box<ReportRow>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ReportCell {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attributes: Vec<ReportAttribute>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ReportAttribute {
    // Both fields are `#[serde(default)]` because Xero's detailed report
    // endpoint omits `Id` (and occasionally `Value`) on certain cell
    // attributes, which caused a hard deserialization failure for every
    // GST report detail fetch.
    #[serde(default)]
    pub value: String,
    #[serde(rename = "Id", default)]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ReportField {
    #[serde(rename = "FieldID", default)]
    pub field_id: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub value: String,
}

// Wrapper for the response
#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReportsResponse {
    pub reports: Vec<Report>,
}

fn report_type_matches(report: &Report, expected: &str) -> bool {
    report.report_type.as_deref() == Some(expected) || report.report_id.as_deref() == Some(expected)
}

macro_rules! report_wrapper {
    ($name:ident, $expected:expr) => {
        #[derive(Debug, Clone, PartialEq)]
        pub struct $name(pub Report);

        impl $name {
            pub const REPORT_TYPE: &'static str = $expected;

            pub fn try_from_report(report: Report) -> Result<Self, String> {
                if report_type_matches(&report, Self::REPORT_TYPE) {
                    Ok(Self(report))
                } else {
                    Err(format!(
                        "Expected report type '{}' but received {:?}",
                        Self::REPORT_TYPE,
                        report.report_type
                    ))
                }
            }

            pub fn inner(&self) -> &Report {
                &self.0
            }
        }

        impl From<$name> for Report {
            fn from(value: $name) -> Self {
                value.0
            }
        }
    };
}

report_wrapper!(AgedPayablesByContactReport, "AgedPayablesByContact");
report_wrapper!(AgedReceivablesByContactReport, "AgedReceivablesByContact");
report_wrapper!(BalanceSheetReport, "BalanceSheet");
report_wrapper!(BankSummaryReport, "BankSummary");
report_wrapper!(BasReport, "SalesTaxReturn");
report_wrapper!(BudgetSummaryReport, "BudgetSummary");
report_wrapper!(ExecutiveSummaryReport, "ExecutiveSummary");
report_wrapper!(GstReport, "GSTReturn");
report_wrapper!(ProfitAndLossReport, "ProfitAndLoss");
report_wrapper!(TrialBalanceReport, "TrialBalance");

#[derive(Debug, Clone, PartialEq)]
pub struct TenNinetyNineReport(pub Report);

impl TenNinetyNineReport {
    pub fn try_from_report(report: Report) -> Result<Self, String> {
        if report.report_name.contains("1099") {
            Ok(Self(report))
        } else {
            Err(format!(
                "Expected a 1099 report but received '{}'.",
                report.report_name
            ))
        }
    }

    pub fn inner(&self) -> &Report {
        &self.0
    }
}

impl From<TenNinetyNineReport> for Report {
    fn from(value: TenNinetyNineReport) -> Self {
        value.0
    }
}
