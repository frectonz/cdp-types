use crate::common::*;
/// DOM Storage identifier.
pub struct StorageId {
    pub security_origin: String,
    pub storage_key: Box<StorageSerializedStorageKey>,
    pub is_local_storage: bool,
}
/// DOM Storage item.
pub struct Item(Vec<String>);
pub type DomStorageClear = ();
pub type DomStorageDisable = ();
pub type DomStorageEnable = ();
pub type DomStorageGetDomStorageItems = ();
pub type DomStorageRemoveDomStorageItem = ();
pub type DomStorageSetDomStorageItem = ();
