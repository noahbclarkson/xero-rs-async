//! Implements a robust, per-tenant rate limiting mechanism to comply with all Xero API limits.

use crate::error::XeroError;
use dashmap::DashMap;
use log::{debug, trace, warn};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::Arc;
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
    // Stores the rate limit state for each tenant in memory.
    tenant_states: DashMap<Uuid, Arc<Mutex<TenantRateLimitState>>>,
}

impl RateLimiter {
    /// Creates a new RateLimiter with in-memory state.
    pub async fn new() -> Result<Self, XeroError> {
        debug!("Initializing RateLimiter with in-memory state");
        let tenant_states: DashMap<Uuid, Arc<Mutex<TenantRateLimitState>>> = DashMap::new();

        Ok(Self {
            concurrent_semaphore: Semaphore::new(CONCURRENT_LIMIT),
            tenant_states,
        })
    }

    /// Acquires a permit to make a request for a specific tenant, waiting if necessary.
    pub async fn acquire_permit(
        &self,
        tenant_id: Uuid,
    ) -> Result<tokio::sync::SemaphorePermit<'_>, XeroError> {
        trace!("Attempting to acquire permit for tenant {}", tenant_id);
        let permit = self.concurrent_semaphore.acquire().await.unwrap();
        debug!("Acquired concurrency permit for tenant {}", tenant_id);

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
                    "Daily rate limit for tenant {} is nearly exhausted.",
                    tenant_id
                )));
            }

            if requests_in_last_minute >= MINUTE_LIMIT - RATE_LIMIT_BUFFER {
                if let Some(oldest_in_minute) = state.requests.iter().find(|&&t| t > minute_ago) {
                    let wait_seconds = (oldest_in_minute + 61 - now).max(1);
                    warn!(
                        "Minute rate limit approaching for tenant {}. Waiting for {} seconds.",
                        tenant_id, wait_seconds
                    );
                    sleep(Duration::from_secs(wait_seconds as u64)).await;
                    continue;
                }
            }
            break;
        }

        trace!("Permit granted. Recording request for tenant {}", tenant_id);
        state.requests.push_back(now);

        // Drop the lock on the individual tenant's state
        drop(state);

        Ok(permit)
    }

}
