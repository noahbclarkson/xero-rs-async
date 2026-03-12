//! Contains all data models (structs and enums) used for representing Xero API resources.

#[cfg(feature = "accounting")]
pub mod accounting;
#[cfg(feature = "assets")]
pub mod assets;
#[cfg(feature = "bank-feeds")]
pub mod bank_feeds;
#[cfg(feature = "files")]
pub mod files;
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
