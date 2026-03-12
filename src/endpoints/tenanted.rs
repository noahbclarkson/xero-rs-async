//! Backwards-compatible aliases for tenant-bound API handles.

#[cfg(feature = "accounting")]
use crate::api::accounting::AccountingApi;
#[cfg(feature = "assets")]
use crate::endpoints::assets::AssetsApi;
#[cfg(feature = "files")]
use crate::endpoints::files::FilesApi;

#[cfg(feature = "accounting")]
#[deprecated(note = "Use XeroClient::tenant(tenant_id).accounting() instead.")]
pub type TenantedAccountingApi = AccountingApi;

#[cfg(feature = "assets")]
#[deprecated(note = "Use XeroClient::tenant(tenant_id).assets() instead.")]
pub type TenantedAssetsApi = AssetsApi;

#[cfg(feature = "files")]
#[deprecated(note = "Use XeroClient::tenant(tenant_id).files() instead.")]
pub type TenantedFilesApi = FilesApi;
