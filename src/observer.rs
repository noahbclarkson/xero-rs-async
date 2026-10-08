//! A hook that sees every HTTP request this crate sends.
//!
//! A caller with its own quota ledger cannot learn from the outside how many requests a single
//! call made: a token refresh retries internally, and a rate-limited API call may wait before it
//! sends. The observer is told at the moment each request is dispatched, so a count taken here is
//! the number of requests Xero actually received.

use std::fmt;
use std::sync::{Arc, PoisonError, RwLock};

use uuid::Uuid;

/// What a dispatched request was for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AttemptKind {
    /// A call to a tenant API (Accounting, Practice Manager, ...).
    Api,
    /// `GET /connections`.
    Connections,
    /// `DELETE /connections/{id}`.
    DeleteConnection,
    /// Exchanging an authorization code for tokens.
    TokenExchange,
    /// Refreshing an access token. One refresh call may produce several of these.
    TokenRefresh,
    /// Revoking a refresh token.
    TokenRevocation,
}

/// One HTTP request about to be sent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct Attempt {
    /// What the request is for.
    pub kind: AttemptKind,
    /// The tenant the request is made for, when it is a tenant API call.
    pub tenant_id: Option<Uuid>,
}

/// Receives every request this crate dispatches.
///
/// Called synchronously, immediately before the request goes on the wire and after any local
/// rate-limit wait, so a request the limiter refused is never reported. Keep it cheap and do not
/// block: increment a counter, and settle ledgers once the call returns.
pub trait AttemptObserver: Send + Sync {
    /// Called once per dispatched request, retries included.
    fn on_attempt(&self, attempt: &Attempt);
}

impl<F> AttemptObserver for F
where
    F: Fn(&Attempt) + Send + Sync,
{
    fn on_attempt(&self, attempt: &Attempt) {
        self(attempt);
    }
}

impl Attempt {
    pub(crate) const fn untenanted(kind: AttemptKind) -> Self {
        Self {
            kind,
            tenant_id: None,
        }
    }

    pub(crate) const fn for_tenant(kind: AttemptKind, tenant_id: Uuid) -> Self {
        Self {
            kind,
            tenant_id: Some(tenant_id),
        }
    }
}

/// The observer installed on a token manager, shared by every client built on it.
#[derive(Default)]
pub(crate) struct ObserverSlot(RwLock<Option<Arc<dyn AttemptObserver>>>);

impl ObserverSlot {
    pub(crate) fn set(&self, observer: Arc<dyn AttemptObserver>) {
        *self.0.write().unwrap_or_else(PoisonError::into_inner) = Some(observer);
    }

    /// Reports `attempt` to the installed observer and to the call-scoped one, if any.
    pub(crate) fn notify(&self, attempt: &Attempt, scoped: Option<&dyn AttemptObserver>) {
        let installed = self.0.read().unwrap_or_else(PoisonError::into_inner).clone();
        if let Some(observer) = installed {
            observer.on_attempt(attempt);
        }
        if let Some(observer) = scoped {
            observer.on_attempt(attempt);
        }
    }
}

impl fmt::Debug for ObserverSlot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ObserverSlot")
    }
}
