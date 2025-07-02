//! # xero-rs-async
//!
//! An unofficial, asynchronous Rust SDK for the Xero API.
//!
//! This library provides a typed, easy-to-use interface for interacting with the
//! Xero Accounting, Assets, and Files APIs. It includes built-in rate limiting
//! and OAuth 2.0 token management.
//!
//! ## Features
//!
//! - Fully asynchronous using `tokio` and `reqwest`.
//! - Complete, typed models for all Xero API resources.
//! - Built-in, persistent rate limiter to avoid hitting API limits.
//! - Automatic OAuth 2.0 token handling, including refresh tokens.
//! - A clean, modular structure separating different Xero APIs.
//! - Convenient per-tenant API handles to simplify calls.
//!
//! ## Quickstart
//!
#![doc = r#"
```rust,no_run
use xero_rs_async::client::XeroClient;
use xero_rs_async::rate_limiter::RateLimiter;
use std::path::PathBuf;
use std::sync::Arc;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client_id = "YOUR_CLIENT_ID".to_string();
    let client_secret = "YOUR_CLIENT_SECRET".to_string();
    let redirect_uri = "http://localhost/callback".to_string();
    let token_cache = PathBuf::from("xero_token.json");
    let rate_limit_cache = PathBuf::from("xero_rate_limit_history.json");

    let rate_limiter = Arc::new(RateLimiter::new(rate_limit_cache).await?);

    let xero_client = XeroClient::new(
        client_id,
        client_secret,
        redirect_uri,
        token_cache,
        rate_limiter
    ).await?;

    // --- Authentication (this would typically involve a web server) ---
    // 1. Get the authorization URL
    // let auth_url = xero_client.token_manager.get_authorization_url(&["accounting.transactions.read"], "state123");
    // 2. Redirect the user to `auth_url` and get the `code` from the callback.
    // 3. Exchange the code for a token set.
    // xero_client.token_manager.exchange_code("THE_CODE_FROM_CALLBACK").await?;

    // --- Making API Calls ---
    // You must have a valid token and tenant_id first.
    let tenant_id = Uuid::parse_str("00000000-0000-0000-0000-000000000000")?;

    // Create a handle for a specific tenant
    let accounting_api = xero_client.accounting_for_tenant(tenant_id);

    // Now you can make calls without passing the tenant_id every time
    // let accounts = accounting_api.get_accounts(None, None, None, None).await?;
    // println!("Found {} accounts", accounts.len());

    Ok(())
}
```
"#]
//! ```
extern crate log;

pub mod auth;
pub mod client;
pub mod endpoints;
pub mod error;
pub mod models;
pub mod rate_limiter;
mod util;
