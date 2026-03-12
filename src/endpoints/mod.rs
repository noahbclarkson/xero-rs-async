//! Contains the API handles for different Xero services.

#[cfg(feature = "accounting")]
pub mod accounting;
#[cfg(feature = "assets")]
pub mod assets;
#[cfg(feature = "files")]
pub mod files;
#[cfg(any(feature = "accounting", feature = "assets", feature = "files"))]
pub mod tenanted;
