//! Contains the custom error types for the Xero API client.

use thiserror::Error;

/// Represents all possible errors that can occur when interacting with the Xero API.
#[derive(Error, Debug)]
pub enum XeroError {
    /// An error occurred during the request, originating from the `reqwest` library.
    #[error("HTTP request error: {0}")]
    Request(#[from] reqwest::Error),

    /// An error occurred while (de)serializing JSON data.
    #[error("Serialization/Deserialization error: {0}")]
    Serde(#[from] serde_json::Error),

    /// The Xero API returned a non-success status code with an error message.
    #[error("Xero API error ({status}): {message}")]
    Api {
        status: reqwest::StatusCode,
        message: String,
    },

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
