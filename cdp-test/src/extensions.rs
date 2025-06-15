use crate::common::*;
/// Storage areas.
pub enum StorageArea {
    Session,
    Local,
    Sync,
    Managed,
}
/** Installs an unpacked extension from the filesystem similar to
--load-extension CLI flags. Returns extension ID once the extension
has been installed. Available if the client is connected using the
--remote-debugging-pipe flag and the --enable-unsafe-extension-debugging
flag is set.*/
pub struct ExtensionsLoadUnpackedParams {
    pub path: String,
}
/** Installs an unpacked extension from the filesystem similar to
--load-extension CLI flags. Returns extension ID once the extension
has been installed. Available if the client is connected using the
--remote-debugging-pipe flag and the --enable-unsafe-extension-debugging
flag is set.*/
pub struct ExtensionsLoadUnpackedParams {
    pub id: String,
}
/** Uninstalls an unpacked extension (others not supported) from the profile.
Available if the client is connected using the --remote-debugging-pipe flag
and the --enable-unsafe-extension-debugging.*/
pub struct ExtensionsUninstallParams {
    pub id: String,
}
/** Uninstalls an unpacked extension (others not supported) from the profile.
Available if the client is connected using the --remote-debugging-pipe flag
and the --enable-unsafe-extension-debugging.*/
pub type ExtensionsUninstallReturns = ();
/** Gets data from extension storage in the given `storageArea`. If `keys` is
specified, these are used to filter the result.*/
pub struct ExtensionsGetStorageItemsParams {
    pub id: String,
    pub storage_area: Box<StorageArea>,
    pub keys: Vec<String>,
}
/** Gets data from extension storage in the given `storageArea`. If `keys` is
specified, these are used to filter the result.*/
pub struct ExtensionsGetStorageItemsParams {
    pub data: serde_json::Map<String, serde_json::Value>,
}
/// Removes `keys` from extension storage in the given `storageArea`.
pub struct ExtensionsRemoveStorageItemsParams {
    pub id: String,
    pub storage_area: Box<StorageArea>,
    pub keys: Vec<String>,
}
/// Removes `keys` from extension storage in the given `storageArea`.
pub type ExtensionsRemoveStorageItemsReturns = ();
/// Clears extension storage in the given `storageArea`.
pub struct ExtensionsClearStorageItemsParams {
    pub id: String,
    pub storage_area: Box<StorageArea>,
}
/// Clears extension storage in the given `storageArea`.
pub type ExtensionsClearStorageItemsReturns = ();
/** Sets `values` in extension storage in the given `storageArea`. The provided `values`
will be merged with existing values in the storage area.*/
pub struct ExtensionsSetStorageItemsParams {
    pub id: String,
    pub storage_area: Box<StorageArea>,
    pub values: serde_json::Map<String, serde_json::Value>,
}
/** Sets `values` in extension storage in the given `storageArea`. The provided `values`
will be merged with existing values in the storage area.*/
pub type ExtensionsSetStorageItemsReturns = ();
