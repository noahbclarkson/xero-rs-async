//! Handles OAuth 2.0 authentication flow and token management.

use crate::error::XeroError;
use log::{debug, info, trace, warn};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

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
    pub fn is_expired(&self) -> bool {
        let now = chrono::Utc::now();
        (self.obtained_at + chrono::Duration::seconds(self.expires_in as i64))
            < (now + chrono::Duration::seconds(60))
    }
}

/// Manages OAuth 2.0 tokens, including fetching, caching, and refreshing.
#[derive(Debug)]
pub struct TokenManager {
    http_client: Client,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    cached_token: Arc<Mutex<Option<TokenSet>>>,
}

impl TokenManager {
    /// Creates a new `TokenManager`.
    pub fn new(
        http_client: Client,
        client_id: String,
        client_secret: String,
        redirect_uri: String,
    ) -> Self {
        Self {
            http_client,
            client_id,
            client_secret,
            redirect_uri,
            cached_token: Arc::new(Mutex::new(None)),
        }
    }

    /// Generates the authorization URL to which the user should be redirected.
    pub fn get_authorization_url(&self, scopes: &[&str], state: &str) -> String {
        let scope_str = scopes.join(" ");
        use url::Url;
        
        let mut url = Url::parse("https://login.xero.com/identity/connect/authorize").unwrap();
        url.query_pairs_mut()
            .append_pair("response_type", "code")
            .append_pair("client_id", &self.client_id)
            .append_pair("redirect_uri", &self.redirect_uri)
            .append_pair("scope", &scope_str)
            .append_pair("state", state);
        
        url.to_string()
    }

    /// Exchanges an authorization code for a token set.
    pub async fn exchange_code(&self, code: &str) -> Result<TokenSet, XeroError> {
        debug!("Exchanging authorization code for token set.");
        let params = [
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", &self.redirect_uri),
        ];
        let response = self
            .http_client
            .post("https://identity.xero.com/connect/token")
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .form(&params)
            .send()
            .await?;

        if response.status().is_success() {
            let token_set = response.json::<TokenSet>().await?;
            info!("Successfully exchanged code for token set. Saving to cache.");
            self.save_token(&token_set).await?;
            Ok(token_set)
        } else {
            let status = response.status();
            let message = response.text().await?;
            Err(XeroError::Auth(format!(
                "Failed to exchange code: {} - {}",
                status, message
            )))
        }
    }

    /// Refreshes an expired access token using a refresh token.
    pub async fn refresh_token(&self, token_set: &TokenSet) -> Result<TokenSet, XeroError> {
        info!("Attempting to refresh access token.");
        let refresh_token = token_set
            .refresh_token
            .as_ref()
            .ok_or_else(|| XeroError::Auth("No refresh token available".to_string()))?;

        let params = [
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
        ];
        let response = self
            .http_client
            .post("https://identity.xero.com/connect/token")
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .form(&params)
            .send()
            .await?;

        if response.status().is_success() {
            let new_token_set = response.json::<TokenSet>().await?;
            info!("Successfully refreshed token set. Saving to cache.");
            self.save_token(&new_token_set).await?;
            Ok(new_token_set)
        } else {
            let status = response.status();
            let message = response.text().await?;
            Err(XeroError::Auth(format!(
                "Failed to refresh token: {} - {}",
                status, message
            )))
        }
    }

    /// Retrieves the current valid access token, refreshing it if necessary.
    pub async fn get_access_token(&self) -> Result<String, XeroError> {
        debug!("Getting access token.");
        let mut token_set = self.load_token().await?.ok_or_else(|| {
            XeroError::Auth("Not authenticated. Please authorize first.".to_string())
        })?;
        trace!("Loaded token set from cache.");

        // Check if token is expired or close to expiring
        if token_set.is_expired()
        {
            warn!("Access token expired or nearing expiry. Refreshing...");
            token_set = self.refresh_token(&token_set).await?;
        } else {
            debug!("Access token is still valid.");
        }

        Ok(token_set.access_token)
    }

    /// Saves the token set to the cache file.
    async fn save_token(&self, token_set: &TokenSet) -> Result<(), XeroError> {
        trace!("Saving token to cache at {:?}", self.cache_path);
        let data = serde_json::to_string(token_set)?;
        fs::write(&self.cache_path, data).await?;
        debug!("Token saved successfully.");
        Ok(())
    }

    /// Loads the token set from the cache file.
    async fn load_token(&self) -> Result<Option<TokenSet>, XeroError> {
        trace!("Loading token from cache at {:?}", self.cache_path);
        if fs::try_exists(&self.cache_path).await? {
            let data = fs::read_to_string(&self.cache_path).await?;
            debug!("Token cache file found and read.");
            Ok(serde_json::from_str(&data).ok())
        } else {
            warn!("Token cache file not found.");
            Ok(None)
        }
    }
}