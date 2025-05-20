use crate::common::*;
/// Storage areas.
pub enum StorageArea {
    Session,
    Local,
    Sync,
    Managed,
}
pub type ExtensionsLoadUnpacked = ();
pub type ExtensionsUninstall = ();
pub type ExtensionsGetStorageItems = ();
pub type ExtensionsRemoveStorageItems = ();
pub type ExtensionsClearStorageItems = ();
pub type ExtensionsSetStorageItems = ();
