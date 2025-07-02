// tests/common.rs

use log::info;
use std::env;
use std::path::PathBuf;
use std::sync::{Arc, Once};
use tokio::sync::OnceCell;
use uuid::Uuid;
use xero_rs_async::client::XeroClient;
use xero_rs_async::rate_limiter::RateLimiter;

pub struct TestClient {
    pub client: XeroClient,
    pub tenant_id: Uuid,
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

    let rate_limit_cache_path =
        env::var("RATE_LIMIT_CACHE_PATH").expect("RATE_LIMIT_CACHE_PATH must be set.");

    // Get or initialize the single, shared RateLimiter instance.
    let rate_limiter = RATE_LIMITER
        .get_or_init(|| async {
            Arc::new(
                RateLimiter::new(PathBuf::from(rate_limit_cache_path))
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
    let token_cache_path = env::var("TOKEN_CACHE_PATH").expect("TOKEN_CACHE_PATH must be set.");

    let tenant_id = Uuid::parse_str(&tenant_id_str).expect("XERO_TENANT_ID is not a valid UUID.");

    let client = XeroClient::new(
        client_id,
        client_secret,
        redirect_uri,
        PathBuf::from(token_cache_path),
        rate_limiter, // Inject the shared limiter
    )
    .await
    .expect("Failed to create XeroClient");

    info!("✅ Test client initialized for tenant: {}", tenant_id);

    TestClient { client, tenant_id }
}
