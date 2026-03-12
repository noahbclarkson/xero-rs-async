// tests/common.rs
#![allow(dead_code)]

use log::{error, info};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Once};
use tokio::sync::OnceCell;
use uuid::Uuid;
use xero_rs_async::auth::TokenSet;
use xero_rs_async::client::XeroClient;
use xero_rs_async::error::XeroError;
use xero_rs_async::rate_limiter::RateLimiter;

pub struct TestClient {
    pub client: XeroClient,
    pub tenant_id: Uuid,
}

pub trait XeroTestResult<T> {
    fn expect_xero(self, context: &str) -> T;
}

impl<T> XeroTestResult<T> for Result<T, XeroError> {
    fn expect_xero(self, context: &str) -> T {
        match self {
            Ok(value) => value,
            Err(err) => {
                error!("Test failed: {context}");
                match &err {
                    XeroError::Api { status, message } => {
                        error!("Xero API error status: {status}");
                        error!("Xero raw response: {message}");
                    }
                    XeroError::SerdeWithBody { body, .. } => {
                        error!("Xero raw response: {body}");
                    }
                    _ => {
                        error!("Xero error: {err}");
                        error!("Xero raw response: <none - request failed before response>");
                    }
                }
                panic!("{context}: {err}");
            }
        }
    }
}

const ACCOUNTING_BASE_URL: &str = "https://api.xero.com/api.xro/2.0";
const ASSETS_BASE_URL: &str = "https://api.xero.com/assets.xro/1.0";
const FILES_BASE_URL: &str = "https://api.xero.com/files.xro/1.0";

pub async fn log_raw_accounting_response(
    client: &TestClient,
    path: &str,
    query: Option<&[(String, String)]>,
) {
    log_raw_response(client, ACCOUNTING_BASE_URL, path, query).await;
}

pub async fn log_raw_assets_response(
    client: &TestClient,
    path: &str,
    query: Option<&[(String, String)]>,
) {
    log_raw_response(client, ASSETS_BASE_URL, path, query).await;
}

pub async fn log_raw_files_response(
    client: &TestClient,
    path: &str,
    query: Option<&[(String, String)]>,
) {
    log_raw_response(client, FILES_BASE_URL, path, query).await;
}

async fn log_raw_response(
    client: &TestClient,
    base_url: &str,
    path: &str,
    query: Option<&[(String, String)]>,
) {
    let access_token = match client.client.token_manager.get_access_token().await {
        Ok(token) => token,
        Err(err) => {
            error!("Failed to get access token for raw response logging: {err}");
            return;
        }
    };

    let url = format!("{base_url}{path}");
    let http_client = reqwest::Client::new();
    let mut request = http_client
        .get(&url)
        .bearer_auth(access_token)
        .header("xero-tenant-id", client.tenant_id.to_string())
        .header("Accept", "application/json");

    if let Some(params) = query {
        request = request.query(params);
    }

    match request.send().await {
        Ok(response) => {
            let status = response.status();
            match response.text().await {
                Ok(body) => {
                    eprintln!("\n=== RAW XERO RESPONSE ===");
                    eprintln!("URL: {url}");
                    eprintln!("Status: {status}");
                    eprintln!("{body}");
                    eprintln!("=== END RAW XERO RESPONSE ===\n");
                }
                Err(err) => {
                    error!("Failed to read raw response body from {url}: {err}");
                }
            }
        }
        Err(err) => {
            error!("Failed to fetch raw response from {url}: {err}");
        }
    }
}

pub async fn assert_non_empty_accounting<T>(
    client: &TestClient,
    items: &[T],
    context: &str,
    path: &str,
    query: Option<&[(String, String)]>,
) {
    if items.is_empty() {
        log_raw_accounting_response(client, path, query).await;
        panic!("{context}");
    }
}

pub async fn assert_non_empty_assets<T>(
    client: &TestClient,
    items: &[T],
    context: &str,
    path: &str,
    query: Option<&[(String, String)]>,
) {
    if items.is_empty() {
        log_raw_assets_response(client, path, query).await;
        panic!("{context}");
    }
}

pub async fn assert_non_empty_files<T>(
    client: &TestClient,
    items: &[T],
    context: &str,
    path: &str,
    query: Option<&[(String, String)]>,
) {
    if items.is_empty() {
        log_raw_files_response(client, path, query).await;
        panic!("{context}");
    }
}

static INIT_LOGGER: Once = Once::new();
// This will hold our single, shared RateLimiter for all tests.
static RATE_LIMITER: OnceCell<Arc<RateLimiter>> = OnceCell::const_new();

// Helper function to initialize the logger
fn setup_logger() {
    INIT_LOGGER.call_once(|| {
        // is_test(true) helps capture output correctly in tests
        // try_init() is used to prevent panic if logger is already set
        env_logger::builder().is_test(true).try_init().ok();
    });
}

/// Creates a new, isolated TestClient that uses a shared RateLimiter.
pub async fn get_test_client() -> TestClient {
    setup_logger();

    dotenvy::dotenv().expect("Failed to load .env file. Make sure it exists at the project root.");

    // Get or initialize the single, shared RateLimiter instance.
    let rate_limiter = RATE_LIMITER
        .get_or_init(|| async {
            Arc::new(
                RateLimiter::new()
                    .await
                    .expect("Failed to create shared RateLimiter"),
            )
        })
        .await
        .clone();

    let client_id = env::var("XERO_CLIENT_ID").expect("XERO_CLIENT_ID must be set.");
    let client_secret = env::var("XERO_CLIENT_SECRET").expect("XERO_CLIENT_SECRET must be set.");
    let redirect_uri = env::var("XERO_REDIRECT_URI").expect("XERO_REDIRECT_URI must be set.");
    let tenant_id_str = env::var("XERO_TENANT_ID").expect("XERO_TENANT_ID must be set.");

    let tenant_id = Uuid::parse_str(&tenant_id_str).expect("XERO_TENANT_ID is not a valid UUID.");

    let client = XeroClient::new(
        client_id,
        client_secret,
        redirect_uri,
        rate_limiter, // Inject the shared limiter
    )
    .await
    .expect("Failed to create XeroClient");

    let token_path = env::var("XERO_TOKEN_PATH").unwrap_or_else(|_| "xero_token.json".to_string());
    let token_path = PathBuf::from(token_path);
    let token_file = fs::read_to_string(&token_path).unwrap_or_else(|err| {
        panic!(
            "Failed to read token file at {}: {err}. Ensure xero_token.json exists next to .env (or set XERO_TOKEN_PATH).",
            token_path.display()
        )
    });
    let token_set: TokenSet = serde_json::from_str(&token_file).unwrap_or_else(|err| {
        panic!(
            "Failed to parse token file at {}: {err}. Ensure it matches the TokenSet schema.",
            token_path.display()
        )
    });
    client.token_manager.set_token(&token_set).await;

    info!("✅ Test client initialized for tenant: {tenant_id}");

    TestClient { client, tenant_id }
}
