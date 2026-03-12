# xero-rs-async

[![CI](https://github.com/noahbclarkson/xero-rs-async/actions/workflows/ci.yml/badge.svg)](https://github.com/noahbclarkson/xero-rs-async/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/xero-rs-async.svg)](https://crates.io/crates/xero-rs-async)
[![Docs.rs](https://docs.rs/xero-rs-async/badge.svg)](https://docs.rs/xero-rs-async)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](./LICENSE-MIT)

An unofficial, asynchronous Rust SDK for the [Xero API](https://developer.xero.com/documentation/api/api-overview).

This library provides a typed, ergonomic interface for interacting with **10 Xero API areas** — Accounting, Assets, Files, Projects, Bank Feeds, Practice Manager (XPM), and Payroll for AU, NZ, and UK. It is built on `tokio` and `reqwest` for fully async I/O and includes built-in OAuth 2.0 token management and a tenant-aware rate limiter.

## Features

- **Fully Asynchronous** — Built on `tokio` and `reqwest` for non-blocking I/O.
- **Broad API Coverage** — Typed models and endpoints for Accounting, Assets, Files, Projects, Bank Feeds, Practice Manager, and Payroll (AU/NZ/UK).
- **Feature-Gated Modules** — Only compile what you need. Each API area is behind a Cargo feature flag.
- **Automatic OAuth 2.0** — Manages the full OAuth 2.0 Authorization Code flow, including automatic token refreshing.
- **Built-in Rate Limiting** — Tenant-aware rate limiter that respects Xero's concurrent, per-minute, and daily limits.
- **Ergonomic Per-Tenant Handles** — Create a handle bound to a tenant ID and make calls without passing it every time.
- **Practice Manager (XPM) Support** — Full XML-based API support for clients, jobs, staff, time entries, invoices, and more.
- **Comprehensive Test Suite** — Integration tests against the Xero Demo Company.

## API Coverage

| API                   | Feature Flag        | Status           |
| --------------------- | ------------------- | ---------------- |
| Accounting API        | `accounting` (default) | Fully Supported |
| Assets API            | `assets`            | Fully Supported  |
| Files API             | `files`             | Fully Supported  |
| Projects API          | `projects`          | Fully Supported  |
| Bank Feeds API        | `bank-feeds`        | Fully Supported  |
| Practice Manager (XPM)| `practice-manager` | Fully Supported  |
| Payroll API (AU)      | `payroll-au`        | Fully Supported  |
| Payroll API (NZ)      | `payroll-nz`        | Fully Supported  |
| Payroll API (UK)      | `payroll-uk`        | Fully Supported  |

## Getting Started

### 1. Add to `Cargo.toml`

```sh
cargo add xero-rs-async
```

By default, only the `accounting` feature is enabled. Enable additional APIs as needed:

```toml
[dependencies]
xero-rs-async = { version = "0.1.0", features = ["accounting", "assets", "payroll-nz", "projects"] }
```

To enable everything:

```toml
[dependencies]
xero-rs-async = { version = "0.1.0", features = [
    "accounting", "assets", "files", "projects",
    "bank-feeds", "practice-manager",
    "payroll-au", "payroll-nz", "payroll-uk",
] }
```

### 2. Configure Your Environment

Create a `.env` file with your [Xero app credentials](https://developer.xero.com/app/manage):

```dotenv
XERO_CLIENT_ID="YOUR_XERO_CLIENT_ID"
XERO_CLIENT_SECRET="YOUR_XERO_CLIENT_SECRET"
XERO_REDIRECT_URI="http://localhost/"
XERO_TENANT_ID="YOUR_XERO_TENANT_ID"
TOKEN_CACHE_PATH="xero_token.json"
RATE_LIMIT_CACHE_PATH="xero_rate_limit_history.json"
```

### 3. Quickstart

```rust
use xero_rs_async::client::XeroClient;
use xero_rs_async::rate_limiter::RateLimiter;
use std::env;
use std::path::PathBuf;
use std::sync::Arc;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().expect("Failed to load .env file");

    let client_id = env::var("XERO_CLIENT_ID")?;
    let client_secret = env::var("XERO_CLIENT_SECRET")?;
    let redirect_uri = env::var("XERO_REDIRECT_URI")?;
    let tenant_id_str = env::var("XERO_TENANT_ID")?;
    let token_cache_path = PathBuf::from(env::var("TOKEN_CACHE_PATH")?);
    let rate_limit_cache_path = PathBuf::from(env::var("RATE_LIMIT_CACHE_PATH")?);

    let rate_limiter = Arc::new(RateLimiter::new(rate_limit_cache_path).await?);

    let xero_client = XeroClient::new(
        client_id,
        client_secret,
        redirect_uri,
        token_cache_path,
        rate_limiter,
    ).await?;

    let tenant_id = Uuid::parse_str(&tenant_id_str)?;
    let accounting_api = xero_client.accounting_for_tenant(tenant_id);

    let orgs = accounting_api.get_organisation().await?;
    let org = orgs.first().expect("No organization found!");
    println!("Connected to: {}", org.name);

    Ok(())
}
```

## Authentication

Xero uses OAuth 2.0 Authorization Code Grant. This library includes a CLI utility to handle the initial interactive flow.

### One-Time Setup

1. Enable the `auth-cli` feature:

    ```toml
    [dependencies]
    xero-rs-async = { version = "0.1.0", features = ["auth-cli"] }
    ```

2. Run the auth binary:

    ```sh
    cargo run --bin auth --features auth-cli
    ```

3. Authorize in your browser — Xero will redirect back to a temporary local server. The utility exchanges the authorization code for tokens and saves them to `TOKEN_CACHE_PATH`.

Once `xero_token.json` exists, `XeroClient` automatically uses and refreshes tokens for all subsequent calls.

## Usage Examples

### Per-Tenant API Handles

```rust
// Accounting
let accounting = xero_client.accounting_for_tenant(tenant_id);
let accounts = accounting.get_accounts(None, None, None, None).await?;

// Assets
let assets = xero_client.assets_for_tenant(tenant_id);
let settings = assets.get_asset_settings().await?;

// Projects
let projects = xero_client.projects_for_tenant(tenant_id);

// Payroll (NZ)
let payroll = xero_client.payroll_nz_for_tenant(tenant_id);
```

### Filtering and Pagination

```rust
let accounting = xero_client.accounting_for_tenant(tenant_id);

let filter = "Status == \"AUTHORISED\" AND AmountDue > 100.00".to_string();
let order = "Date DESC".to_string();

let invoices = accounting.get_invoices(
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
```

### Creating Resources

```rust
use xero_rs_async::models::accounting::contact::Contact;

let new_contact = Contact {
    name: "ACME Inc.".to_string(),
    first_name: Some("John".to_string()),
    last_name: Some("Smith".to_string()),
    email_address: Some("john.smith@acme.com".to_string()),
    is_customer: Some(true),
    ..Default::default()
};

let created = accounting.create_contacts(vec![new_contact]).await?;
```

### Practice Manager (XML API)

```rust
let xpm = xero_client.practice_manager_for_tenant(tenant_id);
let clients = xpm.list_clients(None, None).await?;
let staff = xpm.list_staff(None).await?;
```

## Project Structure

```
src/
  api/               -- Endpoint implementations per API area
    accounting/      -- 30+ resource endpoints (invoices, contacts, accounts, ...)
    bank_feeds/      -- Feed connections, statements
    payroll_au/      -- AU payroll (employees, pay runs, leave, super, ...)
    payroll_nz/      -- NZ payroll (employees, pay runs, leave, timesheets, ...)
    payroll_uk/      -- UK payroll (employees, pay runs, pensions, statutory leave, ...)
    practice_manager/-- XPM (clients, jobs, staff, time, invoices, ...)
    projects/        -- Projects, tasks, time entries, users
  models/            -- Typed request/response structs (serde)
    accounting/      -- Accounts, invoices, contacts, journals, tax rates, ...
    assets/          -- Fixed assets, asset types, settings
    bank_feeds/      -- Feed connections, statements
    files/           -- Files, folders, associations
    payroll_au/      -- AU payroll models
    payroll_nz/      -- NZ payroll models
    payroll_uk/      -- UK payroll models
    practice_manager/-- XPM models (XML-serializable)
    projects/        -- Project models
  auth.rs            -- OAuth 2.0 token manager
  client.rs          -- XeroClient with per-tenant handle constructors
  error.rs           -- Error types
  http.rs            -- HTTP abstraction layer
  rate_limiter.rs    -- Tenant-aware rate limiter
```

## Contributing

Contributions are welcome! Please feel free to submit a pull request.

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Ensure the code is formatted with `cargo fmt`.
4. Ensure all lints pass with `cargo clippy -- -D warnings`.
5. Ensure all tests pass with `cargo test`.
6. Submit a pull request.

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](./LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
