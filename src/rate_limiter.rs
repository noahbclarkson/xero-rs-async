//! Implements a robust, per-tenant rate limiting mechanism to comply with all Xero API limits.

use crate::error::XeroError;
use dashmap::DashMap;
use log::{debug, trace, warn};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs;
// Import the async Mutex
use tokio::sync::{Mutex, Semaphore};
use tokio::time::{sleep, Duration};
use uuid::Uuid;

const CONCURRENT_LIMIT: usize = 5;
const MINUTE_LIMIT: u32 = 60;
const DAILY_LIMIT: u32 = 5000;
const RATE_LIMIT_BUFFER: u32 = 2;

/// Represents the request history for a single tenant.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct TenantRateLimitState {
    requests: VecDeque<i64>, // Timestamps of requests in seconds
}

/// Manages and enforces Xero API rate limits across multiple tenants concurrently.
#[derive(Debug)]
pub struct RateLimiter {
    // Manages the 5 concurrent request limit globally for this client instance.
    concurrent_semaphore: Semaphore,
    // Stores the rate limit state for each tenant.
    tenant_states: DashMap<Uuid, Arc<Mutex<TenantRateLimitState>>>,
    // Path to the file for persisting state.
    cache_path: PathBuf,
    // A Mutex to protect file write operations.
    file_write_lock: Mutex<()>,
}

impl RateLimiter {
    /// Creates a new RateLimiter, loading previous state from a cache file if it exists.
    pub async fn new(cache_path: PathBuf) -> Result<Self, XeroError> {
        debug!("Initializing RateLimiter from cache: {cache_path:?}");
        let tenant_states: DashMap<Uuid, Arc<Mutex<TenantRateLimitState>>> =
            if fs::try_exists(&cache_path).await? {
                let data = fs::read_to_string(&cache_path).await?;
                let loaded_map: HashMap<Uuid, TenantRateLimitState> =
                    serde_json::from_str(&data).unwrap_or_default();
                loaded_map
                    .into_iter()
                    .map(|(k, v)| (k, Arc::new(Mutex::new(v))))
                    .collect()
            } else {
                DashMap::new()
            };

        Ok(Self {
            concurrent_semaphore: Semaphore::new(CONCURRENT_LIMIT),
            tenant_states,
            cache_path,
            // Initialize the new Mutex
            file_write_lock: Mutex::new(()),
        })
    }

    /// Acquires a permit to make a request for a specific tenant, waiting if necessary.
    pub async fn acquire_permit(
        &self,
        tenant_id: Uuid,
    ) -> Result<tokio::sync::SemaphorePermit<'_>, XeroError> {
        trace!("Attempting to acquire permit for tenant {tenant_id}");
        let permit = self.concurrent_semaphore.acquire().await.unwrap();
        debug!("Acquired concurrency permit for tenant {tenant_id}");

        let tenant_state_lock = self.tenant_states.entry(tenant_id).or_default().clone();

        let mut state = tenant_state_lock.lock().await;
        let now = chrono::Utc::now().timestamp();

        state.requests.retain(|&t| now - t < 86400);

        loop {
            let now = chrono::Utc::now().timestamp();
            let minute_ago = now - 60;

            let requests_in_last_minute =
                state.requests.iter().filter(|&&t| t > minute_ago).count() as u32;
            let requests_in_last_day = state.requests.len() as u32;

            if requests_in_last_day >= DAILY_LIMIT - RATE_LIMIT_BUFFER {
                return Err(XeroError::RateLimiter(format!(
                    "Daily rate limit for tenant {tenant_id} is nearly exhausted."
                )));
            }

            if requests_in_last_minute >= MINUTE_LIMIT - RATE_LIMIT_BUFFER {
                if let Some(oldest_in_minute) = state.requests.iter().find(|&&t| t > minute_ago) {
                    let wait_seconds = (oldest_in_minute + 61 - now).max(1);
                    warn!(
                        "Minute rate limit approaching for tenant {tenant_id}. Waiting for {wait_seconds} seconds."
                    );
                    sleep(Duration::from_secs(wait_seconds as u64)).await;
                    continue;
                }
            }
            break;
        }

        trace!("Permit granted. Recording request for tenant {tenant_id}");
        state.requests.push_back(now);

        // Drop the lock on the individual tenant's state before saving the whole map
        // to avoid potential deadlocks if save_state needed to lock it again (it doesn't, but it's good practice).
        drop(state);

        // Persist the entire state map to the cache file, protected by the mutex.
        self.save_state().await?;

        Ok(permit)
    }

    /// Persists the current state of all tenants to the cache file.
    async fn save_state(&self) -> Result<(), XeroError> {
        // Acquire the file write lock before proceeding.
        let _lock = self.file_write_lock.lock().await;
        trace!("Acquired file write lock. Saving rate limiter state to cache.");

        let mut serializable_map = HashMap::new();
        for item in self.tenant_states.iter() {
            let state = item.value().lock().await;
            serializable_map.insert(*item.key(), state.clone());
        }
        let data = serde_json::to_string(&serializable_map)?;
        fs::write(&self.cache_path, data).await?;
        debug!("Rate limiter state saved successfully.");
        Ok(())
    }
}
