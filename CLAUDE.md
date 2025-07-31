# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Common Development Commands

### Building and Testing
- `cargo build` - Build the project
- `cargo test` - Run all tests (requires environment variables)
- `cargo test -- --nocapture` - Run tests with output capture disabled
- `cargo fmt` - Format code according to Rust standards
- `cargo fmt -- --check` - Check if code is properly formatted
- `cargo clippy` - Run the Clippy linter
- `cargo clippy -- -D warnings` - Run Clippy treating warnings as errors

### Authentication Binary
- `cargo run --bin auth` - Run the OAuth 2.0 authentication utility to obtain initial tokens

### Test Environment Setup
Tests require environment variables to be set (usually via a `.env` file):
- `XERO_CLIENT_ID` - Xero app client ID
- `XERO_CLIENT_SECRET` - Xero app client secret
- `XERO_REDIRECT_URI` - OAuth redirect URI
- `XERO_TENANT_ID` - Target tenant/organization ID
- `TOKEN_CACHE_PATH` - Path for token cache file (e.g., "xero_token.json")
- `RATE_LIMIT_CACHE_PATH` - Path for rate limiter cache (e.g., "xero_rate_limit_history.json")

## Code Architecture

### Core Components

**XeroClient** (`src/client.rs`):
- Main entry point for all API interactions
- Manages HTTP client, token manager, and rate limiter
- Provides both generic API handles and tenant-specific convenience handles
- Key methods: `accounting()`, `assets()`, `files()`, `accounting_for_tenant(tenant_id)`

**TokenManager** (`src/auth.rs`):
- Handles OAuth 2.0 flow including authorization URLs, code exchange, and token refresh
- Automatically manages token expiration and refresh
- Persists tokens to cache file for reuse across sessions
- Key methods: `get_authorization_url()`, `exchange_code()`, `get_access_token()`

**RateLimiter** (`src/rate_limiter.rs`):
- Enforces Xero API rate limits: 5 concurrent, 60/minute, 5000/day per tenant
- Thread-safe, tenant-aware rate limiting with persistent state
- Uses semaphores and queues to manage request timing
- Automatically persists state to cache file

**API Endpoints** (`src/endpoints/`):
- `accounting.rs` - Accounting API endpoints (invoices, contacts, accounts, etc.)
- `assets.rs` - Fixed Assets API endpoints
- `files.rs` - Files API endpoints
- `tenanted.rs` - Convenience wrappers that bind API handles to specific tenants

**Models** (`src/models/`):
- Comprehensive typed models for all Xero API resources
- Organized by API: `accounting/`, `assets/`, `files/`
- Uses serde for JSON serialization/deserialization
- Common types in `common.rs` files

### Design Patterns

**Tenant-Scoped API Handles**:
```rust
// Generic handle (requires tenant_id on each call)
let accounting = client.accounting();
let accounts = accounting.get_accounts(tenant_id, ...).await?;

// Tenant-specific handle (tenant_id bound once)
let accounting = client.accounting_for_tenant(tenant_id);
let accounts = accounting.get_accounts(..).await?;
```

**Rate Limiting Integration**:
All API calls automatically acquire permits from the rate limiter before execution. The rate limiter is shared across all API handles for a client instance.

**Token Management**:
Access tokens are automatically refreshed when expired. The TokenManager handles this transparently during API calls.

### Testing Strategy

Tests are integration tests that run against Xero's Demo Company:
- `tests/common.rs` - Shared test utilities and client setup
- `tests/accounting_get.rs` - Accounting API GET endpoint tests
- `tests/assets_get.rs` - Assets API tests
- `tests/files_get.rs` - Files API tests

Tests use a shared RateLimiter instance to avoid rate limit conflicts and require valid Xero API credentials.

### Key Dependencies

- `reqwest` - HTTP client for API calls
- `tokio` - Async runtime and utilities
- `serde` - JSON serialization/deserialization
- `chrono` - Date/time handling for tokens and rate limiting
- `uuid` - Tenant and resource ID handling
- `dashmap` - Concurrent HashMap for rate limiter state
- `tiny_http` - Temporary server for OAuth callback handling