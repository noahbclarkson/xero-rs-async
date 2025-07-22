// tests/accounting_get.rs

mod common;
use xero_rs_async::models::accounting::invoice::InvoiceType;

#[tokio::test]
async fn get_organisation() {
    let test_client = common::get_test_client().await;
    let api = test_client
        .client
        .accounting_for_tenant(test_client.tenant_id, None);
    let result = api.get_organisation().await;

    let orgs = result.expect("API call to get organisation failed");
    assert_eq!(orgs.len(), 1, "Expected exactly one organisation.");
    let org = &orgs[0];
    println!("Successfully retrieved organisation: {}", org.name);
}

#[tokio::test]
async fn get_accounts() {
    let test_client = common::get_test_client().await;
    let api = test_client
        .client
        .accounting_for_tenant(test_client.tenant_id, None);
    let result = api.get_accounts(None, None, None, None).await;

    let accounts = result.expect("API call to get accounts failed");
    assert!(
        !accounts.is_empty(),
        "Expected to find at least one account in the demo company."
    );
    println!("Successfully retrieved {} accounts.", accounts.len());

    // Test getting a single account by ID
    let first_account_id = accounts[0].account_id.unwrap();
    let single_result = api
        .get_accounts(Some(first_account_id), None, None, None)
        .await;
    let single_account_vec = single_result.expect("Failed to get single account by ID");
    assert_eq!(
        single_account_vec.len(),
        1,
        "Expected to get exactly one account by ID"
    );
    assert_eq!(single_account_vec[0].account_id, Some(first_account_id));
    println!(
        "Successfully retrieved single account: {:?}",
        single_account_vec[0].name
    );
}

#[tokio::test]
async fn get_invoices() {
    let test_client = common::get_test_client().await;
    let api = test_client
        .client
        .accounting_for_tenant(test_client.tenant_id, None);
    let result = api
        .get_invoices(
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(1),
            None,
            None,
            None,
        )
        .await;

    let invoices = result.expect("API call to get invoices failed");
    assert!(
        !invoices.is_empty(),
        "Expected to find at least one invoice in the demo company."
    );
    println!("Successfully retrieved {} invoices.", invoices.len());
}

#[tokio::test]
async fn get_report_balance_sheet() {
    let test_client = common::get_test_client().await;
    let api = test_client
        .client
        .accounting_for_tenant(test_client.tenant_id, None);
    let result = api.get_report("BalanceSheet", vec![]).await;

    let report = result.expect("API call to get BalanceSheet failed");
    assert_eq!(report.report_id, "BalanceSheet");
    assert!(
        !report.rows.is_empty(),
        "Balance sheet report should not be empty."
    );
    println!(
        "Successfully retrieved Balance Sheet report titled '{}'",
        report.report_name
    );
}

#[tokio::test]
async fn get_history_and_online_url_for_invoice() {
    let test_client = common::get_test_client().await;
    let api = test_client
        .client
        .accounting_for_tenant(test_client.tenant_id, None);

    // Find an ACCREC invoice to test with
    let invoices = api
        .get_invoices(
            None,
            None,
            None,
            Some(vec!["AUTHORISED".to_string()]),
            None,
            Some(format!(
                "Type == \"{}\"",
                serde_json::to_string(&InvoiceType::Accrec)
                    .unwrap()
                    .replace('\"', "")
            )),
            None,
            Some(1),
            None,
            None,
            None,
        )
        .await
        .expect("Failed to get invoices to test history");
    let invoice = invoices
        .first()
        .expect("No authorised ACCREC invoices found to test history/online URL");
    let invoice_id = invoice.invoice_id.unwrap();

    // Test get_history
    let history_result = api.get_history("Invoices", invoice_id).await;
    let history = history_result.expect("API call to get invoice history failed");
    assert!(
        !history.is_empty(),
        "Expected to find at least one history record for the invoice."
    );
    println!(
        "Successfully retrieved {} history records for invoice {}.",
        history.len(),
        invoice_id
    );

    // Test get_online_invoice_url
    let online_url_result = api.get_online_invoice_url(invoice_id).await;
    let online_invoice = online_url_result.expect("API call to get online invoice URL failed");
    println!(
        "Successfully retrieved online invoice URL: {}",
        online_invoice.online_invoice_url
    );
}