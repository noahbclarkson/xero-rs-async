use serde_json::from_str;
use xero_rs_async::models::accounting::report::ReportsResponse;

#[test]
fn test_deserialize_gst_report() {
    let json = r#"
    {
      "Reports": [
        {
          "ReportID": "86894",
          "ReportName": "GST and Provisional Tax Return",
          "ReportType": "GSTReturn",
          "ReportDate": "01 Jul 2016 to 31 Dec 2016",
          "UpdatedDateUTC": "/Date(1519596630000)/",
          "Fields": [
            {
              "FieldID": "1",
              "Description": "Registration number",
              "Value": "111-111-111"
            },
            {
              "FieldID": "GSTBasis",
              "Description": "GST basis",
              "Value": "Invoice Basis"
            },
            {
              "FieldID": "5",
              "Description": "Total sales and income for the period (including GST and zero-rated Supplies)",
              "Value": "115.00"
            }
          ]
        }
      ]
    }
    "#;

    let response: ReportsResponse = from_str(json).expect("Should deserialize GST report");
    let report = &response.reports[0];

    assert_eq!(report.report_id.as_deref(), Some("86894"));
    assert_eq!(report.report_type.as_deref(), Some("GSTReturn"));
    assert!(!report.fields.is_empty());

    let gst_basis = report
        .fields
        .iter()
        .find(|f| f.field_id == "GSTBasis")
        .unwrap();
    assert_eq!(gst_basis.value, "Invoice Basis");
}

#[test]
fn test_deserialize_aged_payables() {
    let json = r#"
    {
      "Reports": [
        {
          "ReportID": "AgedPayablesByContact",
          "ReportName": "Aged Payables By Contact",
          "ReportType": "AgedPayablesByContact",
          "ReportTitles": [
            "Invoices",
            "Xero",
            "To 28 February 2018",
            "Showing payments to 28 February 2018"
          ],
          "ReportDate": "23 February 2018",
          "UpdatedDateUTC": "/Date(1519357171249)/",
          "Rows": [
            {
              "RowType": "Header",
              "Cells": [
                { "Value": "Date" },
                { "Value": "Reference" }
              ]
            },
            {
              "RowType": "Section",
              "Rows": [
                {
                  "RowType": "Row",
                  "Cells": [
                    {
                      "Value": "2018-01-15T00:00:00",
                      "Attributes": [
                        {
                          "Value": "935fc854-8037-4111-8d91-993010c331cc",
                          "Id": "invoiceID"
                        }
                      ]
                    },
                    { "Value": "" }
                  ]
                }
              ]
            }
          ]
        }
      ]
    }
    "#;

    let response: ReportsResponse = from_str(json).expect("Should deserialize Aged Payables");
    let report = &response.reports[0];

    assert_eq!(report.report_type.as_deref(), Some("AgedPayablesByContact"));
    assert_eq!(report.rows[0].row_type, "Header");

    let section = &report.rows[1];
    assert_eq!(section.row_type, "Section");
    assert!(!section.rows.is_empty());

    let data_row = &section.rows[0];
    let first_cell = &data_row.cells[0];
    assert_eq!(first_cell.value.as_ref().unwrap(), "2018-01-15T00:00:00");
    assert_eq!(first_cell.attributes[0].id, "invoiceID");
}

#[test]
fn test_deserialize_balance_sheet() {
    let json = r#"
    {
      "Reports": [
        {
          "ReportID": "BalanceSheet",
          "ReportName": "Balance Sheet",
          "ReportType": "BalanceSheet",
          "ReportDate": "31 December 2023",
          "UpdatedDateUTC": "/Date(1519357171249)/",
          "Rows": [
            {
              "RowType": "Section",
              "Title": "Assets"
            },
            {
              "RowType": "Section",
              "Title": "Bank",
              "Rows": [
                {
                  "RowType": "Row",
                  "Cells": [
                    {
                      "Value": "Business Bank Account",
                      "Attributes": [
                        {
                          "Value": "13918178-849a-4823-9a31-57b7eac713d7",
                          "Id": "account"
                        }
                      ]
                    },
                    { "Value": "-2894.08" }
                  ]
                },
                {
                  "RowType": "SummaryRow",
                  "Cells": [
                    { "Value": "Total Bank" },
                    { "Value": "3984.20" }
                  ]
                }
              ]
            }
          ]
        }
      ]
    }
    "#;

    let response: ReportsResponse = from_str(json).expect("Should deserialize Balance Sheet");
    let report = &response.reports[0];

    let bank_section = &report.rows[1];
    assert_eq!(bank_section.title.as_ref().unwrap(), "Bank");

    let summary_row = &bank_section.rows.last().unwrap();
    assert_eq!(summary_row.row_type, "SummaryRow");
    assert_eq!(summary_row.cells[0].value.as_ref().unwrap(), "Total Bank");
}
