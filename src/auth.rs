//! Handles OAuth 2.0 authentication flow and token management.
//!
//! Two flows are supported, selected at construction time:
//!
//! * **Authorization code** — uses a client secret and Basic auth on the token
//!   endpoint. Construct with [`TokenManager::new`].
//! * **PKCE (Proof Key for Code Exchange)** — no client secret; the caller
//!   generates a verifier/challenge pair, sends the challenge with the
//!   authorize redirect, and presents the verifier when exchanging the code.
//!   Construct with [`TokenManager::new_pkce`].

use crate::error::XeroError;
use base64::Engine;
use log::{debug, info, trace, warn};
use rand::RngExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use tokio::sync::Mutex;

const AUTHORIZE_URL: &str = "https://login.xero.com/identity/connect/authorize";
const TOKEN_URL: &str = "https://identity.xero.com/connect/token";
const REVOCATION_URL: &str = "https://identity.xero.com/connect/revocation";

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TokenSet {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub id_token: Option<String>,
    pub expires_in: u64,
    pub token_type: String,
    #[serde(default = "chrono::Utc::now")]
    pub obtained_at: chrono::DateTime<chrono::Utc>,
}

impl TokenSet {
    /// Checks if the access token is expired or will expire within the next 60 seconds.
    #[must_use]
    pub fn is_expired(&self) -> bool {
        let now = chrono::Utc::now();
        (self.obtained_at + chrono::Duration::seconds(self.expires_in as i64))
            < (now + chrono::Duration::seconds(60))
    }
}

/// Selects how the [`TokenManager`] authenticates against the token endpoint.
#[derive(Debug, Clone)]
enum AuthMode {
    /// Standard authorization code flow with a client secret.
    Code { client_secret: String },
    /// PKCE flow: no client secret, `client_id` carried in the request body.
    Pkce,
}

/// A PKCE code verifier paired with its derived `S256` challenge.
///
/// The caller is responsible for storing the `verifier` between the authorize
/// redirect and the token exchange (typically alongside the `state` value in a
/// signed cookie or session store). The library does not retain it.
#[derive(Debug, Clone)]
pub struct PkceCodes {
    /// Random 43–128 char string in `[A-Z a-z 0-9 -._~]`. Keep this secret.
    pub verifier: String,
    /// `BASE64URL-NOPAD(SHA256(verifier))` — safe to send in the authorize URL.
    pub challenge: String,
}

/// Manages OAuth 2.0 tokens, including fetching, caching, and refreshing.
#[derive(Debug)]
pub struct TokenManager {
    http_client: Client,
    client_id: String,
    redirect_uri: String,
    auth_mode: AuthMode,
    cached_token: Arc<Mutex<Option<TokenSet>>>,
}

impl TokenManager {
    /// Creates a new `TokenManager` for the standard authorization code flow.
    #[must_use]
    pub fn new(
        http_client: Client,
        client_id: String,
        client_secret: String,
        redirect_uri: String,
    ) -> Self {
        Self {
            http_client,
            client_id,
            redirect_uri,
            auth_mode: AuthMode::Code { client_secret },
            cached_token: Arc::new(Mutex::new(None)),
        }
    }

    /// Creates a new `TokenManager` for the PKCE flow.
    ///
    /// PKCE apps have no client secret. Pair this constructor with
    /// [`Self::generate_pkce`], [`Self::get_authorization_url_pkce`], and
    /// [`Self::exchange_code_pkce`].
    #[must_use]
    pub fn new_pkce(http_client: Client, client_id: String, redirect_uri: String) -> Self {
        Self {
            http_client,
            client_id,
            redirect_uri,
            auth_mode: AuthMode::Pkce,
            cached_token: Arc::new(Mutex::new(None)),
        }
    }

    /// Returns `true` if this manager is configured for the PKCE flow.
    #[must_use]
    pub fn is_pkce(&self) -> bool {
        matches!(self.auth_mode, AuthMode::Pkce)
    }

    /// Generates a fresh PKCE verifier and its derived `S256` challenge.
    ///
    /// The verifier is 43 characters of base64url(no-pad)-encoded random bytes,
    /// well within the RFC 7636 [43, 128] length window and using only
    /// characters from the allowed unreserved set.
    #[must_use]
    pub fn generate_pkce() -> PkceCodes {
        let bytes: [u8; 32] = rand::rng().random();
        let verifier = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes);
        let challenge = Self::challenge_for(&verifier);
        PkceCodes {
            verifier,
            challenge,
        }
    }

    /// Derives the `S256` code challenge for a given verifier.
    ///
    /// `BASE64URL-NOPAD(SHA256(ASCII(verifier)))`. Useful when the verifier was
    /// generated elsewhere (e.g. a frontend) and you only need to recompute
    /// the challenge.
    #[must_use]
    pub fn challenge_for(verifier: &str) -> String {
        let digest = Sha256::digest(verifier.as_bytes());
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(digest)
    }

    /// Generates the authorization URL for the standard code flow.
    #[must_use]
    pub fn get_authorization_url(&self, scopes: &[&str], state: &str) -> String {
        let scope_str = scopes.join(" ");
        use url::Url;

        let mut url = Url::parse(AUTHORIZE_URL).unwrap();
        url.query_pairs_mut()
            .append_pair("response_type", "code")
            .append_pair("client_id", &self.client_id)
            .append_pair("redirect_uri", &self.redirect_uri)
            .append_pair("scope", &scope_str)
            .append_pair("state", state);

        url.to_string()
    }

    /// Generates the authorization URL for the PKCE flow.
    ///
    /// `code_challenge` should be the `challenge` field of a [`PkceCodes`]
    /// generated via [`Self::generate_pkce`]. The verifier must be retained
    /// by the caller for the subsequent [`Self::exchange_code_pkce`] call.
    #[must_use]
    pub fn get_authorization_url_pkce(
        &self,
        scopes: &[&str],
        state: &str,
        code_challenge: &str,
    ) -> String {
        let scope_str = scopes.join(" ");
        use url::Url;

        let mut url = Url::parse(AUTHORIZE_URL).unwrap();
        url.query_pairs_mut()
            .append_pair("response_type", "code")
            .append_pair("client_id", &self.client_id)
            .append_pair("redirect_uri", &self.redirect_uri)
            .append_pair("scope", &scope_str)
            .append_pair("state", state)
            .append_pair("code_challenge", code_challenge)
            .append_pair("code_challenge_method", "S256");

        url.to_string()
    }

    /// Builds a POST request to the token endpoint with the right auth shape
    /// for this manager's mode. For PKCE, `client_id` is appended to `params`.
    fn token_request<'a>(
        &'a self,
        mut params: Vec<(&'a str, &'a str)>,
    ) -> reqwest::RequestBuilder {
        let req = self.http_client.post(TOKEN_URL);
        match &self.auth_mode {
            AuthMode::Code { client_secret } => req
                .basic_auth(&self.client_id, Some(client_secret))
                .form(&params),
            AuthMode::Pkce => {
                params.push(("client_id", self.client_id.as_str()));
                req.form(&params)
            }
        }
    }

    async fn exchange_code_inner(
        &self,
        code: &str,
        code_verifier: Option<&str>,
        persist_cache: bool,
    ) -> Result<TokenSet, XeroError> {
        debug!("Exchanging authorization code for token set.");
        let mut params = vec![
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", self.redirect_uri.as_str()),
        ];
        if let Some(v) = code_verifier {
            params.push(("code_verifier", v));
        }
        let response = self.token_request(params).send().await?;

        if response.status().is_success() {
            let token_set = response.json::<TokenSet>().await?;
            if persist_cache {
                info!("Successfully exchanged code for token set. Saving to in-memory cache.");
                self.save_token(&token_set).await;
            } else {
                info!("Successfully exchanged code for token set.");
            }
            Ok(token_set)
        } else {
            let status = response.status();
            let message = response.text().await?;
            Err(XeroError::Auth(format!(
                "Failed to exchange code: {status} - {message}"
            )))
        }
    }

    /// Exchanges an authorization code for a token set (code flow).
    pub async fn exchange_code(&self, code: &str) -> Result<TokenSet, XeroError> {
        if matches!(self.auth_mode, AuthMode::Pkce) {
            return Err(XeroError::Auth(
                "exchange_code called on a PKCE TokenManager — use exchange_code_pkce".to_string(),
            ));
        }
        self.exchange_code_inner(code, None, true).await
    }

    /// Exchanges an authorization code without mutating in-memory token cache.
    ///
    /// Useful for multi-tenant/server workflows where tokens are persisted externally
    /// and a shared cache could leak auth context between requests.
    pub async fn exchange_code_no_cache(&self, code: &str) -> Result<TokenSet, XeroError> {
        if matches!(self.auth_mode, AuthMode::Pkce) {
            return Err(XeroError::Auth(
                "exchange_code_no_cache called on a PKCE TokenManager — use exchange_code_pkce_no_cache".to_string(),
            ));
        }
        self.exchange_code_inner(code, None, false).await
    }

    /// Exchanges an authorization code for a token set (PKCE flow).
    ///
    /// `code_verifier` must be the same string whose challenge was sent in the
    /// authorize URL.
    pub async fn exchange_code_pkce(
        &self,
        code: &str,
        code_verifier: &str,
    ) -> Result<TokenSet, XeroError> {
        if !matches!(self.auth_mode, AuthMode::Pkce) {
            return Err(XeroError::Auth(
                "exchange_code_pkce called on a code-flow TokenManager — use exchange_code"
                    .to_string(),
            ));
        }
        self.exchange_code_inner(code, Some(code_verifier), true)
            .await
    }

    /// Exchanges a PKCE authorization code without mutating in-memory token cache.
    pub async fn exchange_code_pkce_no_cache(
        &self,
        code: &str,
        code_verifier: &str,
    ) -> Result<TokenSet, XeroError> {
        if !matches!(self.auth_mode, AuthMode::Pkce) {
            return Err(XeroError::Auth(
                "exchange_code_pkce_no_cache called on a code-flow TokenManager — use exchange_code_no_cache"
                    .to_string(),
            ));
        }
        self.exchange_code_inner(code, Some(code_verifier), false)
            .await
    }

    /// Refreshes an expired access token using a refresh token.
    ///
    /// Works for both the code and PKCE flows. The auth shape is selected from
    /// the manager's configured mode.
    ///
    /// Retries up to 2 times (3 total attempts) with backoff on transient
    /// network errors.  HTTP 4xx responses are **not** retried because they
    /// indicate a permanent token problem (e.g. `invalid_grant`).
    async fn refresh_token_inner(
        &self,
        token_set: &TokenSet,
        persist_cache: bool,
    ) -> Result<TokenSet, XeroError> {
        info!("Attempting to refresh access token.");
        let refresh_token = token_set
            .refresh_token
            .as_ref()
            .ok_or_else(|| XeroError::Auth("No refresh token available".to_string()))?;

        let backoff_ms = [500u64, 1000];
        let max_attempts: usize = 3;
        let mut last_err: Option<XeroError> = None;

        for attempt in 0..max_attempts {
            let params = vec![
                ("grant_type", "refresh_token"),
                ("refresh_token", refresh_token.as_str()),
            ];
            let result = self.token_request(params).send().await;

            match result {
                Ok(response) => {
                    if response.status().is_success() {
                        let new_token_set = response.json::<TokenSet>().await?;
                        if persist_cache {
                            info!("Successfully refreshed token set. Saving to in-memory cache.");
                            self.save_token(&new_token_set).await;
                        } else {
                            info!("Successfully refreshed token set.");
                        }
                        return Ok(new_token_set);
                    }

                    // Non-success HTTP response – do NOT retry 4xx errors as they
                    // indicate a permanent problem (bad token, revoked grant, etc.).
                    let status = response.status();
                    let message = response.text().await?;
                    if status.is_client_error() {
                        return Err(XeroError::Auth(format!(
                            "Failed to refresh token: {status} - {message}"
                        )));
                    }
                    // 5xx or other server-side errors are transient – retry.
                    warn!(
                        "Token refresh attempt {}/{} got server error {status}: {message}",
                        attempt + 1,
                        max_attempts
                    );
                    last_err = Some(XeroError::Auth(format!(
                        "Failed to refresh token: {status} - {message}"
                    )));
                }
                Err(e) => {
                    // Network / connection errors are transient – retry.
                    warn!(
                        "Token refresh attempt {}/{} failed with network error: {e}",
                        attempt + 1,
                        max_attempts
                    );
                    last_err = Some(XeroError::Request(e));
                }
            }

            // Sleep before the next retry (skip sleep after the last attempt).
            if attempt < max_attempts - 1 {
                let delay = backoff_ms[attempt];
                debug!("Retrying token refresh in {delay}ms");
                tokio::time::sleep(std::time::Duration::from_millis(delay)).await;
            }
        }

        Err(last_err.unwrap_or_else(|| {
            XeroError::Auth("Token refresh failed after all retry attempts".to_string())
        }))
    }

    /// Refreshes an expired access token using a refresh token.
    pub async fn refresh_token(&self, token_set: &TokenSet) -> Result<TokenSet, XeroError> {
        self.refresh_token_inner(token_set, true).await
    }

    /// Refreshes a token without mutating in-memory token cache.
    ///
    /// Useful for per-connection token management where the caller persists
    /// refreshed tokens in its own storage.
    pub async fn refresh_token_no_cache(
        &self,
        token_set: &TokenSet,
    ) -> Result<TokenSet, XeroError> {
        self.refresh_token_inner(token_set, false).await
    }

    /// Revokes a refresh token and removes all of the user's connections to this app.
    ///
    /// Works for both the code and PKCE flows. Per the Xero docs, revocation
    /// for PKCE apps still uses Basic auth but with an empty password
    /// (`base64(client_id + ":")`), which `reqwest::basic_auth` produces when
    /// passed `Some("")`.
    pub async fn revoke_token(&self, refresh_token: &str) -> Result<(), XeroError> {
        let req = self.http_client.post(REVOCATION_URL);
        let req = match &self.auth_mode {
            AuthMode::Code { client_secret } => {
                req.basic_auth(&self.client_id, Some(client_secret))
            }
            AuthMode::Pkce => req.basic_auth(&self.client_id, Some("")),
        };
        let response = req.form(&[("token", refresh_token)]).send().await?;

        if response.status().is_success() {
            info!("Successfully revoked refresh token.");
            Ok(())
        } else {
            let status = response.status();
            let message = response.text().await?;
            Err(XeroError::Auth(format!(
                "Failed to revoke token: {status} - {message}"
            )))
        }
    }

    /// Retrieves the current valid access token, refreshing it if necessary.
    pub async fn get_access_token(&self) -> Result<String, XeroError> {
        debug!("Getting access token.");
        let mut token_set = self.load_token().await.ok_or_else(|| {
            XeroError::Auth("Not authenticated. Please authorize first.".to_string())
        })?;
        trace!("Loaded token set from in-memory cache.");

        // Check if token is expired or close to expiring
        if token_set.is_expired() {
            warn!("Access token expired or nearing expiry. Refreshing...");
            token_set = self.refresh_token(&token_set).await?;
        } else {
            debug!("Access token is still valid.");
        }

        Ok(token_set.access_token)
    }

    /// Saves the token set to the in-memory cache.
    async fn save_token(&self, token_set: &TokenSet) {
        trace!("Saving token to in-memory cache");
        let mut cached_token = self.cached_token.lock().await;
        *cached_token = Some(token_set.clone());
        debug!("Token saved successfully to in-memory cache.");
    }

    /// Loads the token set from the in-memory cache.
    async fn load_token(&self) -> Option<TokenSet> {
        trace!("Loading token from in-memory cache");
        let cached_token = self.cached_token.lock().await;
        if let Some(token) = cached_token.as_ref() {
            debug!("Token found in in-memory cache.");
            Some(token.clone())
        } else {
            warn!("No token found in in-memory cache.");
            None
        }
    }

    /// Sets a token directly in the in-memory cache (useful for external token management).
    pub async fn set_token(&self, token_set: &TokenSet) {
        self.save_token(token_set).await;
    }

    /// Gets the current token from in-memory cache without refreshing.
    pub async fn get_cached_token(&self) -> Option<TokenSet> {
        self.load_token().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// RFC 7636 Appendix B test vector.
    #[test]
    fn challenge_matches_rfc7636_vector() {
        let verifier = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk";
        let expected = "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM";
        assert_eq!(TokenManager::challenge_for(verifier), expected);
    }

    #[test]
    fn generate_pkce_produces_valid_codes() {
        let codes = TokenManager::generate_pkce();

        // Verifier length: 32 random bytes → 43 base64url-no-pad chars.
        assert_eq!(codes.verifier.len(), 43);

        // Verifier alphabet: base64url uses [A-Za-z0-9-_], a strict subset of
        // the PKCE-allowed [A-Za-z0-9-._~].
        for c in codes.verifier.chars() {
            assert!(
                c.is_ascii_alphanumeric() || c == '-' || c == '_',
                "verifier contains disallowed char: {c:?}"
            );
        }

        // Challenge must be the deterministic SHA256/base64url of the verifier.
        assert_eq!(codes.challenge, TokenManager::challenge_for(&codes.verifier));
    }

    #[test]
    fn generate_pkce_produces_distinct_codes() {
        let a = TokenManager::generate_pkce();
        let b = TokenManager::generate_pkce();
        assert_ne!(a.verifier, b.verifier);
        assert_ne!(a.challenge, b.challenge);
    }

    #[test]
    fn pkce_authorize_url_contains_challenge_and_method() {
        let tm = TokenManager::new_pkce(
            Client::new(),
            "CLIENT123".to_string(),
            "http://localhost/cb".to_string(),
        );
        let url = tm.get_authorization_url_pkce(
            &["openid", "accounting.transactions"],
            "state-xyz",
            "CHALLENGE_VAL",
        );

        assert!(url.contains("response_type=code"));
        assert!(url.contains("client_id=CLIENT123"));
        assert!(url.contains("code_challenge=CHALLENGE_VAL"));
        assert!(url.contains("code_challenge_method=S256"));
        assert!(url.contains("state=state-xyz"));
    }

    #[test]
    fn code_authorize_url_does_not_contain_challenge() {
        let tm = TokenManager::new(
            Client::new(),
            "CLIENT123".to_string(),
            "SECRET".to_string(),
            "http://localhost/cb".to_string(),
        );
        let url = tm.get_authorization_url(&["openid"], "state-xyz");

        assert!(!url.contains("code_challenge"));
        assert!(!url.contains("code_challenge_method"));
    }

    #[test]
    fn is_pkce_reflects_construction() {
        let pkce = TokenManager::new_pkce(
            Client::new(),
            "id".to_string(),
            "http://localhost/cb".to_string(),
        );
        assert!(pkce.is_pkce());

        let code = TokenManager::new(
            Client::new(),
            "id".to_string(),
            "secret".to_string(),
            "http://localhost/cb".to_string(),
        );
        assert!(!code.is_pkce());
    }

    #[tokio::test]
    async fn exchange_code_rejected_in_pkce_mode() {
        let tm = TokenManager::new_pkce(
            Client::new(),
            "id".to_string(),
            "http://localhost/cb".to_string(),
        );
        let err = tm.exchange_code("anything").await.unwrap_err();
        assert!(matches!(err, XeroError::Auth(msg) if msg.contains("PKCE")));
    }

    #[tokio::test]
    async fn exchange_code_pkce_rejected_in_code_mode() {
        let tm = TokenManager::new(
            Client::new(),
            "id".to_string(),
            "secret".to_string(),
            "http://localhost/cb".to_string(),
        );
        let err = tm.exchange_code_pkce("code", "verifier").await.unwrap_err();
        assert!(matches!(err, XeroError::Auth(msg) if msg.contains("code-flow")));
    }
}
