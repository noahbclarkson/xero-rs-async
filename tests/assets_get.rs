#![cfg(feature = "assets")]

// tests/assets_get.rs

mod common;
use common::{assert_non_empty_assets, log_assets_response_metadata, XeroTestResult};
use xero_rs_async::models::assets::asset::AssetStatus;

/// Debug test to inspect response shape without printing provider data.
/// Run with: cargo test -p xero-rs-async --test assets_get debug_assets_response_shape -- --nocapture
#[tokio::test]
#[ignore = "Requires Xero API credentials"]
async fn debug_assets_response_shape() {
    let test_client = common::get_test_client().await;

    // Get access token
    let access_token = test_client
        .client
        .token_manager
        .get_access_token()
        .await
        .expect_xero("Failed to get access token");

    // Make a direct request so generic and typed parsing can be compared locally.
    let http_client = reqwest::Client::new();
    let response = http_client
        .get("https://api.xero.com/assets.xro/1.0/Assets")
        .bearer_auth(&access_token)
        .header("xero-tenant-id", test_client.tenant_id.to_string())
        .header("Accept", "application/json")
        .query(&[("status", "REGISTERED")])
        .send()
        .await
        .expect("Failed to send request");

    let status = response.status();
    let raw_json = response.text().await.expect("Failed to get response text");

    println!("\n=== ASSETS API RESPONSE METADATA ===");
    println!("Status: {status}");
    println!("Response length: {} chars", raw_json.len());
    println!("Response body: [REDACTED]\n");

    // Try to parse it and show the specific error
    let parse_result: Result<serde_json::Value, _> = serde_json::from_str(&raw_json);
    match parse_result {
        Ok(value) => {
            println!("=== PARSED AS GENERIC JSON (SUCCESS) ===");
            // Report only structural metadata, never provider values.
            if let Some(object) = value.as_object() {
                let mut keys: Vec<_> = object.keys().map(String::as_str).collect();
                keys.sort_unstable();
                println!("Top-level keys: {keys:?}");
            }
            if let Some(items) = value.get("items").and_then(|v| v.as_array()) {
                println!("Found {} items", items.len());
            }
        }
        Err(e) => {
            println!("=== FAILED TO PARSE AS GENERIC JSON ===");
            println!(
                "Classification: {:?}; line={}; column={}",
                e.classify(),
                e.line(),
                e.column()
            );
        }
    }

    // Now try to parse using the actual Asset model
    println!("\n=== TRYING TO PARSE WITH AssetsResponse MODEL ===");
    use xero_rs_async::models::assets::asset::Asset;

    #[derive(serde::Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct TestAssetsResponse {
        #[allow(dead_code)]
        #[serde(default)]
        pagination: Option<serde_json::Value>,
        #[allow(dead_code)]
        #[serde(default)]
        links: Option<serde_json::Value>,
        items: Vec<Asset>,
    }

    let model_result: Result<TestAssetsResponse, _> = serde_json::from_str(&raw_json);
    match model_result {
        Ok(resp) => {
            println!("SUCCESS! Parsed {} assets", resp.items.len());
        }
        Err(e) => {
            println!("FAILED to parse with Asset model!");
            println!(
                "Classification: {:?}; line={}; column={}",
                e.classify(),
                e.line(),
                e.column()
            );
        }
    }

    // The test passes regardless - it's for debugging
    if !status.is_success() {
        log_assets_response_metadata(&test_client, "/Assets", None).await;
        panic!("API call failed with status {status}");
    }
}

#[tokio::test]
#[ignore = "Requires Xero API credentials"]
async fn get_asset_settings() {
    let test_client = common::get_test_client().await;
    let api = test_client.client.assets_for_tenant(test_client.tenant_id);
    let result = api.get_asset_settings().await;

    let settings = result.expect_xero("API call to get asset settings failed");
    if settings.asset_number_prefix.is_empty() {
        log_assets_response_metadata(&test_client, "/Settings", None).await;
        panic!("Asset number prefix should not be empty.");
    }
    println!(
        "Successfully retrieved asset settings. Prefix: {}",
        settings.asset_number_prefix
    );
}

#[tokio::test]
#[ignore = "Requires Xero API credentials"]
async fn get_asset_types() {
    let test_client = common::get_test_client().await;
    let api = test_client.client.assets_for_tenant(test_client.tenant_id);
    let result = api.get_asset_types().await;

    let asset_types = result.expect_xero("API call to get asset types failed");
    assert_non_empty_assets(
        &test_client,
        &asset_types,
        "Expected to find at least one asset type.",
        "/AssetTypes",
        None,
    )
    .await;
    println!("Successfully retrieved {} asset types.", asset_types.len());
}

#[tokio::test]
#[ignore = "Requires Xero API credentials"]
async fn get_assets_and_by_id() {
    let test_client = common::get_test_client().await;
    let api = test_client.client.assets_for_tenant(test_client.tenant_id);

    // Get a list of registered assets
    let result = api
        .get_assets(AssetStatus::Registered, None, None, None, None, None)
        .await;

    let assets = result.expect_xero("API call to get assets failed");
    let assets_query = vec![("status".to_string(), "REGISTERED".to_string())];
    assert_non_empty_assets(
        &test_client,
        &assets,
        "Expected to find registered assets.",
        "/Assets",
        Some(&assets_query),
    )
    .await;
    println!("Successfully retrieved {} registered assets.", assets.len());

    // Test getting a single asset by ID
    let first_asset_id = assets[0].asset_id;
    let single_result = api.get_asset_by_id(first_asset_id).await;
    let single_asset = single_result.expect_xero("Failed to get single asset by ID");

    assert_eq!(
        single_asset.asset_id, first_asset_id,
        "Returned asset ID does not match requested ID."
    );
    println!(
        "Successfully retrieved single asset by ID: {}",
        single_asset.asset_name
    );
}
