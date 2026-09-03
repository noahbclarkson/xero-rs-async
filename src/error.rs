//! Contains the custom error types for the Xero API client.

use reqwest::header::{CONTENT_LENGTH, CONTENT_TYPE};
use thiserror::Error;

const MAX_ENDPOINT_CHARS: usize = 256;
const MAX_HEADER_CHARS: usize = 128;

/// Builds bounded response metadata without consuming or exposing the response body.
pub(crate) fn redacted_response_metadata(response: &reqwest::Response) -> String {
    let endpoint = bounded_metadata(response.url().path(), MAX_ENDPOINT_CHARS);
    let content_type = bounded_header(response, CONTENT_TYPE.as_str()).unwrap_or("unknown");
    let content_length = bounded_header(response, CONTENT_LENGTH.as_str()).unwrap_or("unknown");
    let request_id = [
        "xero-correlation-id",
        "xero-request-id",
        "x-request-id",
        "request-id",
    ]
    .into_iter()
    .find_map(|header| bounded_header(response, header))
    .unwrap_or("unavailable");

    format!(
        "endpoint={endpoint}; response_body=redacted; content_type={content_type}; \
         content_length={content_length}; request_id={request_id}"
    )
}

/// Converts a JSON response decode failure into bounded diagnostics without retaining response data.
pub(crate) fn redacted_json_decode_error(
    error: &serde_json::Error,
    response_bytes: usize,
    response_metadata: &str,
) -> XeroError {
    let classification = match error.classify() {
        serde_json::error::Category::Io => "io",
        serde_json::error::Category::Syntax => "syntax",
        serde_json::error::Category::Data => "data",
        serde_json::error::Category::Eof => "unexpected-eof",
    };
    let diagnostic = format!(
        "format=json; classification={classification}; line={}; column={}; \
         response_bytes={response_bytes}; {response_metadata}",
        error.line(),
        error.column()
    );
    log::error!("Failed to deserialize provider response: {diagnostic}");

    XeroError::SerdeWithBody {
        source: serde_json::Error::io(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "provider JSON did not match the expected response schema",
        )),
        body: diagnostic,
    }
}

fn bounded_header<'a>(response: &'a reqwest::Response, name: &str) -> Option<&'a str> {
    response
        .headers()
        .get(name)
        .and_then(|value| value.to_str().ok())
        .filter(|value| {
            value.len() <= MAX_HEADER_CHARS
                && value
                    .chars()
                    .all(|character| character.is_ascii_graphic() || character == ' ')
        })
}

fn bounded_metadata(value: &str, max_chars: usize) -> String {
    value
        .chars()
        .filter(|character| character.is_ascii_graphic())
        .take(max_chars)
        .collect()
}

/// Represents all possible errors that can occur when interacting with the Xero API.
#[derive(Error, Debug)]
pub enum XeroError {
    /// An error occurred during the request, originating from the `reqwest` library.
    #[error("HTTP request error: {0}")]
    Request(#[from] reqwest::Error),

    /// An error occurred while (de)serializing JSON data.
    #[error("Serialization/Deserialization error: {0}")]
    Serde(#[from] serde_json::Error),

    /// An error occurred while deserializing a JSON response.
    ///
    /// The `body` field is retained for API compatibility, but the SDK stores only bounded,
    /// redacted classification and request metadata in it. It never contains the response body.
    #[error("Serialization/Deserialization error: {source}")]
    SerdeWithBody {
        source: serde_json::Error,
        body: String,
    },

    /// The Xero API returned a non-success status code with an error message.
    #[error("Xero API error ({status}): {message}")]
    Api {
        status: reqwest::StatusCode,
        message: String,
    },

    /// An error occurred while deserializing XML data.
    #[error("XML deserialization error: {0}")]
    Xml(#[from] quick_xml::DeError),

    /// An error related to OAuth 2.0 authentication.
    #[error("Authentication error: {0}")]
    Auth(String),

    /// An error occurred within the rate limiter.
    #[error("Rate limiter error: {0}")]
    RateLimiter(String),

    /// An I/O error occurred, typically when interacting with the cache file.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}
