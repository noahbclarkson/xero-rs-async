//! Shared HTTP client utilities for Xero APIs.

use crate::auth::{TokenManager, TokenSet};
use crate::error::{redacted_json_decode_error, redacted_response_metadata, XeroError};
use crate::rate_limiter::RateLimiter;
use log::{debug, error, trace};
use reqwest::{multipart::Form, Client, Method, RequestBuilder};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub(crate) struct ApiClient {
    base_url: String,
    tenant_id: Uuid,
    http_client: Client,
    token_manager: Arc<TokenManager>,
    rate_limiter: Arc<RateLimiter>,
    token_override: Option<Arc<TokenSet>>,
}

impl fmt::Debug for ApiClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ApiClient")
            .field("base_url", &self.base_url)
            .field("tenant_id", &self.tenant_id)
            .field("token_manager", &self.token_manager)
            .field("rate_limiter", &self.rate_limiter)
            .field("token_override", &self.token_override.is_some())
            .finish_non_exhaustive()
    }
}

impl ApiClient {
    pub(crate) fn new(
        base_url: impl Into<String>,
        tenant_id: Uuid,
        http_client: Client,
        token_manager: Arc<TokenManager>,
        rate_limiter: Arc<RateLimiter>,
    ) -> Self {
        let mut base_url = base_url.into();
        while base_url.ends_with('/') {
            base_url.pop();
        }
        Self {
            base_url,
            tenant_id,
            http_client,
            token_manager,
            rate_limiter,
            token_override: None,
        }
    }

    pub(crate) fn with_token_override(mut self, token: Arc<TokenSet>) -> Self {
        self.token_override = Some(token);
        self
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    async fn access_token(&self) -> Result<String, XeroError> {
        if let Some(token) = &self.token_override {
            Ok(token.access_token.clone())
        } else {
            self.token_manager.get_access_token().await
        }
    }

    async fn build_request(
        &self,
        method: Method,
        path: &str,
        accept_json: bool,
    ) -> Result<(String, RequestBuilder), XeroError> {
        let url = self.url(path);
        // Query values and provider object names can contain client information.
        // Log only the bounded path shape; request metadata is correlated from
        // response headers on failure.
        debug!("Sending API request: {} {}", method, path);
        let access_token = self.access_token().await?;

        let mut builder = self
            .http_client
            .request(method, &url)
            .bearer_auth(access_token)
            .header("xero-tenant-id", self.tenant_id.to_string());

        if accept_json {
            builder = builder.header("Accept", "application/json");
        }

        Ok((url, builder))
    }

    async fn send(&self, builder: RequestBuilder) -> Result<reqwest::Response, XeroError> {
        let _permit = self.rate_limiter.acquire_permit(self.tenant_id).await?;
        trace!("Rate limiter permit acquired for tenant {}", self.tenant_id);
        Ok(builder.send().await?)
    }

    async fn send_expect_success(
        &self,
        builder: RequestBuilder,
    ) -> Result<reqwest::Response, XeroError> {
        let response = self.send(builder).await?;
        if response.status().is_success() {
            Ok(response)
        } else {
            let status = response.status();
            let message = redacted_response_metadata(&response);
            Err(XeroError::Api { status, message })
        }
    }

    fn deserialize_json<R>(
        &self,
        response_metadata: &str,
        response_text: &str,
    ) -> Result<R, XeroError>
    where
        R: DeserializeOwned,
    {
        let trimmed = response_text.trim();
        serde_json::from_str::<R>(trimmed).map_err(|error| {
            redacted_json_decode_error(&error, response_text.len(), response_metadata)
        })
    }

    pub(crate) async fn send_request<R, B>(
        &self,
        method: Method,
        path: &str,
        query: Option<&[(String, String)]>,
        body: Option<B>,
    ) -> Result<R, XeroError>
    where
        R: DeserializeOwned,
        B: Serialize,
    {
        let (_url, mut builder) = self.build_request(method, path, true).await?;
        if let Some(q) = &query {
            trace!(
                "Request has {} query parameter(s); values redacted",
                q.len()
            );
            builder = builder.query(q);
        }
        if let Some(b) = body {
            trace!("Request has a JSON body.");
            builder = builder.json(&b);
        }

        let response = self.send_expect_success(builder).await?;
        let response_metadata = redacted_response_metadata(&response);
        let response_text = response.text().await?;
        self.deserialize_json(&response_metadata, &response_text)
    }

    /// Like `send_request` but adds an `If-Modified-Since` header.
    ///
    /// The Xero Journals endpoint supports this header to return only
    /// journals created or modified after the specified UTC datetime.
    pub(crate) async fn send_request_modified_since<R>(
        &self,
        method: Method,
        path: &str,
        query: Option<&[(String, String)]>,
        if_modified_since: &chrono::DateTime<chrono::Utc>,
    ) -> Result<R, XeroError>
    where
        R: DeserializeOwned,
    {
        let (_url, mut builder) = self.build_request(method, path, true).await?;
        if let Some(q) = &query {
            trace!(
                "Request has {} query parameter(s); values redacted",
                q.len()
            );
            builder = builder.query(q);
        }
        let header_value = if_modified_since
            .format("%a, %d %b %Y %H:%M:%S GMT")
            .to_string();
        trace!("If-Modified-Since: {header_value}");
        builder = builder.header("If-Modified-Since", header_value);

        let response = self.send_expect_success(builder).await?;
        let response_metadata = redacted_response_metadata(&response);
        let response_text = response.text().await?;
        self.deserialize_json(&response_metadata, &response_text)
    }

    pub(crate) async fn send_request_text(
        &self,
        method: Method,
        path: &str,
        query: Option<&[(String, String)]>,
    ) -> Result<String, XeroError> {
        let (_url, mut builder) = self.build_request(method, path, true).await?;
        if let Some(q) = &query {
            trace!(
                "Request has {} query parameter(s); values redacted",
                q.len()
            );
            builder = builder.query(q);
        }

        let response = self.send_expect_success(builder).await?;
        Ok(response.text().await?)
    }

    pub(crate) async fn send_request_empty_response<B>(
        &self,
        method: Method,
        path: &str,
        body: Option<B>,
    ) -> Result<(), XeroError>
    where
        B: Serialize,
    {
        let (_url, mut builder) = self.build_request(method, path, false).await?;
        if let Some(b) = body {
            builder = builder.json(&b);
        }

        self.send_expect_success(builder).await?;
        Ok(())
    }

    pub(crate) async fn send_request_raw_body<R, B>(
        &self,
        method: Method,
        path: &str,
        content_type: &str,
        body: B,
    ) -> Result<R, XeroError>
    where
        R: DeserializeOwned,
        B: Into<reqwest::Body>,
    {
        let (_url, builder) = self.build_request(method, path, true).await?;
        let builder = builder.header("Content-Type", content_type).body(body);

        let response = self.send_expect_success(builder).await?;
        let response_metadata = redacted_response_metadata(&response);
        let response_text = response.text().await?;
        self.deserialize_json(&response_metadata, &response_text)
    }

    pub(crate) async fn send_request_bytes(
        &self,
        method: Method,
        path: &str,
        query: Option<&[(String, String)]>,
    ) -> Result<Vec<u8>, XeroError> {
        let (_url, mut builder) = self.build_request(method, path, false).await?;
        if let Some(q) = &query {
            builder = builder.query(q);
        }

        let response = self.send_expect_success(builder).await?;
        Ok(response.bytes().await?.to_vec())
    }

    #[allow(dead_code)]
    pub(crate) async fn send_request_multipart<R>(
        &self,
        method: Method,
        path: &str,
        form: Form,
    ) -> Result<R, XeroError>
    where
        R: DeserializeOwned,
    {
        let (_url, builder) = self.build_request(method, path, true).await?;
        let builder = builder.multipart(form);

        let response = self.send_expect_success(builder).await?;
        let response_metadata = redacted_response_metadata(&response);
        let response_text = response.text().await?;
        self.deserialize_json(&response_metadata, &response_text)
    }

    // ── XML helpers (Practice Manager / XPM) ─────────────────────────

    /// Deserialize an XPM XML response, checking for the `<Status>ERROR</Status>` envelope first.
    #[allow(dead_code)]
    fn deserialize_xml<R>(
        &self,
        response_metadata: &str,
        response_text: &str,
    ) -> Result<R, XeroError>
    where
        R: DeserializeOwned,
    {
        let trimmed = response_text.trim();

        // Check for XPM error envelope
        if trimmed.contains("<Status>ERROR</Status>") {
            return Err(XeroError::Api {
                status: reqwest::StatusCode::BAD_REQUEST,
                message: format!(
                    "XPM response reported an error; response_bytes={}; {response_metadata}",
                    response_text.len()
                ),
            });
        }

        quick_xml::de::from_str::<R>(trimmed).map_err(|e| {
            let classification = xml_error_classification(&e);
            let diagnostic = format!(
                "format=xml; classification={classification}; response_bytes={}; \
                 {response_metadata}",
                response_text.len()
            );
            error!("Failed to deserialize provider response: {diagnostic}");
            XeroError::Xml(quick_xml::DeError::Custom(format!(
                "provider XML did not match the expected response schema; {diagnostic}"
            )))
        })
    }

    /// Send a request expecting an XML response. Parses the XPM response envelope.
    #[allow(dead_code)]
    pub(crate) async fn send_request_xml<R>(
        &self,
        method: Method,
        path: &str,
        query: Option<&[(String, String)]>,
    ) -> Result<R, XeroError>
    where
        R: DeserializeOwned,
    {
        let (_url, mut builder) = self.build_request(method, path, false).await?;
        if let Some(q) = &query {
            trace!(
                "Request has {} query parameter(s); values redacted",
                q.len()
            );
            builder = builder.query(q);
        }

        let response = self.send_expect_success(builder).await?;
        let response_metadata = redacted_response_metadata(&response);
        let response_text = response.text().await?;
        self.deserialize_xml(&response_metadata, &response_text)
    }

    /// Send a request with an XML body, expecting an XML response.
    #[allow(dead_code)]
    pub(crate) async fn send_request_xml_with_body<R>(
        &self,
        method: Method,
        path: &str,
        xml_body: &str,
    ) -> Result<R, XeroError>
    where
        R: DeserializeOwned,
    {
        let (_url, builder) = self.build_request(method, path, false).await?;
        let builder = builder
            .header("Content-Type", "text/xml")
            .body(xml_body.to_string());

        let response = self.send_expect_success(builder).await?;
        let response_metadata = redacted_response_metadata(&response);
        let response_text = response.text().await?;
        self.deserialize_xml(&response_metadata, &response_text)
    }

    /// Send a request with an optional XML body, expecting only a success status (no body to parse).
    #[allow(dead_code)]
    pub(crate) async fn send_request_xml_empty_response(
        &self,
        method: Method,
        path: &str,
        xml_body: Option<&str>,
    ) -> Result<(), XeroError> {
        let (_url, mut builder) = self.build_request(method, path, false).await?;
        if let Some(body) = xml_body {
            builder = builder
                .header("Content-Type", "text/xml")
                .body(body.to_string());
        }

        let response = self.send_expect_success(builder).await?;
        let response_metadata = redacted_response_metadata(&response);
        let response_text = response.text().await?;
        let trimmed = response_text.trim();

        // Still check for XPM error envelope even on "empty" responses
        if trimmed.contains("<Status>ERROR</Status>") {
            return Err(XeroError::Api {
                status: reqwest::StatusCode::BAD_REQUEST,
                message: format!(
                    "XPM response reported an error; response_bytes={}; {response_metadata}",
                    response_text.len()
                ),
            });
        }

        Ok(())
    }
}

fn xml_error_classification(error: &quick_xml::DeError) -> &'static str {
    match error {
        quick_xml::DeError::Custom(_) => "data",
        quick_xml::DeError::InvalidXml(_) => "syntax",
        quick_xml::DeError::KeyNotRead => "deserializer-state",
        quick_xml::DeError::UnexpectedStart(_) => "unexpected-start",
        quick_xml::DeError::UnexpectedEof => "unexpected-eof",
    }
}
