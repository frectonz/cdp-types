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
pub type ExtensionsLoadUnpacked = ();
/** Uninstalls an unpacked extension (others not supported) from the profile.
Available if the client is connected using the --remote-debugging-pipe flag
and the --enable-unsafe-extension-debugging.*/
pub type ExtensionsUninstall = ();
/** Gets data from extension storage in the given `storageArea`. If `keys` is
specified, these are used to filter the result.*/
pub type ExtensionsGetStorageItems = ();
/// Removes `keys` from extension storage in the given `storageArea`.
pub type ExtensionsRemoveStorageItems = ();
/// Clears extension storage in the given `storageArea`.
pub type ExtensionsClearStorageItems = ();
/** Sets `values` in extension storage in the given `storageArea`. The provided `values`
will be merged with existing values in the storage area.*/
pub type ExtensionsSetStorageItems = ();
