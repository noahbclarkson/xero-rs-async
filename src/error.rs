//! Contains the custom error types for the Xero API client.
//!
//! Xero responses carry customer data (invoice amounts, contact names), so no error renders a
//! response body through `Display` or `Debug`. A caller that needs the text for its own purposes
//! asks for it explicitly with [`XeroError::response_body`].

use std::fmt;
use std::time::Duration;

use chrono::{DateTime, Utc};
use reqwest::StatusCode;
use serde::Deserialize;
use thiserror::Error;

/// A provider response body, kept reachable but never printed.
///
/// `Debug` shows only the length, so deriving `Debug` on an error that holds one cannot leak it.
#[derive(Clone, Default, PartialEq, Eq)]
pub struct ResponseBody(String);

impl ResponseBody {
    pub(crate) fn new(text: String) -> Self {
        Self(text)
    }

    /// The body text. Treat it as customer data.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// The body length in bytes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Whether the body was empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl fmt::Debug for ResponseBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ResponseBody({} bytes, redacted)", self.0.len())
    }
}

/// Which Xero service refused a request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Service {
    /// An Accounting, Practice Manager or other tenant API.
    Api,
    /// The identity service's token and revocation endpoints.
    Identity,
    /// The `/connections` endpoint.
    Connections,
}

impl fmt::Display for Service {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Api => "API",
            Self::Identity => "identity",
            Self::Connections => "connections",
        })
    }
}

/// A non-success answer from Xero.
#[derive(Debug, Clone)]
pub struct ProviderResponse {
    /// Which service answered.
    pub service: Service,
    /// The HTTP status.
    pub status: StatusCode,
    /// The wait the provider named in `Retry-After`, when it sent one this crate could read.
    pub retry_after: Option<Duration>,
    /// The OAuth 2.0 `error` code from an identity-service answer (`invalid_grant`, ...). Only
    /// short lowercase codes from the RFC 6749 vocabulary are kept.
    pub oauth_error: Option<String>,
    body: ResponseBody,
}

impl ProviderResponse {
    pub(crate) fn new(
        service: Service,
        status: StatusCode,
        retry_after: Option<Duration>,
        body: String,
    ) -> Self {
        let oauth_error = if service == Service::Identity {
            oauth_error_code(&body)
        } else {
            None
        };
        Self {
            service,
            status,
            retry_after,
            oauth_error,
            body: ResponseBody::new(body),
        }
    }

    /// The raw response body. Treat it as customer data.
    #[must_use]
    pub fn body(&self) -> &str {
        self.body.as_str()
    }
}

impl fmt::Display for ProviderResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Xero {} refused the request ({})", self.service, self.status)?;
        if let Some(code) = &self.oauth_error {
            write!(f, ": {code}")?;
        }
        Ok(())
    }
}

/// A response that arrived but did not decode into the expected type.
///
/// Records where decoding failed and how large the response was, not what it said.
#[derive(Debug, Clone)]
pub struct DecodeError {
    format: &'static str,
    detail: String,
    bytes: usize,
    body: ResponseBody,
}

impl DecodeError {
    pub(crate) fn json(error: &serde_json::Error, body: &str) -> Self {
        let mut decode = Self::json_redacted(error, body.len());
        decode.body = ResponseBody::new(body.to_owned());
        decode
    }

    /// For a response that holds credentials: records the failure and the size, keeps no body.
    pub(crate) fn json_redacted(error: &serde_json::Error, bytes: usize) -> Self {
        Self {
            format: "JSON",
            detail: format!(
                "{:?} error at line {} column {}",
                error.classify(),
                error.line(),
                error.column()
            ),
            bytes,
            body: ResponseBody::default(),
        }
    }

    pub(crate) fn xml(body: &str) -> Self {
        Self {
            format: "XML",
            detail: "the document did not match the expected shape".to_owned(),
            bytes: body.len(),
            body: ResponseBody::new(body.to_owned()),
        }
    }

    /// The raw response that failed to decode. Treat it as customer data.
    #[must_use]
    pub fn body(&self) -> &str {
        self.body.as_str()
    }
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "could not decode the {} response ({} bytes): {}",
            self.format,
            self.bytes,
            self.detail
        )
    }
}

impl std::error::Error for DecodeError {}

/// Represents all possible errors that can occur when interacting with the Xero API.
///
/// None of the variants prints a response body; see [`XeroError::response_body`].
#[derive(Error, Debug)]
pub enum XeroError {
    /// An error occurred during the request, originating from the `reqwest` library. The URL is
    /// removed from it, because a query string can carry search terms.
    #[error("HTTP request error: {0}")]
    Request(reqwest::Error),

    /// An error occurred while serializing a request body or reading a value outside a response.
    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    /// A response arrived but could not be decoded.
    #[error("{0}")]
    Decode(DecodeError),

    /// An error the library itself describes, with the status to report it under. The message is
    /// written by this crate, never taken from a response.
    #[error("Xero API error ({status}): {message}")]
    Api {
        status: StatusCode,
        message: String,
    },

    /// Xero answered with a non-success status.
    #[error("{0}")]
    Provider(ProviderResponse),

    /// A local authentication problem: no token yet, or the wrong flow for this manager.
    #[error("Authentication error: {0}")]
    Auth(String),

    /// The local rate limiter refused to send the request.
    #[error("Rate limiter error: {0}")]
    RateLimiter(String),

    /// An I/O error occurred, typically when interacting with the cache file.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

impl From<reqwest::Error> for XeroError {
    fn from(error: reqwest::Error) -> Self {
        Self::Request(error.without_url())
    }
}

/// A coarse classification of a failure, for callers that decide what to do next.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorKind {
    /// Xero answered 429. See [`XeroError::retry_after`].
    Throttled,
    /// The local rate limiter will send nothing more for this tenant until its day rolls over.
    DailyLimit,
    /// The identity service said the refresh token or code is no longer valid.
    InvalidGrant,
    /// 401: the access token is missing, expired or revoked.
    Unauthorized,
    /// 403.
    Forbidden,
    /// 404.
    NotFound,
    /// 400 or 422.
    InvalidRequest,
    /// 5xx.
    ServerError,
    /// Any other refusal: another status, or a local authentication problem.
    Rejected,
    /// The request did not complete: connection, TLS or timeout failure.
    Transport,
    /// A response arrived that could not be decoded, or a local read or write failed.
    InvalidResponse,
}

impl XeroError {
    /// Classifies this error.
    #[must_use]
    pub fn kind(&self) -> ErrorKind {
        match self {
            Self::Provider(response) => {
                if response.oauth_error.as_deref() == Some("invalid_grant") {
                    ErrorKind::InvalidGrant
                } else {
                    kind_of_status(response.status)
                }
            }
            Self::Api { status, .. } => kind_of_status(*status),
            Self::Request(error) => error
                .status()
                .map_or(ErrorKind::Transport, kind_of_status),
            Self::RateLimiter(_) => ErrorKind::DailyLimit,
            Self::Auth(_) => ErrorKind::Rejected,
            Self::Serde(_) | Self::Decode(_) | Self::Io(_) => ErrorKind::InvalidResponse,
        }
    }

    /// The HTTP status behind this error, when a response produced it.
    #[must_use]
    pub fn status(&self) -> Option<StatusCode> {
        match self {
            Self::Provider(response) => Some(response.status),
            Self::Api { status, .. } => Some(*status),
            Self::Request(error) => error.status(),
            _ => None,
        }
    }

    /// The wait Xero asked for in `Retry-After`, as seconds or an HTTP date.
    #[must_use]
    pub fn retry_after(&self) -> Option<Duration> {
        match self {
            Self::Provider(response) => response.retry_after,
            _ => None,
        }
    }

    /// The raw response body behind this error, when there is one.
    ///
    /// This is the only way to reach it: it is never part of `Display` or `Debug`. It is customer
    /// data and must not be logged.
    #[must_use]
    pub fn response_body(&self) -> Option<&str> {
        match self {
            Self::Provider(response) => Some(response.body()),
            Self::Decode(error) => Some(error.body()).filter(|body| !body.is_empty()),
            _ => None,
        }
    }
}

fn kind_of_status(status: StatusCode) -> ErrorKind {
    match status.as_u16() {
        429 => ErrorKind::Throttled,
        401 => ErrorKind::Unauthorized,
        403 => ErrorKind::Forbidden,
        404 => ErrorKind::NotFound,
        400 | 422 => ErrorKind::InvalidRequest,
        500..=599 => ErrorKind::ServerError,
        _ => ErrorKind::Rejected,
    }
}

/// Reads a `Retry-After` header value: whole seconds, or an HTTP date measured from `now`.
#[must_use]
pub(crate) fn parse_retry_after(value: &str, now: DateTime<Utc>) -> Option<Duration> {
    let value = value.trim();
    if let Ok(seconds) = value.parse::<u64>() {
        return Some(Duration::from_secs(seconds));
    }
    let date = DateTime::parse_from_rfc2822(value).ok()?;
    let wait = date.with_timezone(&Utc) - now;
    Some(wait.to_std().unwrap_or(Duration::ZERO))
}

/// Reads the `Retry-After` header of a response.
pub(crate) fn retry_after_of(headers: &reqwest::header::HeaderMap) -> Option<Duration> {
    let value = headers.get(reqwest::header::RETRY_AFTER)?.to_str().ok()?;
    parse_retry_after(value, Utc::now())
}

/// Turns a non-success response into an error, reading the body for [`XeroError::response_body`].
pub(crate) async fn provider_error(service: Service, response: reqwest::Response) -> XeroError {
    let status = response.status();
    let retry_after = retry_after_of(response.headers());
    let mut response = response;
    let mut bytes = Vec::new();
    // A refusal is short. The cap keeps a misbehaving server from filling memory through an error.
    while bytes.len() < MAX_ERROR_BODY_BYTES {
        match response.chunk().await {
            Ok(Some(chunk)) => bytes.extend_from_slice(&chunk),
            Ok(None) => break,
            Err(error) => return error.into(),
        }
    }
    bytes.truncate(MAX_ERROR_BODY_BYTES);
    let body = String::from_utf8_lossy(&bytes).into_owned();
    XeroError::Provider(ProviderResponse::new(service, status, retry_after, body))
}

/// The most of an error response that is kept.
const MAX_ERROR_BODY_BYTES: usize = 64 * 1024;

/// The `error` member of an OAuth 2.0 error response, when it is a plain RFC 6749 code.
fn oauth_error_code(body: &str) -> Option<String> {
    #[derive(Deserialize)]
    struct OauthError {
        error: String,
    }
    let parsed: OauthError = serde_json::from_str(body.trim()).ok()?;
    let plain = !parsed.error.is_empty()
        && parsed.error.len() <= 64
        && parsed
            .error
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte == b'_');
    plain.then_some(parsed.error)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn provider(service: Service, status: u16, body: &str) -> XeroError {
        XeroError::Provider(ProviderResponse::new(
            service,
            StatusCode::from_u16(status).expect("valid status"),
            None,
            body.to_owned(),
        ))
    }

    #[test]
    fn display_and_debug_never_carry_the_response_body() {
        let error = provider(Service::Api, 400, "Invoice INV-001 for Acme Ltd is $12,000");
        for rendered in [error.to_string(), format!("{error:?}")] {
            assert!(!rendered.contains("Acme"), "{rendered}");
            assert!(!rendered.contains("12,000"), "{rendered}");
        }
        assert_eq!(
            error.response_body(),
            Some("Invoice INV-001 for Acme Ltd is $12,000")
        );
    }

    #[test]
    fn a_decode_error_reports_position_not_content() {
        let body = r#"{"Total": "Acme Ltd"}"#;
        let source = serde_json::from_str::<f64>(body).expect_err("not a number");
        let error = XeroError::Decode(DecodeError::json(&source, body));
        for rendered in [error.to_string(), format!("{error:?}")] {
            assert!(!rendered.contains("Acme"), "{rendered}");
        }
        assert_eq!(error.kind(), ErrorKind::InvalidResponse);
        assert_eq!(error.response_body(), Some(body));
    }

    #[test]
    fn statuses_classify() {
        for (status, kind) in [
            (429, ErrorKind::Throttled),
            (401, ErrorKind::Unauthorized),
            (403, ErrorKind::Forbidden),
            (404, ErrorKind::NotFound),
            (400, ErrorKind::InvalidRequest),
            (422, ErrorKind::InvalidRequest),
            (503, ErrorKind::ServerError),
            (418, ErrorKind::Rejected),
        ] {
            assert_eq!(provider(Service::Api, status, "").kind(), kind, "{status}");
        }
        assert_eq!(
            XeroError::RateLimiter("spent".into()).kind(),
            ErrorKind::DailyLimit
        );
        assert_eq!(XeroError::Auth("none".into()).kind(), ErrorKind::Rejected);
    }

    #[test]
    fn an_invalid_grant_is_recognised_from_the_identity_body_only() {
        let grant = provider(Service::Identity, 400, r#"{"error":"invalid_grant"}"#);
        assert_eq!(grant.kind(), ErrorKind::InvalidGrant);
        assert!(grant.to_string().contains("invalid_grant"));
        let other = provider(Service::Identity, 400, r#"{"error":"invalid_client"}"#);
        assert_eq!(other.kind(), ErrorKind::InvalidRequest);
        // A tenant API body that happens to say the same thing is not a grant failure.
        let api = provider(Service::Api, 400, r#"{"error":"invalid_grant"}"#);
        assert_eq!(api.kind(), ErrorKind::InvalidRequest);
        // Free text is never echoed as a code.
        let free = provider(Service::Identity, 400, r#"{"error":"Acme Ltd owes $5"}"#);
        assert!(!free.to_string().contains("Acme"));
    }

    #[test]
    fn retry_after_reads_seconds_and_http_dates() {
        let now = DateTime::parse_from_rfc3339("2026-09-09T06:59:00Z")
            .expect("fixture")
            .with_timezone(&Utc);
        assert_eq!(parse_retry_after(" 30 ", now), Some(Duration::from_secs(30)));
        assert_eq!(
            parse_retry_after("Wed, 09 Sep 2026 07:00:00 GMT", now),
            Some(Duration::from_secs(60))
        );
        assert_eq!(
            parse_retry_after("Wed, 09 Sep 2026 06:00:00 GMT", now),
            Some(Duration::ZERO)
        );
        assert_eq!(parse_retry_after("soon", now), None);
    }
}
