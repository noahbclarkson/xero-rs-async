//! API modules for Xero services.

#[cfg(feature = "accounting")]
pub mod accounting;
#[cfg(feature = "bank-feeds")]
pub mod bank_feeds;
#[cfg(feature = "payroll-au")]
pub mod payroll_au;
#[cfg(feature = "payroll-nz")]
pub mod payroll_nz;
#[cfg(feature = "payroll-uk")]
pub mod payroll_uk;
#[cfg(feature = "practice-manager")]
pub mod practice_manager;
#[cfg(feature = "projects")]
pub mod projects;
