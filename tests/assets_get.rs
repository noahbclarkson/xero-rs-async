// tests/assets_get.rs

mod common;
use xero_rs_async::models::assets::asset::AssetStatus;

#[tokio::test]
async fn get_asset_settings() {
    let test_client = common::get_test_client().await;
    let api = test_client.client.assets_for_tenant(test_client.tenant_id);
    let result = api.get_asset_settings().await;

    let settings = result.expect("API call to get asset settings failed");
    assert!(
        !settings.asset_number_prefix.is_empty(),
        "Asset number prefix should not be empty."
    );
    println!(
        "Successfully retrieved asset settings. Prefix: {}",
        settings.asset_number_prefix
    );
}

#[tokio::test]
async fn get_asset_types() {
    let test_client = common::get_test_client().await;
    let api = test_client.client.assets_for_tenant(test_client.tenant_id);
    let result = api.get_asset_types().await;

    let asset_types = result.expect("API call to get asset types failed");
    assert!(
        !asset_types.is_empty(),
        "Expected to find at least one asset type."
    );
    println!("Successfully retrieved {} asset types.", asset_types.len());
}

#[tokio::test]
async fn get_assets_and_by_id() {
    let test_client = common::get_test_client().await;
    let api = test_client.client.assets_for_tenant(test_client.tenant_id);

    // Get a list of registered assets
    let result = api
        .get_assets(AssetStatus::Registered, None, None, None, None, None)
        .await;

    let assets = result.expect("API call to get assets failed");
    assert!(!assets.is_empty(), "Expected to find registered assets.");
    println!("Successfully retrieved {} registered assets.", assets.len());

    // Test getting a single asset by ID
    let first_asset_id = assets[0].asset_id;
    let single_result = api.get_asset_by_id(first_asset_id).await;
    let single_asset = single_result.expect("Failed to get single asset by ID");

    assert_eq!(
        single_asset.asset_id, first_asset_id,
        "Returned asset ID does not match requested ID."
    );
    println!(
        "Successfully retrieved single asset by ID: {}",
        single_asset.asset_name
    );
}
