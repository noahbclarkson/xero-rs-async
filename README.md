# xero-rs-async

[![CI](https://github.com/noahbclarkson/xero-rs-async/actions/workflows/ci.yml/badge.svg)](https://github.com/noahbclarkson/xero-rs-async/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/xero-rs-async.svg)](https://crates.io/crates/xero-rs-async)
[![Docs.rs](https://docs.rs/xero-rs-async/badge.svg)](https://docs.rs/xero-rs-async)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](./LICENSE-MIT)

An unofficial, asynchronous Rust SDK for the Xero API.

This library provides a typed, ergonomic, and robust interface for interacting with the Xero Accounting, Assets, and Files APIs. It is built with `tokio` and `reqwest` for a fully async experience and includes built-in, persistent OAuth 2.0 token management and a smart rate limiter to ensure compliance with all Xero API limits.

## Features

- **Fully Asynchronous**: Built on `tokio` and `reqwest` for non-blocking I/O.
- **Complete API Models**: Typed, serializable structs for all Xero API resources, powered by `serde`.
- **Automatic OAuth 2.0 Handling**: Manages the entire OAuth 2.0 flow, including automatic token refreshing and caching to a local file.
- **Built-in Rate Limiting**: A persistent, tenant-aware rate limiter that respects Xero's concurrent, per-minute, and daily limits to prevent your application from being blocked.
- **Ergonomic API Design**: A clean client structure with convenient, per-tenant API handles to simplify calls.
- **Comprehensive Test Suite**: Integration tests that run against the Xero Demo Company to ensure reliability.

## Getting Started

### 1. Add to `Cargo.toml`

Add the library to your project's dependencies.

```sh
cargo add xero-rs-async
```

Or add it manually:

```toml
[dependencies]
xero-rs-async = "0.1.0" # Replace with the latest version
```

### 2. Configure Your Environment

The client is configured using environment variables. Create a `.env` file in your project root and add your Xero application credentials. You can get these from the [Xero Developer Portal](https://developer.xero.com/app/manage).

```dotenv
# .env
# Copy this file from .env.example and fill in your Xero App credentials.

# Get these from your app at https://developer.xero.com/myapps
XERO_CLIENT_ID="YOUR_XERO_CLIENT_ID"
XERO_CLIENT_SECRET="YOUR_XERO_CLIENT_SECRET"
XERO_REDIRECT_URI="http://localhost/" # Must match the redirect URI in your Xero app

# The tenant ID of the demo company or your test organisation.
# You can get this after authorizing by making a call to the connections endpoint.
XERO_TENANT_ID="YOUR_XERO_TENANT_ID"

# Paths for cache files. These can be left as-is.
TOKEN_CACHE_PATH="xero_token.json"
RATE_LIMIT_CACHE_PATH="xero_rate_limit_history.json"
```

### 3. Quickstart Example

This example shows how to initialize the client and make a simple API call to fetch your organization details.

```rust
use xero_rs_async::client::XeroClient;
use xero_rs_async::rate_limiter::RateLimiter;
use std::env;
use std::path::PathBuf;
use std::sync::Arc;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Load configuration from .env file
    dotenvy::dotenv().expect("Failed to load .env file");

    let client_id = env::var("XERO_CLIENT_ID")?;
    let client_secret = env::var("XERO_CLIENT_SECRET")?;
    let redirect_uri = env::var("XERO_REDIRECT_URI")?;
    let tenant_id_str = env::var("XERO_TENANT_ID")?;
    let token_cache_path = PathBuf::from(env::var("TOKEN_CACHE_PATH")?);
    let rate_limit_cache_path = PathBuf::from(env::var("RATE_LIMIT_CACHE_PATH")?);

    // 2. Initialize a shared, persistent rate limiter
    let rate_limiter = Arc::new(RateLimiter::new(rate_limit_cache_path).await?);

    // 3. Create the Xero Client
    // The client will automatically load and refresh tokens from `token_cache_path`.
    let xero_client = XeroClient::new(
        client_id,
        client_secret,
        redirect_uri,
        token_cache_path,
        rate_limiter,
    ).await?;

    // 4. Get the Tenant ID and create a tenant-specific API handle
    let tenant_id = Uuid::parse_str(&tenant_id_str)?;
    let accounting_api = xero_client.accounting_for_tenant(tenant_id);

    // 5. Make an API call
    println!("Fetching organization details...");
    let orgs = accounting_api.get_organisation().await?;
    let org = orgs.first().expect("No organization found!");

    println!("Successfully connected to: {}", org.name);
    println!("Organisation Type: {}", org.organisation_type);
    println!("Base Currency: {}", org.base_currency);

    Ok(())
}
```

## Authentication

Xero uses the OAuth 2.0 Authorization Code Grant. This library provides a utility binary to handle the initial, interactive authorization flow.

### One-Time Setup

1. **Run the `auth` binary:**

    ```sh
    cargo run --bin auth
    ```

2. **Authorize in Browser:** Your web browser will open to the Xero login and authorization page. Log in and select the organization you want to grant access to.
3. **Callback Handling:** After you approve, Xero will redirect back to a temporary local server run by the utility. The utility will catch the authorization code, exchange it for a token set, and save it to the file specified by `TOKEN_CACHE_PATH` (e.g., `xero_token.json`).

Once `xero_token.json` exists, the `XeroClient` will automatically use and refresh the tokens from this file for all subsequent API calls. You do not need to run the `auth` utility again unless the refresh token expires or is revoked.

## Usage Examples

### Tenanted API Handles

The most convenient way to use the API is to create a handle for a specific tenant. This avoids passing the `tenant_id` on every call.

```rust
# use xero_rs_async::client::XeroClient;
# use uuid::Uuid;
# async fn example(xero_client: XeroClient, tenant_id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
// Create a handle bound to a specific tenant
let accounting_api = xero_client.accounting_for_tenant(tenant_id);

// Now you can make calls without passing the tenant_id
let accounts = accounting_api.get_accounts(None, None, None, None).await?;
println!("Found {} accounts", accounts.len());

let assets_api = xero_client.assets_for_tenant(tenant_id);
let settings = assets_api.get_asset_settings().await?;
println!("Asset prefix: {}", settings.asset_number_prefix);
# Ok(())
# }
```

### Filtering and Pagination

Most `get` methods support filtering with a `where` clause and pagination.

```rust
# use xero_rs_async::client::XeroClient;
# use uuid::Uuid;
# async fn example(xero_client: XeroClient, tenant_id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
let accounting_api = xero_client.accounting_for_tenant(tenant_id);

// Find all 'AUTHORISED' invoices with an amount due over 100
let filter = "Status == \"AUTHORISED\" AND AmountDue > 100.00".to_string();
let order = "Date DESC".to_string();

let invoices = accounting_api.get_invoices(
    None,           // invoice_id
    None,           // invoice_numbers
    None,           // contact_ids
    None,           // statuses
    None,           // modified_after
    Some(filter),   // where_filter
    Some(order),    // order_by
    Some(1),        // page
    Some(50),       // page_size
    None,           // summary_only
    None,           // search_term
).await?;

println!("Found {} matching invoices.", invoices.len());
# Ok(())
# }
```

### Creating a Resource

To create a resource, build the corresponding model struct and pass it to a `create_*` method.

```rust
# use xero_rs_async::client::XeroClient;
# use uuid::Uuid;
use xero_rs_async::models::accounting::contact::Contact;

# async fn example(xero_client: XeroClient, tenant_id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
let accounting_api = xero_client.accounting_for_tenant(tenant_id);

let new_contact = Contact {
    name: "ACME Inc.".to_string(),
    first_name: Some("John".to_string()),
    last_name: Some("Smith".to_string()),
    email_address: Some("john.smith@acme.com".to_string()),
    is_customer: Some(true),
    ..Default::default()
};

let created_contacts = accounting_api.create_contacts(vec![new_contact]).await?;
let created_contact = created_contacts.first().unwrap();

println!("Successfully created contact with ID: {:?}", created_contact.contact_id);
# Ok(())
# }
```

## API Coverage

| API            | Status          |
| -------------- | --------------- |
| Accounting API | Fully Supported |
| Assets API     | Fully Supported |
| Files API      | Fully Supported |

## Contributing

Contributions are welcome! Please feel free to submit a pull request.

To contribute:

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Make your changes.
4. Ensure the code is formatted with `cargo fmt`.
5. Ensure all lints pass with `cargo clippy -- -D warnings`.
6. Ensure all tests pass with `cargo test`.
7. Submit a pull request.

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](./LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
