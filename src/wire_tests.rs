//! Wire-level behaviour against a local server: attempts, `Retry-After`, redaction, connections.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use reqwest::Method;
use serde_json::json;
use uuid::Uuid;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use crate::auth::{TokenManager, TokenSet};
use crate::client::XeroClient;
use crate::error::{ErrorKind, XeroError};
use crate::http::ApiClient;
use crate::observer::{Attempt, AttemptKind};
use crate::rate_limiter::RateLimiter;

const PRIVATE_BODY: &str = "Invoice INV-9 for Acme Ltd totals $12,000.00";

async fn client_for(server: &MockServer) -> XeroClient {
    let http = reqwest::Client::new();
    let manager = TokenManager::new(
        http.clone(),
        "client-id".into(),
        "client-secret".into(),
        "http://localhost/callback".into(),
    )
    .with_identity_base_url(&server.uri());
    let limiter = Arc::new(RateLimiter::new().await.expect("limiter"));
    XeroClient::assemble(http, Arc::new(manager), limiter)
        .with_connections_url(&format!("{}/connections", server.uri()))
}

fn stored_token() -> TokenSet {
    TokenSet {
        access_token: "old-access".into(),
        refresh_token: Some("refresh".into()),
        token_type: "Bearer".into(),
        expires_in: 1800,
        ..TokenSet::default()
    }
}

fn token_body() -> serde_json::Value {
    json!({
        "access_token": "new-access",
        "refresh_token": "new-refresh",
        "expires_in": 1800,
        "token_type": "Bearer"
    })
}

#[derive(Default)]
struct Recorder(Mutex<Vec<AttemptKind>>);

impl Recorder {
    fn kinds(&self) -> Vec<AttemptKind> {
        self.0.lock().expect("recorder").clone()
    }
}

impl crate::observer::AttemptObserver for Recorder {
    fn on_attempt(&self, attempt: &Attempt) {
        self.0.lock().expect("recorder").push(attempt.kind);
    }
}

#[tokio::test]
async fn a_refresh_reports_every_request_including_the_retries() {
    let server = MockServer::start().await;
    let hits = Arc::new(AtomicUsize::new(0));
    let counter = hits.clone();
    Mock::given(method("POST"))
        .and(path("/connect/token"))
        .respond_with(move |_: &wiremock::Request| {
            if counter.fetch_add(1, Ordering::SeqCst) < 2 {
                ResponseTemplate::new(503).set_body_string(PRIVATE_BODY)
            } else {
                ResponseTemplate::new(200).set_body_json(token_body())
            }
        })
        .mount(&server)
        .await;
    let client = client_for(&server).await;
    let installed = Arc::new(Recorder::default());
    client.set_attempt_observer(installed.clone());
    let scoped = Recorder::default();

    let refreshed = client
        .token_manager
        .refresh_token_no_cache_observed(&stored_token(), &scoped)
        .await
        .expect("third attempt succeeds");

    assert_eq!(refreshed.access_token, "new-access");
    assert_eq!(hits.load(Ordering::SeqCst), 3);
    assert_eq!(scoped.kinds(), vec![AttemptKind::TokenRefresh; 3]);
    assert_eq!(installed.kinds(), vec![AttemptKind::TokenRefresh; 3]);
}

#[tokio::test]
async fn a_refused_refresh_is_one_request_and_a_classified_grant_failure() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/connect/token"))
        .respond_with(ResponseTemplate::new(400).set_body_json(json!({"error": "invalid_grant"})))
        .expect(1)
        .mount(&server)
        .await;
    let client = client_for(&server).await;
    let scoped = Recorder::default();

    let error = client
        .token_manager
        .refresh_token_no_cache_observed(&stored_token(), &scoped)
        .await
        .expect_err("revoked grant");

    assert_eq!(error.kind(), ErrorKind::InvalidGrant);
    assert_eq!(scoped.kinds(), vec![AttemptKind::TokenRefresh]);
    assert!(!format!("{error:?}").contains("refresh"), "{error:?}");
}

#[tokio::test]
async fn a_throttled_api_call_keeps_retry_after_and_never_prints_the_body() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/thing"))
        .respond_with(
            ResponseTemplate::new(429)
                .insert_header("Retry-After", "30")
                .set_body_string(PRIVATE_BODY),
        )
        .mount(&server)
        .await;
    let client = client_for(&server).await;
    let recorder = Arc::new(Recorder::default());
    client.set_attempt_observer(recorder.clone());
    let tenant = Uuid::new_v4();
    let api = ApiClient::new(
        server.uri(),
        tenant,
        reqwest::Client::new(),
        client.token_manager.clone(),
        client.rate_limiter(),
    )
    .with_token_override(Arc::new(stored_token()));

    let error = api
        .send_request::<serde_json::Value, ()>(Method::GET, "/thing", None, None)
        .await
        .expect_err("throttled");

    assert_eq!(error.kind(), ErrorKind::Throttled);
    assert_eq!(error.retry_after(), Some(Duration::from_secs(30)));
    for rendered in [error.to_string(), format!("{error:?}")] {
        assert!(!rendered.contains("Acme"), "{rendered}");
        assert!(!rendered.contains("12,000"), "{rendered}");
    }
    assert_eq!(error.response_body(), Some(PRIVATE_BODY));
    assert_eq!(recorder.kinds(), vec![AttemptKind::Api]);
}

#[tokio::test]
async fn retry_after_may_be_an_http_date() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/thing"))
        .respond_with(
            ResponseTemplate::new(429).insert_header("Retry-After", "Wed, 09 Sep 2020 07:00:00 GMT"),
        )
        .mount(&server)
        .await;
    let client = client_for(&server).await;
    let api = ApiClient::new(
        server.uri(),
        Uuid::new_v4(),
        reqwest::Client::new(),
        client.token_manager.clone(),
        client.rate_limiter(),
    )
    .with_token_override(Arc::new(stored_token()));

    let error = api
        .send_request::<serde_json::Value, ()>(Method::GET, "/thing", None, None)
        .await
        .expect_err("throttled");

    // A date in the past is a wait of nothing, not a missing header.
    assert_eq!(error.retry_after(), Some(Duration::ZERO));
}

#[tokio::test]
async fn a_malformed_success_is_a_decode_error_that_reports_position_only() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/thing"))
        .respond_with(ResponseTemplate::new(200).set_body_string(r#"{"Total": "Acme Ltd"}"#))
        .mount(&server)
        .await;
    let client = client_for(&server).await;
    let api = ApiClient::new(
        server.uri(),
        Uuid::new_v4(),
        reqwest::Client::new(),
        client.token_manager.clone(),
        client.rate_limiter(),
    )
    .with_token_override(Arc::new(stored_token()));

    let error = api
        .send_request::<f64, ()>(Method::GET, "/thing", None, None)
        .await
        .expect_err("not a number");

    assert_eq!(error.kind(), ErrorKind::InvalidResponse);
    assert!(!error.to_string().contains("Acme"));
    assert!(!format!("{error:?}").contains("Acme"));
}

#[tokio::test]
async fn delete_connection_targets_one_connection_with_the_callers_token() {
    let server = MockServer::start().await;
    let connection = Uuid::new_v4();
    Mock::given(method("DELETE"))
        .and(path(format!("/connections/{connection}")))
        .and(header("authorization", "Bearer the-token"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;
    let client = client_for(&server).await;
    let recorder = Arc::new(Recorder::default());
    client.set_attempt_observer(recorder.clone());

    client
        .delete_connection("the-token", connection)
        .await
        .expect("deleted");

    assert_eq!(recorder.kinds(), vec![AttemptKind::DeleteConnection]);
}

#[tokio::test]
async fn delete_connection_reports_gone_throttled_and_redirected_answers() {
    let server = MockServer::start().await;
    let gone = Uuid::new_v4();
    let throttled = Uuid::new_v4();
    let moved = Uuid::new_v4();
    Mock::given(method("DELETE"))
        .and(path(format!("/connections/{gone}")))
        .respond_with(ResponseTemplate::new(404))
        .mount(&server)
        .await;
    Mock::given(method("DELETE"))
        .and(path(format!("/connections/{throttled}")))
        .respond_with(ResponseTemplate::new(429).insert_header("Retry-After", "17"))
        .mount(&server)
        .await;
    Mock::given(method("DELETE"))
        .and(path(format!("/connections/{moved}")))
        .respond_with(ResponseTemplate::new(302).insert_header("Location", "http://127.0.0.1:9/x"))
        .mount(&server)
        .await;
    let client = client_for(&server).await;

    let error = client.delete_connection("t", gone).await.expect_err("404");
    assert_eq!(error.kind(), ErrorKind::NotFound);
    let error = client.delete_connection("t", throttled).await.expect_err("429");
    assert_eq!(error.retry_after(), Some(Duration::from_secs(17)));
    let error = client.delete_connection("t", moved).await.expect_err("302");
    assert_eq!(error.status().map(|status| status.as_u16()), Some(302));
}

#[tokio::test]
async fn a_failed_connection_list_keeps_its_shape_and_drops_its_content() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/connections"))
        .respond_with(ResponseTemplate::new(200).set_body_string(r#"[{"id": "Acme Ltd"}]"#))
        .mount(&server)
        .await;
    let client = client_for(&server).await;

    let error = client
        .get_connections_with_access_token("t")
        .await
        .expect_err("malformed");

    assert!(matches!(error, XeroError::Decode(_)));
    assert!(!format!("{error:?}").contains("Acme"));
    assert_eq!(error.response_body(), None);
}
